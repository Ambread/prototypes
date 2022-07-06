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
    'text.send': z.infer<typeof zMessage>;
    'text.clear': null;
}

export const appRouter = createRouter()
    .query('channels', {
        output: z.array(
            z.object({
                id: z.string(),
                title: z.string(),
            }),
        ),

        resolve({ ctx }) {
            return ctx.prisma.channel.findMany();
        },
    })
    .query('text.messages', {
        output: z.array(zMessage),

        resolve({ ctx }) {
            return ctx.prisma.message.findMany({ include: { author: true } });
        },
    })
    .mutation('text.send', {
        input: z.object({
            content: z.string(),
        }),

        output: zMessage,

        async resolve({ input, ctx }) {
            const message = await ctx.prisma.message.create({
                data: {
                    content: input.content,
                    authorId: ctx.requiredUser.id,
                    channelId: '0',
                },
                include: {
                    author: true,
                },
            });

            ctx.emitEvent('text.send', message);
            return message;
        },
    })
    .mutation('text.clear', {
        async resolve({ ctx }) {
            await ctx.prisma.message.deleteMany();
            ctx.emitEvent('text.clear', null);
        },
    })
    .subscription('text.onSend', {
        resolve({ ctx }) {
            return ctx.useEvent('text.send', (emit, message) =>
                emit.data(message),
            );
        },
    })
    .subscription('text.onClear', {
        resolve({ ctx }) {
            return ctx.useEvent('text.clear', (emit) => emit.data(null));
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
