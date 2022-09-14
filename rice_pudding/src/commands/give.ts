import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';

export const give: Command = {
    builder: new SlashCommandBuilder().addUserOption((builder) =>
        builder.setName('person').setRequired(true),
    ),

    async execute(interaction, prisma) {
        const senderId = interaction.user.id;
        const sender = await prisma.user.upsert({
            where: { id: senderId },
            update: {},
            create: { id: senderId },
        });

        if (sender.pudding === 0) {
            return interaction.reply('You have no pudding left to give!');
        }

        const receiverId = interaction.options.getUser('person', true).id;
        const receiver = await prisma.user.upsert({
            where: { id: receiverId },
            update: {},
            create: { id: receiverId },
        });

        await prisma.$transaction([
            prisma.user.update({
                where: { id: senderId },
                data: { pudding: sender.pudding - 1 },
            }),
            prisma.user.update({
                where: { id: receiverId },
                data: { pudding: receiver.pudding + 1 },
            }),
        ]);
    },
};
