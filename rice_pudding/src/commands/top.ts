import { User } from '@prisma/client';
import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';
import { getUserData, ordinal } from '../util';

const format = (data: User, index: number) =>
    `• \`${ordinal(index)}\`  |  <@${data.id}>  (${data.pudding})\n`;

export const top: Command = {
    builder: new SlashCommandBuilder().setDescription(
        'See the people with the highest rice pudding',
    ),

    async execute(interaction, prisma) {
        const sender = interaction.user;
        getUserData(sender);

        const top = await prisma.user.findMany({
            take: 10,
            orderBy: {
                pudding: 'desc',
            },
        });

        if (top.length === 0) {
            return interaction.reply('No one has any rice pudding!');
        }

        let previous = Infinity;
        let rank = 0;

        const message = top.map((data) => {
            if (data.pudding < previous) {
                previous = data.pudding;
                rank += 1;
            }

            return format(data, rank);
        });

        interaction.reply(message.join(''));
    },
};
