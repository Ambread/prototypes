import { PrismaClient } from '@prisma/client';
import {
    ChatInputCommandInteraction,
    SharedNameAndDescription,
} from 'discord.js';

export interface Command {
    builder: SharedNameAndDescription;
    execute(
        interaction: ChatInputCommandInteraction,
        prisma: PrismaClient,
    ): Promise<unknown>;
}

import { ping } from './commands/ping';
import { give } from './commands/give';

export const commands: Record<string, Command> = { ping, give };

for (const [name, command] of Object.entries(commands)) {
    command.builder.setName(name);
}
