import prism from "prism-media";

import {AudioSettings} from "./selectAudioSettings.js";

/**
 * Creates an audio stream from the prism-media ffmpeg core class.
 * This is mostly used to save some space in the main logic and having all
 * the settings written here.
 * @param settings Audio settings that need to be fetched before
 */
export default function createAudioStream(settings: AudioSettings) {
  return new prism.FFmpeg({
    args: [
      "-analyzeduration", "0",
      "-loglevel", "0",
      "-f", "dshow",
      "-i", "audio=" + settings.device,
      "-acodec", "libopus",
      "-f", "opus",
      "-ar", `${settings.sampleRate}`,
      "-ac", `${settings.channelAmount}`
    ]
  });
}
