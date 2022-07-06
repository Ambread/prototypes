import { PrismaClient, User } from '@prisma/client';
import {
    router,
    Subscription,
    SubscriptionEmit,
    TRPCError,
} from '@trpc/server';
import { CreateNextContextOptions } from '@trpc/server/adapters/next';
import { NodeHTTPCreateContextFnOptions } from '@trpc/server/dist/declarations/src/adapters/node-http';
import { IncomingMessage } from 'node:http';
import ws from 'ws';
import { EventEmitter } from 'node:events';
import { Events } from '.';

declare global {
    var __prisma: PrismaClient | undefined;
}

global.__prisma ??= new PrismaClient();
const prisma = global.__prisma;

const events = new EventEmitter();

const useEvent = <E extends keyof Events>(
    event: E,
    listener: (emit: SubscriptionEmit<Events[E]>, data: Events[E]) => unknown,
) =>
    new Subscription<Events[E]>((emit) => {
        const wrapper = (data: Events[E]) => listener(emit, data);
        events.on(event, wrapper);
        return () => events.off(event, wrapper);
    });

const emitEvent = <E extends keyof Events>(event: E, value: Events[E]) =>
    events.emit(event, value);

export interface Context {
    prisma: PrismaClient;
    useEvent: typeof useEvent;
    emitEvent: typeof emitEvent;
    user: User | null;
    requiredUser: User;
}

export const createRouter = () => router<Context>();

type Props =
    | CreateNextContextOptions
    | NodeHTTPCreateContextFnOptions<IncomingMessage, ws>;

export const createContext = async ({ req, res }: Props) => {
    return {
        prisma,
        useEvent,
        emitEvent,
        user: null,
        get requiredUser() {
            if (!this.user) {
                throw new TRPCError({ code: 'UNAUTHORIZED' });
            }
            return this.user;
        },
    };
};
