import {Client} from "discord.js";

export default function setCommands(client: Client) {
  return client.application?.commands.set([
    {
      name: "warp",
      description: "Warp the bot to you.",
      options: [{
        name: "here",
        type: "SUB_COMMAND",
        description: "Warp the bot to you."
      }]
    },
    {
      name: "leave",
      description: "Let the bot leave.",
      options: [{
        name: "me",
        type: "SUB_COMMAND",
        description: "Let the bot leave."
      }]
    }
  ]);
}
