/*
This file is used to set up the app in the packaged .exe.
This is needed because the AudioWarp uses ffmpeg for the audio interaction.
To easily use it, this setup extracts the ffmpeg.exe from the final .exe and
places it next to it.
 */

import {mkdirSync, readFileSync, writeFileSync} from "fs";
import {join} from "path";

const snapshotPath = join(__dirname, "../lib");
const snapshotFfmpeg = join(snapshotPath, "ffmpeg.exe");

const realPath = join(process.cwd(), "lib");
const realFfmpeg = join(realPath, "ffmpeg.exe");

const file = readFileSync(snapshotFfmpeg);
mkdirSync(realPath, {recursive: true});
writeFileSync(realFfmpeg, file);

process.env.FFMPEG_BIN = realFfmpeg;

require("./app");
