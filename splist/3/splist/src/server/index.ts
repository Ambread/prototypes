import { z } from 'zod';
import { createRouter } from './createContext';

const zMessage = z.object({
    id: z.string(),
    content: z.string(),
    author: z.object({
        id: z.string(),
        name: z.string(),
    }),
    channel: z.object({
        id: z.string(),
    }),
});

export interface Events {
    'text.send': z.infer<typeof zMessage>;
    'text.clear': { channelId: string };
}

export const appRouter = createRouter()
    .query('channels', {
        output: z.array(
            z.object({
                id: z.string(),
                title: z.string(),
                text: z
                    .object({
                        id: z.string(),
                    })
                    .nullable(),
            }),
        ),

        resolve({ ctx }) {
            return ctx.prisma.channel.findMany({
                include: {
                    text: {
                        select: { id: true },
                    },
                },
            });
        },
    })
    .query('text.messages', {
        input: z.object({
            channelId: z.string(),
        }),

        output: z.array(zMessage),

        resolve({ ctx, input }) {
            return ctx.prisma.message.findMany({
                include: { author: true, channel: true },
                where: { channelId: input.channelId },
            });
        },
    })
    .mutation('text.send', {
        input: z.object({
            content: z.string(),
            channelId: z.string(),
        }),

        output: zMessage,

        async resolve({ input, ctx }) {
            const message = await ctx.prisma.message.create({
                data: {
                    content: input.content,
                    authorId: ctx.requiredUser.id,
                    channelId: input.channelId,
                },
                include: {
                    author: true,
                    channel: true,
                },
            });

            ctx.emitEvent('text.send', message);
            return message;
        },
    })
    .mutation('text.clear', {
        input: z.object({
            channelId: z.string(),
        }),

        async resolve({ ctx, input }) {
            await ctx.prisma.message.deleteMany({
                where: { channelId: input.channelId },
            });
            ctx.emitEvent('text.clear', { channelId: input.channelId });
        },
    })
    .subscription('text.onSend', {
        input: z.object({
            channelId: z.string(),
        }),

        resolve({ ctx, input }) {
            return ctx.useEvent('text.send', (emit, message) => {
                if (message.channel.id !== input.channelId) {
                    return;
                }
                emit.data(message);
            });
        },
    })
    .subscription('text.onClear', {
        input: z.object({
            channelId: z.string(),
        }),

        resolve({ ctx, input }) {
            return ctx.useEvent('text.clear', (emit, { channelId }) => {
                if (channelId !== input.channelId) {
                    return;
                }
                emit.data({ channelId });
            });
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
