import {Client, Intents} from "discord.js";

export default function createClient(): Client {
  return new Client({
    intents: [
      Intents.FLAGS.GUILDS,
      Intents.FLAGS.GUILD_VOICE_STATES,
      Intents.FLAGS.GUILD_MESSAGES,
      Intents.FLAGS.GUILD_INTEGRATIONS
    ]
  })
}
