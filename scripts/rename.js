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
