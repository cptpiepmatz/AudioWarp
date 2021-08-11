import {Client, Intents} from "discord.js";

/**
 * Create a discord client.
 * It used to be simply to create discord clients but since intents are needed
 * this goes into it's own separate file.
 */
export default function createClient(): Client {
  return new Client({
    intents: [
      Intents.FLAGS.GUILDS,
      Intents.FLAGS.GUILD_VOICE_STATES,
      Intents.FLAGS.GUILD_MESSAGES,
      Intents.FLAGS.GUILD_INTEGRATIONS
    ]
  });
}
