import {Client, Guild, GuildMember, Interaction} from "discord.js";
import {AudioPlayer, getVoiceConnection, joinVoiceChannel, VoiceConnection} from "@discordjs/voice";

export default function deployInteractionHandler(
  client: Client,
  player: AudioPlayer
) {
  client.on("interactionCreate", async (interaction: Interaction) => {
    if (!interaction.isCommand()) return;

    if (!interaction.guild) return;
    const guild = interaction.guild as Guild;

    if (!interaction.member) return;
    const member = interaction.member as GuildMember;

    if (!member.voice.channel) return;

    let connection: VoiceConnection | undefined;

    switch (interaction.commandName) {
      case "warp":
        await interaction.reply({
          ephemeral: true,
          content: "I'm warping to you..."
        });
        connection = joinVoiceChannel({
          channelId: member.voice.channel.id,
          guildId: guild.id,
          adapterCreator: guild.voiceAdapterCreator,
          selfMute: false
        });
        connection.subscribe(player);
        break;
      case "leave":
        await interaction.reply({
          ephemeral: true,
          content: "Aight, imma head out."
        });
        connection = getVoiceConnection(guild.id);
        connection?.destroy();
    }
  });
}
