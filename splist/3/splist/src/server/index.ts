import { router } from '@trpc/server';
import { z } from 'zod';
import { Context, createRouter } from './createContext';

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

        resolve({ input, ctx }) {
            return ctx.prisma.message.create({
                data: input,
            });
        },
    })
    .mutation('clear', {
        async resolve({ ctx }) {
            await ctx.prisma.message.deleteMany();
        },
    });

export type AppRouter = typeof appRouter;
