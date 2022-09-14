import { REST, Routes } from 'discord.js';
import { config } from '../config';
import { commands } from './command';

const rest = new REST({ version: '10' }).setToken(config.token);

Promise.all(
    config.guildIds.map((guildId) =>
        rest.put(Routes.applicationGuildCommands(config.clientId, guildId), {
            body: Object.values(commands).map((command) =>
                command.builder.toJSON(),
            ),
        }),
    ),
)
    .then(() =>
        console.log(
            `Successfully registered ${
                Object.keys(commands).length
            } commands for ${config.guildIds.length} guilds.`,
        ),
    )
    .catch(console.error);
