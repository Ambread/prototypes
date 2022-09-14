import { SlashCommandBuilder } from 'discord.js';
import { Command } from '../command';

export const give: Command = {
    builder: new SlashCommandBuilder().addUserOption((builder) =>
        builder.setName('person').setRequired(true),
    ),

    async execute(interaction, prisma) {
        const sender = interaction.user;
        const senderData = await prisma.user.upsert({
            where: { id: sender.id },
            update: { username: sender.username },
            create: { id: sender.id, username: sender.username },
        });

        if (senderData.pudding === 0) {
            return interaction.reply('You have no pudding left to give!');
        }

        const receiver = interaction.options.getUser('person', true);
        const receiverData = await prisma.user.upsert({
            where: { id: receiver.id },
            update: { username: receiver.username },
            create: { id: receiver.id, username: receiver.username },
        });

        await prisma.$transaction([
            prisma.user.update({
                where: { id: sender.id },
                data: { pudding: senderData.pudding - 1 },
            }),
            prisma.user.update({
                where: { id: receiver.id },
                data: { pudding: receiverData.pudding + 1 },
            }),
        ]);
    },
};
