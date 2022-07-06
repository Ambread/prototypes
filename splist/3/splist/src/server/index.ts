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
        }),

        output: zMessage,

        async resolve({ input, ctx }) {
            const message = await ctx.prisma.message.create({
                data: {
                    content: input.content,
                    authorId: ctx.requiredUser.id,
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
    })
    .mutation('login', {
        input: z.object({
            name: z.string(),
        }),

        output: z
            .object({
                id: z.string(),
                name: z.string(),
            })
            .nullable(),

        async resolve({ input, ctx }) {
            const user = await ctx.prisma.user.findFirst({
                where: { name: input.name },
            });
            ctx.user = user;
            return user;
        },
    });

export type AppRouter = typeof appRouter;
