import {exec} from "child_process";

export default function fetchAudioDevices(): Promise<string[]> {
  return new Promise(((resolve, reject) => {
    const command = require("ffmpeg-static") +
      " -list_devices true -f dshow -i dummy";
    exec(command, (error, stdout, stderr) => {
      const noEnumerate =
        "Could not enumerate audio only devices (or none found)."
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
