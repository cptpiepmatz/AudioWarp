/*
This file is used to inject the .ico into the prebuilt binary from pkg.
This also renames the fetched file to allow this change.
After the first run this will fail but I certainly don't care.
 */

const rcedit = require("rcedit");
const path = require("path");
const os = require("os");
const fs = require("fs/promises");

const prebuilt = path
  .join(os.homedir(), "./.pkg-cache/v3.2/fetched-v16.4.1-win-x64");

rcedit(prebuilt, {
  icon: path.join(__dirname, "../icon/icon.ico")
})
  .then(() => fs.rename(prebuilt, prebuilt.replace("fetched-", "built-")))
  .catch(console.error);
