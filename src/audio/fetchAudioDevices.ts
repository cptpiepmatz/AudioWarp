import {exec} from "child_process";

/**
 * Fetching audio devices.
 * It runs the ffmpeg with the list_devices flag to return the audio devices
 * of the currently running machine.
 * If ffmpeg finds some devices, it will output them.
 * This function searches in the output for that lines and extracts them as
 * an array of their names.
 */
export default function fetchAudioDevices(): Promise<string[]> {
  return new Promise(((resolve, reject) => {
    const command = require("ffmpeg-static") +
      " -list_devices true -f dshow -i dummy";
    exec(command, (error, stdout, stderr) => {
      // With this way of executing ffmpeg it will always return non-zero.
      // Therefore this needs to inspect the stderr output.

      const noEnumerate =
        "Could not enumerate audio only devices (or none found).";
      if (stderr.includes(noEnumerate)) {
        return reject(new Error(noEnumerate));
      }

      const searchIndexer = "DirectShow audio devices";
      if (stderr.includes(searchIndexer)) {
        const index = stderr.indexOf(searchIndexer);
        const subSearch = stderr.substring(index + searchIndexer.length);
        const findings: string[] = [];
        for (let line of subSearch.split("\n")) {
          line = line.trim();
          if (!line.match(/\w/)) continue;
          if (line.includes("Alternative name")) continue;
          if (line.startsWith("dummy")) continue;
          findings.push(line.substring(
            line.indexOf('"') + 1,
            line.lastIndexOf('"')
          ));
        }
        return resolve(findings);
      }

      reject(error);
    });
  }));
}
