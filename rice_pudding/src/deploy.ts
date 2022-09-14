import { REST, Routes } from 'discord.js';
import { config } from '../config';
import { commands } from './command';

if (require.main === module) {
    const rest = new REST({ version: '10' }).setToken(config.token);

    const body = Object.values(commands).map((command) =>
        command.builder.toJSON(),
    );

    const updateGuild = (guildId: string) => {
        const route = Routes.applicationGuildCommands(config.clientId, guildId);
        return rest.put(route, { body });
    };

    const updateGuilds = Promise.all(config.guildIds.map(updateGuild));

    updateGuilds.then(() =>
        console.log(
            `Successfully registered ${
                Object.keys(commands).length
            } commands for ${config.guildIds.length} guilds.`,
        ),
    );
}
