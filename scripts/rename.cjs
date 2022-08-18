/*
This is being used to rename the file that pkg creates.
It renames it to use "AudioWarp" as it's name and concat the version from the
package.json and the current short commit hash as build data.
This should conform the semantic versioning.
 */

const fs = require("fs");
const path = require("path");
const git = require("git-last-commit");
const packageJson = require("../package.json");

const distPath = path.join(__dirname, "../dist");

git.getLastCommit((err, commit) => {
  fs.renameSync(
    distPath + "/audio-warp.exe",
    `${distPath}/AudioWarp-${packageJson.version}+${commit.shortHash}.exe`
  );
});
