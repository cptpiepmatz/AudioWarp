import getAudioDevices from "./audio/getAudioDevices";
import selectAudioSettings from "./audio/selectAudioSettings";

(async function() {
  const audioSettings = await selectAudioSettings(await getAudioDevices());
})();
