import { PrismaClient } from '@prisma/client';
import { router } from '@trpc/server';
import { CreateNextContextOptions } from '@trpc/server/adapters/next';
import { NodeHTTPCreateContextFnOptions } from '@trpc/server/dist/declarations/src/adapters/node-http';
import { IncomingMessage } from 'node:http';
import ws from 'ws';

declare global {
    var __prisma: PrismaClient | undefined;
}

global.__prisma ??= new PrismaClient();
const prisma = global.__prisma;

export interface Context {
    prisma: PrismaClient;
}

export const createRouter = () => router<Context>();

type Props =
    | CreateNextContextOptions
    | NodeHTTPCreateContextFnOptions<IncomingMessage, ws>;

export const createContext = async ({ req, res }: Props): Promise<Context> => {
    return {
        prisma,
    };
};
