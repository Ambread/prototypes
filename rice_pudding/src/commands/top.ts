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

        interaction.reply(top.map(format).join('\n'));
    },
};
