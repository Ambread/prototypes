import { Message } from '@prisma/client';
import { z } from 'zod';
import { createRouter } from './createContext';

export interface Events {
    send: Message;
    clear: null;
}

const zMessage = z.object({
    id: z.string(),
    content: z.string(),
});

export const appRouter = createRouter()
    .query('messages', {
        output: z.array(zMessage),

        resolve({ ctx }) {
            return ctx.prisma.message.findMany();
        },
    })
    .mutation('send', {
        input: z.object({
            content: z.string(),
        }),

        output: zMessage,

        async resolve({ input, ctx }) {
            const message = await ctx.prisma.message.create({
                data: input,
            });
            ctx.events.emit('send', message);
            return message;
        },
    })
    .mutation('clear', {
        async resolve({ ctx }) {
            await ctx.prisma.message.deleteMany();
            ctx.events.emit('clear');
        },
    })
    .subscription('onSend', {
        resolve({ ctx }) {
            return ctx.useEvent('send', (emit, message) => emit.data(message));
        },
    })
    .subscription('onClear', {
        resolve({ ctx }) {
            return ctx.useEvent('clear', (emit) => emit.data(null));
        },
    });

export type AppRouter = typeof appRouter;
