import {
  AudioPlayer,
  AudioPlayerStatus,
  createAudioPlayer,
  createAudioResource,
  NoSubscriberBehavior,
  StreamType
} from "@discordjs/voice";
import {Readable} from "stream";
import {AudioSettings} from "../audio/selectAudioSettings";
import createAudioStream from "../audio/createAudioStream";

export default function createRadioPlayer(
  audioSettings: AudioSettings
): AudioPlayer & {startStreaming: () => void} {
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

  player.on('stateChange', (oldState, newState) => {
    if (oldState.status === AudioPlayerStatus.Idle && newState.status === AudioPlayerStatus.Playing) {
      console.log('Playing audio output on audio player');
    } else if (newState.status === AudioPlayerStatus.Idle) {
      console.log('Playback has stopped. Attempting to restart.');
      startStreaming();
    }
  });

  return Object.assign(player, {startStreaming: startStreaming});
}

