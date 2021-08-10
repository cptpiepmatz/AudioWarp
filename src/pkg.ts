// This file is used to setup the app in the packaged .exe
import {join} from "path";
import {mkdirSync, readFileSync, writeFileSync} from "fs";

const snapshotPath = join(__dirname, "../lib");
const snapshotFfmpeg = join(snapshotPath, "ffmpeg.exe");

const realPath = join(process.cwd(), "lib");
const realFfmpeg = join(realPath, "ffmpeg.exe");

const file = readFileSync(snapshotFfmpeg);
mkdirSync(realPath, {recursive: true});
writeFileSync(realFfmpeg, file);

process.env.FFMPEG_BIN = realFfmpeg;

require("./app");
