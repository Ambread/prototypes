import { PrismaClient } from '@prisma/client';
import { router } from '@trpc/server';

declare global {
    var __prisma: PrismaClient | undefined;
}

global.__prisma ??= new PrismaClient();
const prisma = global.__prisma;

export interface Context {
    prisma: PrismaClient;
}

export const createRouter = router<Context>;

export const createContext = async (): Promise<Context> => {
    return {
        prisma,
    };
};


