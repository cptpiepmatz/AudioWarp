import {
  createAudioPlayer,
  createAudioResource,
  NoSubscriberBehavior,
  AudioPlayer,
  StreamType
} from "@discordjs/voice";

import {AudioSettings} from "../audio/selectAudioSettings.js";
import createAudioStream from "../audio/createAudioStream.js";

/**
 * This creates the audio player that takes the audio stream and does some
 * music to make it playable for discord.
 * @param audioSettings Audio settings so that this function can setup the
 *                      stream itself
 */
export default function createRadioPlayer(
  audioSettings: AudioSettings
): AudioPlayer & { startStreaming: () => void } {
  const player = createAudioPlayer({
    behaviors: {
      noSubscriber: NoSubscriberBehavior.Play,
      maxMissedFrames: 100
    }
  });

  /**
   * This function lets the player start with a new audio input stream.
   * May be used if the old one fails but since it currently works without,
   * this just stands here.
   */
  function startStreaming() {
    player.play(createAudioResource(createAudioStream(audioSettings), {
      inputType: StreamType.OggOpus
    }));
  }

  return Object.assign(player, {startStreaming: startStreaming});
}

