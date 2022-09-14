import { PrismaClient } from '@prisma/client';
import { ChatInputCommandInteraction, SlashCommandBuilder } from 'discord.js';

export interface Command {
    builder: Pick<SlashCommandBuilder, 'toJSON' | 'setName'>;
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
