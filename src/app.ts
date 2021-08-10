import fetchAudioDevices from "./audio/fetchAudioDevices";
import selectAudioSettings from "./audio/selectAudioSettings";
import createClient from "./discord/createClient";
import readToken from "./discord/readToken";
import createRadioPlayer from "./voice/createAudioPlayer";
import deployInteractionHandler from "./discord/deployInteractionHandler";
import setCommands from "./discord/setCommands";
import ora from "ora";
import chalk from "chalk";

const spinner = ora();
spinner.color = "magenta";
spinner.spinner = "dots";

console.log(`Running ${chalk.magenta("AudioWarp")}, a tool to warp your ` +
  `music input to ${chalk.blue("Discord")}.`);

(async function () {
  spinner.start("Fetching audio devices...");
  const devices = await fetchAudioDevices();
  spinner.succeed("Fetched audio devices");

  const settings = await selectAudioSettings(devices);

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

  spinner.stop();
  spinner.succeed("Booted up");

  function terminate() {
    console.log(chalk.red("Shutting down!"));
    client.destroy();
    process.exit();
  }

  process.on("SIGINT", terminate);
  process.on("SIGHUP", terminate);
})();
