import {prompt} from "inquirer";

/** Interface for the audio settings. */
export interface AudioSettings {
  device: string;
  channelAmount: 1 | 2 | number;
  sampleRate: number;
}

/**
 * This function uses the promp from inquirer to ask the use some questions.
 * It asks to select the audio device and how it should be sampled.
 * The resulting answers will be returned.
 * @param audioDevices String array of possible audio devices
 */
export default async function selectAudioSettings(
  audioDevices: string[]
): Promise<AudioSettings> {
  const answers: {
    device: string,
    channelAmount: "mono" | "stereo",
    sampleRate: number
  } = await prompt([
    {
      type: "list",
      name: "device",
      message: "Select audio device to listen to.",
      choices: audioDevices
    },
    {
      type: "list",
      name: "channelAmount",
      message: "Select how to listen to your device.",
      choices: ["stereo", "mono"]
    },
    {
      type: "number",
      name: "sampleRate",
      message: "Input your sample rate.",
      default: 48000
    }
  ]);

  return {
    device: answers.device,
    channelAmount: answers.channelAmount === "mono" ? 1 : 2,
    sampleRate: answers.sampleRate
  };
}
