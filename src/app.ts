// Main file for the bot, the entry point of the core logic.

import chalk from "chalk";
import createRPCClient from "discord-rich-presence";
import ora from "ora";

import fetchAudioDevices from "./audio/fetchAudioDevices.js";
import selectAudioSettings from "./audio/selectAudioSettings.js";
import selectAntiAFKMeasure from "./discord/selectAntiAFKMeasure.js";
import createClient from "./discord/createClient.js";
import readToken from "./discord/readToken.js";
import setCommands from "./discord/setCommands.js";
import deployInteractionHandler from "./discord/deployInteractionHandler.js";
import startAntiAFKMeasure from "./discord/startAntiAFKMeasure.js";
import createRadioPlayer from "./voice/createAudioPlayer.js";


const rpcClientId = "874344696728678410";

// The spinner used to displaying actions taking some time
const spinner = ora();
spinner.color = "magenta";
spinner.spinner = "dots";

// Some details about this tool, not much
console.log(`Running ${chalk.magenta("AudioWarp")}, a tool to warp your ` +
  `music input to ${chalk.blue("Discord")}.`);

(async function () {
  // Here starts the main logic.
  spinner.start("Fetching audio devices...");
  const devices = await fetchAudioDevices();
  spinner.succeed("Fetched audio devices");

  const settings = await selectAudioSettings(devices);

  const antiAFKMeasure = await selectAntiAFKMeasure();

  spinner.start("Creating client...");
  const client = createClient();
  spinner.succeed("Created Discord client");

  spinner.start("Reading token...");
  const token = readToken();
  spinner.succeed("Read token");

  spinner.start("Logging in...");
  await client.login(token);
  spinner.succeed("Logged in");

  spinner.start("Creating radio player...");
  const player = createRadioPlayer(settings);
  player.startStreaming();
  spinner.succeed("Created radio player");

  spinner.start("Registering commands...");
  await setCommands(client);
  spinner.succeed("Registered commands");

  spinner.start("Deploying interaction handlers...");
  deployInteractionHandler(client, player);
  spinner.succeed("Deployed interaction handlers");

  spinner.start("Setting up RPC...");
  const rpcClient = createRPCClient(rpcClientId);
  await rpcClient.updatePresence({
    details: "Streaming Audio",
    state: "Input device: " + settings.device,
    largeImageKey: "icon",
    largeImageText: "AudioWarp",
    startTimestamp: new Date()
  });
  spinner.succeed("Set up RPC");

  spinner.start("Setting up anti AFK measures...");
  let antiAfkTimer = startAntiAFKMeasure(antiAFKMeasure);
  spinner.succeed("Set up anti AFK measures");

  spinner.stop();
  spinner.succeed("Booted up");

  function terminate() {
    console.log(chalk.red("Shutting down!"));
    client.destroy();
    rpcClient.disconnect();
    if (antiAfkTimer) clearInterval(antiAfkTimer);
    process.exit();
  }

  process.on("SIGINT", terminate);
  process.on("SIGHUP", terminate);
})();
