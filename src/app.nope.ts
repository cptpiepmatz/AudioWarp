import {AudioIO, getDevices} from "naudiodon";
import cliSelect from "cli-select";
import {Client, Guild, GuildMember, Intents, Interaction} from "discord.js";
import {
  AudioPlayerStatus,
  createAudioPlayer,
  createAudioResource, getVoiceConnection,
  joinVoiceChannel,
  NoSubscriberBehavior,
  StreamType
} from "@discordjs/voice";
import {Readable} from "stream";
import {FFmpeg} from "prism-media";
import {createWriteStream} from "fs";
import { join } from "path";

// haha lol, bad practice
const token = "ODc0MzQ0Njk2NzI4Njc4NDEw.YRFm9A.tDoC7pAWMBPTETiNsHwK4KPwPpw";
const client = new Client({
  intents: [
    Intents.FLAGS.GUILDS,
    Intents.FLAGS.GUILD_VOICE_STATES,
    Intents.FLAGS.GUILD_MESSAGES,
    Intents.FLAGS.GUILD_INTEGRATIONS
  ]
});

(async function() {
  console.log("Reading possible devices...");
  const possibleDevices: {[key: number]: string} = {};
  for (let device of getDevices()) {
    possibleDevices[device.id] = device.name;
  }

  console.log("Select your input device.");
  const response = await cliSelect({values: possibleDevices});
  const audioIn = AudioIO({
    inOptions: {
      deviceId: +response.id,
      channelCount: 1,
      sampleRate: 65000 * 2
    }
  });

  const transcoder = new FFmpeg({
    args: [
      "-analyzeduration",
      "0",
      "-loglevel",
      "1",
      "-acodec",
      "libopus",
      "-f",
      "opus",
      "-ar",
      "48000",
      "-ac",
      "2",
    ]
  });

  await client.login(token);
  console.log("Logged in to Discord.");

  console.log("Registering commands...");
  await client.application?.commands.set([
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
  console.log("Registered commands.");

  const resource = createAudioResource(audioIn as unknown as Readable, {
    inputType: StreamType.Raw
  });
  console.log("Created audio resource.");

  const player = createAudioPlayer({
    behaviors: {
      noSubscriber: NoSubscriberBehavior.Play
    }
  });

  player.on("stateChange", ((oldState, newState) => {
    console.log(`Player transitioned from ${oldState.status} to ${newState.status}`);
  }))

  audioIn.pipe(createWriteStream(join(process.cwd(), "rawAudio.raw")));

  player.play(resource);
  audioIn.start();
  console.log("Created audio player.");


  process.on("SIGINT", function() {
    player.stop();
    audioIn.quit();
    client.destroy();
    process.exit(0);
  });

  client.on("interactionCreate", (interaction: Interaction) => {
    if (!interaction.isCommand()) return;
    if (!interaction.guild) return;
    const guild = interaction.guild as Guild;
    const member = interaction.member as GuildMember;

    if (interaction.commandName === "warp") {
      interaction.reply({
        ephemeral: true,
        content: "Warping to you..."
      });

      const channel = member.voice.channel;
      if (!channel) return;
      const connection = joinVoiceChannel({
        selfMute: false,
        selfDeaf: false,
        channelId: channel.id,
        guildId: guild.id,
        adapterCreator: guild.voiceAdapterCreator
      });

      connection.subscribe(player);
    }

    if (interaction.commandName == "leave") {
      const connection = getVoiceConnection(guild.id);
      connection?.destroy();
      interaction.reply({
        ephemeral: true,
        content: "Ok, I'm leaving."
      });
    }

  });

  console.log("Listening to interactions now...");

})();

