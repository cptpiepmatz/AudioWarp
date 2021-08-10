import {AudioPlayer, createAudioPlayer, createAudioResource, NoSubscriberBehavior, StreamType} from "@discordjs/voice";
import {AudioSettings} from "../audio/selectAudioSettings";
import createAudioStream from "../audio/createAudioStream";

export default function createRadioPlayer(
  audioSettings: AudioSettings
): AudioPlayer & { startStreaming: () => void } {
  const player = createAudioPlayer({
    behaviors: {
      noSubscriber: NoSubscriberBehavior.Play,
      maxMissedFrames: 100
    }
  });

  function startStreaming() {
    player.play(createAudioResource(createAudioStream(audioSettings), {
      inputType: StreamType.OggOpus
    }));
  }

  return Object.assign(player, {startStreaming: startStreaming});
}

