import fetchAudioDevices from "./audio/fetchAudioDevices";
import selectAudioSettings from "./audio/selectAudioSettings";
import createAudioStream from "./audio/createAudioStream";
import createClient from "./discord/createClient";
import readToken from "./discord/readToken";
import createRadioPlayer from "./voice/createAudioPlayer";
import deployInteractionHandler from "./discord/deployInteractionHandler";
import setCommands from "./discord/setCommands";

(async function() {
  console.log("Fetching audio devices");
  const settings = await selectAudioSettings(await fetchAudioDevices());
  const client = createClient();
  console.log("Created client");
  const token = readToken();
  console.log("Read token");
  await client.login(token);
  console.log("Logged in");
  const player = createRadioPlayer(settings);
  console.log("Created player");
  player.startStreaming();
  await setCommands(client);
  console.log("Registered commands");
  deployInteractionHandler(client, player);
  console.log("done");
})();
