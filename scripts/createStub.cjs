/**
 * Compile the stub with go, add the icon to it and then append the separator
 * text.
 */

const {copyFile, appendFile} = require("fs/promises");
const {join} = require("path");
const {exec} = require("child_process");
const rcedit = require("rcedit");

(async () => {
  await copyFile(
    join(__dirname, "../node_modules/caxa/stubs/stub.go"),
    join(__dirname, "../out/stub.go")
  );
  await new Promise((resolve, reject) => {
    exec("go build stub.go", {cwd: join(__dirname, "../out")}, error => {
      if (error) return reject(error);
      resolve();
    });
  });
  await rcedit(join(__dirname, "../out/stub.exe"), {
    icon: join(__dirname, "../icon/icon.ico")
  });
  await appendFile(join(__dirname, "../out/stub.exe"), "\nCAXACAXACAXA\n");
})();
