import { PrismaClient } from '@prisma/client';
import { ChatInputCommandInteraction, SlashCommandBuilder } from 'discord.js';

export interface Command {
    builder: Pick<SlashCommandBuilder, 'toJSON' | 'setName'>;

    execute(
        interaction: ChatInputCommandInteraction,
        prisma: PrismaClient,
    ): Promise<unknown>;
}

import { give } from './commands/give';
import { wallet } from './commands/wallet';
import { top } from './commands/top';

export const commands = { give, wallet, top };

for (const [name, command] of Object.entries(commands)) {
    command.builder.setName(name);
}
