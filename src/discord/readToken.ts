import {readFileSync} from "fs";
import {join} from "path";

/** This one reads the token from the ".token" file in the root in sync. */
export default function readToken(): string {
  return readFileSync(join(__dirname, "../../.token"), "utf8").trim();
}
