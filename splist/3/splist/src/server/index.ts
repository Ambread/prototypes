import { PrismaClient } from '@prisma/client';
import { router } from '@trpc/server';
import { z } from 'zod';

const prisma = new PrismaClient();

const zMessage = z.object({
    id: z.string(),
    content: z.string(),
});

export const appRouter = router()
    .query('messages', {
        output: z.array(zMessage),

        resolve() {
            return prisma.message.findMany();
        },
    })
    .mutation('send', {
        input: z.object({
            content: z.string(),
        }),

        output: zMessage,

        resolve({ input }) {
            return prisma.message.create({
                data: input,
            });
        },
    })
    .mutation('clear', {
        async resolve() {
            await prisma.message.deleteMany();
        },
    });

export type AppRouter = typeof appRouter;
