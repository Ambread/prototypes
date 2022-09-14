import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';

export const ping: Command = {
    builder: new SlashCommandBuilder().setDescription('Pong!'),

    async execute(interaction) {
        interaction.reply('Pong!');
    },
};
