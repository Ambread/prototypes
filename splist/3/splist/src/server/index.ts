import { TRPCError } from '@trpc/server';
import { z } from 'zod';
import { createRouter } from './createContext';

const zMessage = z.object({
    id: z.string(),
    content: z.string(),
    author: z.object({
        id: z.string(),
        name: z.string(),
    }),
});

export interface Events {
    send: z.infer<typeof zMessage>;
    clear: null;
}

export const appRouter = createRouter()
    .query('messages', {
        output: z.array(zMessage),

        resolve({ ctx }) {
            return ctx.prisma.message.findMany({ include: { author: true } });
        },
    })
    .mutation('send', {
        input: z.object({
            content: z.string(),
            name: z.string(),
        }),

        output: zMessage,

        async resolve({ input, ctx }) {
            if (!ctx.user) {
                throw new TRPCError({ code: 'UNAUTHORIZED' });
            }

            const message = await ctx.prisma.message.create({
                data: {
                    content: input.content,
                    authorId: ctx.user.id,
                },
                include: {
                    author: true,
                },
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
