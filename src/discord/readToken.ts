import {readFileSync} from "fs";
import {join} from "path";

export default function readToken(): string {
  return readFileSync(join(__dirname, "../../.token"), "utf8").trim();
}
