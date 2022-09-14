import { User } from '@prisma/client';
import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';
import { ordinal } from '../util';

const format = (data: User, index: number) =>
    `• \`${ordinal(index + 1)}\`  |  <@${data.id}>  (${data.pudding})`;

export const top: Command = {
    builder: new SlashCommandBuilder().setDescription(
        'See the people with the highest rice puddings',
    ),

    async execute(interaction, prisma) {
        const top = await prisma.user.findMany({
            take: 10,
            orderBy: {
                pudding: 'desc',
            },
        });

        if (top.length === 0) {
            return interaction.reply('No one has any rice pudding!');
        }

        interaction.reply(top.map(format).join('\n'));
    },
};
