import {Client, ApplicationCommandOptionType} from "discord.js";

/**
 * This one defines the commands the bot uses.
 *
 * It has two commands:
 *
 * One being the "warp here" that lets the bot join your voice channel and
 * listens to the audio stream.
 *
 * And the other being the "leave me" command that simply let's the bot leave
 * the channel.
 * @param client The client used to apply the commands to
 */
export default function setCommands(client: Client) {
  return client.application?.commands.set([
    {
      name: "warp",
      description: "Warp the bot to you.",
      options: [{
        name: "here",
        type: ApplicationCommandOptionType.Subcommand,
        description: "Warp the bot to you."
      }]
    },
    {
      name: "leave",
      description: "Let the bot leave.",
      options: [{
        name: "me",
        type: ApplicationCommandOptionType.Subcommand,
        description: "Let the bot leave."
      }]
    }
  ]);
}
