use std::fs;
use std::io::ErrorKind;

const ENV_NAME: &str = "INCLUDED_TOKEN";
const TOKEN_FILE: &str = ".token";

/// Build script to include the token file optionally.
///
/// If the local token file `.token` is missing, this script will set [ENV_NAME]
/// to en empty string which is handled as the file is non-existent.
///
/// # Panics
///
/// The script fails if the reading of the file fails with an Error that is not
/// [ErrorKind::NotFound].
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={TOKEN_FILE}");

    match fs::read_to_string(".token") {
        Ok(token) => println!("cargo:rustc-env={ENV_NAME}={}", token.trim()),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => println!("cargo:rustc-env={ENV_NAME}="),
            _ => panic!("cannot read .token file")
        }
    }
}
