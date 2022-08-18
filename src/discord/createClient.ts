import {GatewayIntentBits, Client} from "discord.js";

/**
 * Create a discord client.
 * It used to be simply to create discord clients but since intents are needed
 * this goes into it's own separate file.
 */
export default function createClient(): Client {
  return new Client({
    intents: [
      GatewayIntentBits.Guilds,
      GatewayIntentBits.GuildVoiceStates,
      GatewayIntentBits.GuildMessages,
      GatewayIntentBits.GuildIntegrations
    ]
  });
}
