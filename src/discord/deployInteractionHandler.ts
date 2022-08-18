import {
  getVoiceConnection,
  joinVoiceChannel,
  VoiceConnection,
  AudioPlayer
} from "@discordjs/voice";
import {Client, Guild, Interaction, GuildMember} from "discord.js";

/**
 * This defines the interaction event handlers.
 * It listens for the "interactionCreate" event.
 * If something is off it just returns without any real error.
 * Since discord expects a reply in the time of 3 seconds the user always gets
 * some kind of feedback.
 *
 * If the "warp here" command is used the bot joins your voice channel, if it
 * can find it.
 *
 * On "leave me" it just leaves.
 * @param client The client to attach the handlers to
 * @param player An audio player that the voice connection should subscribe to
 */
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
