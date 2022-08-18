/**
 * Fetching audio devices.
 * It runs the ffmpeg with the list_devices flag to return the audio devices
 * of the currently running machine.
 * If ffmpeg finds some devices, it will output them.
 * This function searches in the output for that lines and extracts them as
 * an array of their names.
 */
import {exec} from "child_process";
import ffmpegStatic from "ffmpeg-static";

export default function fetchAudioDevices(): Promise<string[]> {
  return new Promise(((resolve, reject) => {
    const command = `"${ffmpegStatic}" ` +
      "-list_devices true -f dshow -i dummy";
    exec(command, (error, stdout, stderr) => {
      // With this way of executing ffmpeg it will always return non-zero.
      // Therefore, this needs to inspect the stderr output.

      const noEnumerate =
        "Could not enumerate audio only devices (or none found).";
      if (stderr.includes(noEnumerate)) {
        return reject(new Error(noEnumerate));
      }

      const searchMatcher = /\[dshow @ \w+]\s+"(?<device>[^"]+)"\s+\(audio\)/g;
      let findings = Array.from(stderr.matchAll(searchMatcher))
        .map(v => v.groups?.device)
        .filter(v => !!v) as string[];
      if (findings.length) return resolve(findings);

      reject(error);
    });
  }));
}
