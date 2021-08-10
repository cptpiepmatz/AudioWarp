import {prompt} from "inquirer";

export default async function selectAudioSettings(
  audioDevices: string[]
): Promise<{
  device: string,
  channelAmount: number,
  sampleRate: number
}> {
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
      choices: ["mono", "stereo"]
    },
    {
      type: "number",
      name: "sampleRate",
      message: "Input your sample rate.",
      default: 4800
    }
  ]);

  return {
    device: answers.device,
    channelAmount: answers.channelAmount === "mono" ? 1 : 2,
    sampleRate: answers.sampleRate
  };
}
