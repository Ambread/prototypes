import { PrismaClient } from '@prisma/client';
import { router } from '@trpc/server';
import { z } from 'zod';

const prisma = new PrismaClient();

export const appRouter = router()
    .query('hello', {
        input: z
            .object({
                text: z.string().nullish(),
            })
            .nullish(),

        output: z.object({
            greeting: z.string(),
        }),

        resolve({ input }) {
            return {
                greeting: `Hello ${input?.text ?? 'world'}!`,
            };
        },
    })
    .query('messages', {
        output: z.array(
            z.object({
                id: z.string(),
                content: z.string(),
            }),
        ),

        resolve() {
            return prisma.message.findMany();
        },
    });

export type AppRouter = typeof appRouter;
