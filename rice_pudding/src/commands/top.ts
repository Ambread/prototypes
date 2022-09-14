import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';
import { ordinal } from '../util';

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

        interaction.reply(
            top
                .map(
                    (data, index) =>
                        `• \`${ordinal(index + 1)}\`  |  <@${data.id}>  (${
                            data.pudding
                        })`,
                )
                .join('\n'),
        );
    },
};
