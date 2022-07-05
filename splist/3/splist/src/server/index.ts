import { Message } from '@prisma/client';
import { Subscription } from '@trpc/server';
import { EventEmitter } from 'stream';
import { z } from 'zod';
import { createRouter } from './createContext';

const zMessage = z.object({
    id: z.string(),
    content: z.string(),
});

const events = new EventEmitter();

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
            events.emit('send', message);
            return message;
        },
    })
    .mutation('clear', {
        async resolve({ ctx }) {
            await ctx.prisma.message.deleteMany();
        },
    })
    .subscription('onSend', {
        resolve() {
            return new Subscription<Message>((emit) => {
                const handle = (message: Message) => {
                    emit.data(message);
                };

                events.on('send', handle);
                return () => {
                    events.off('send', handle);
                };
            });
        },
    });

export type AppRouter = typeof appRouter;
