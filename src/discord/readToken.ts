import {dirname} from "dirname-filename-esm";
import {readFileSync} from "fs";
import {join} from "path";

/** This one reads the token from the ".token" file in the root in sync. */
export default function readToken(): string {
  let __dirname;
  return readFileSync(join(
    __dirname ?? dirname(import.meta),
    "../../.token"
  ), "utf8").trim();
}
