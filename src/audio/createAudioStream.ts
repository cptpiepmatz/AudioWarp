import {AudioSettings} from "./selectAudioSettings";
import {FFmpeg} from "prism-media";

export default function createAudioStream(settings: AudioSettings) {
  return new FFmpeg({
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
