const rcedit = require("rcedit");
const path = require("path");

rcedit(path.join(__dirname, "../dist/audio-warp.exe"), {
  icon: path.join(__dirname, "../icon/icon.ico")
}).catch(console.error);
