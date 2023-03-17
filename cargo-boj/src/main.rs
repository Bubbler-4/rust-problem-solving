mod submit;
mod optparse;
mod datastore;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36";

// Planned features:
// cargo-boj login [--bojautologin=<str> --onlinejudge=<str>]
//   register BOJ login cookie information.
//   if not specified in the flag, the cookies are entered through a prompt.
// cargo-boj test <prob> [--bin=<bin>]
//   fetch sample tests for <prob> and run tests on the binary.
//   sample tests are cached by problem id.
//   if bin is supplied, uses its bin name. if set is also set, it is stored for later runs.
//   if bin is not supplied, the stored bin name is used; if it doesn't exist, defaults to `main`.
// cargo-boj submit <prob> [--path=<path>] [--lang-id=<lang>] [--code-open=(y|n|acc)]
//   submit the file at <path> as the solution to problem <prob>.
//   each option defaults to:
//   path = src/main.rs or src/bin/main.rs
//   lang-id = 113 (Rust 2021)
//   code-open = follow account default
// cargo-boj set [--bin=<bin>] [--path=<path>]
//   store settings for test binary name and submit file path.

use optparse::*;
use std::fs;
use std::io::{self, Write};

fn main() -> Result<()> {
    let opts = cargo_boj_opts();
    match opts {
        Opts::Login(Login { cookies }) => {
            let stdin = io::stdin();
            let mut stdout = io::stdout();
            let mut credentials = datastore::Credentials::load();
            credentials.remove();
            let cookies = cookies.unwrap_or_else(|| {
                println!("First log in to www.acmicpc.net on your browser with auto-login enabled.");
                println!("Then copy and paste two cookies for www.acmicpc.net from your browser.");
                print!("bojautologin: ");
                stdout.flush().unwrap();
                let mut bojautologin = String::new();
                stdin.read_line(&mut bojautologin).unwrap();
                print!("OnlineJudge: ");
                stdout.flush().unwrap();
                let mut onlinejudge = String::new();
                stdin.read_line(&mut onlinejudge).unwrap();
                datastore::Cookies { bojautologin, onlinejudge }
            });
            credentials.update_cookie(&cookies);
            println!("Cookies set.");
        }
        Opts::Submit(Submit { problem_id, language }) => {
            let language = language.unwrap_or(113);
            let credentials = datastore::Credentials::load();
            let Some(cookies) = &credentials.cookies else {
                println!("Use `cargo-boj login` first to log in.");
                return Ok(());
            };
            let source = ["src/main.rs", "src/bin/main.rs"]
                .into_iter()
                .find_map(|file| fs::read_to_string(file).ok());
            let Some(source) = source else {
                println!("Neither src/main.rs nor src/bin/main.rs not found. Try running again at the crate root.");
                return Ok(());
            };
            submit::submit_solution(cookies, &problem_id, &source, language);
        }
    }
    Ok(())
}