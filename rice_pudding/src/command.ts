import { ChatInputCommandInteraction, SlashCommandBuilder } from 'discord.js';

export interface Command {
    builder: SlashCommandBuilder;
    execute(interaction: ChatInputCommandInteraction): Promise<unknown>;
}

import { ping } from './commands/ping';

export const commands: Record<string, Command> = { ping };

for (const [name, command] of Object.entries(commands)) {
    command.builder.setName(name);
}
