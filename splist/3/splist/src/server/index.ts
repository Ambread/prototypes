import { router } from '@trpc/server';
import { z } from 'zod';

export const appRouter = router().query('hello', {
    input: z
        .object({
            text: z.string().nullish(),
        })
        .nullish(),

    resolve({ input }) {
        return {
            greeting: `Hello ${input?.text ?? 'world'}!`,
        };
    },
});

export type AppRouter = typeof appRouter;
