use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // std::env::args returns an iterator over the CLI arguments passed into this binary.
    let args = env::args().collect::<Vec<_>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
}