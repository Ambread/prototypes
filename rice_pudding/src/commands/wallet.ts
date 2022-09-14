import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';
import { ordinal } from '../util';

export const wallet: Command = {
    builder: new SlashCommandBuilder()
        .setDescription(
            'View the rice pudding count and ranking of a person or yourself',
        )
        .addUserOption((option) =>
            option
                .setName('person')
                .setDescription('The person to view')
                .setRequired(false),
        ),

    async execute(interaction, prisma) {
        const personOption = interaction.options.getUser('person');

        const person = personOption ?? interaction.user;
        const data = await prisma.user.upsert({
            where: { id: person.id },
            update: { username: person.username },
            create: { id: person.id, username: person.username },
        });

        const rankData = await prisma.user.aggregate({
            _count: true,
            where: {
                pudding: { gt: data.pudding },
            },
        });
        const rank = ordinal(rankData._count + 1);

        if (person.id === interaction.user.id) {
            return interaction.reply(
                `You have ${data.pudding} rice pudding and are in ${rank} place.`,
            );
        }

        interaction.reply(
            `<@${person.id}> has ${data.pudding} rice pudding and is in ${rank} place.`,
        );
    },
};
