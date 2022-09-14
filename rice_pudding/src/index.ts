import { Client, GatewayIntentBits } from 'discord.js';
import { config } from '../config';
import { commands } from './command';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const client = new Client({
    intents: [GatewayIntentBits.Guilds],
    allowedMentions: {
        users: [],
        roles: [],
        parse: [],
    },
});

client.on('ready', () => {
    console.log('Ready!');
});

client.on('interactionCreate', (interaction) => {
    if (!interaction.isChatInputCommand()) return;

    commands[interaction.commandName].execute(interaction, prisma);
});

client.login(config.token);
