import { User } from '@prisma/client';
import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';
import { ordinal } from '../util';

const format = (data: User, index: number) =>
    `• \`${ordinal(index)}\`  |  <@${data.id}>  (${data.pudding})\n`;

export const top: Command = {
    builder: new SlashCommandBuilder().setDescription(
        'See the people with the highest rice puddings',
    ),

    async execute(interaction, prisma) {
        const sender = interaction.user;
        await prisma.user.upsert({
            where: { id: sender.id },
            update: { username: sender.username },
            create: { id: sender.id, username: sender.username },
        });

        const top = await prisma.user.findMany({
            take: 10,
            orderBy: {
                pudding: 'desc',
            },
        });

        if (top.length === 0) {
            return interaction.reply('No one has any rice pudding!');
        }

        let message = '';
        let previous = Infinity;
        let rank = 0;
        for (const data of top) {
            if (data.pudding < previous) {
                previous = data.pudding;
                rank += 1;
            }

            message += format(data, rank);
        }

        interaction.reply(message);
    },
};
