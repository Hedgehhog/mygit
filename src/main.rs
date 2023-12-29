

use std::env;
use std::io::{self, Write};
use std::collections::HashMap;

mod database;
mod repository;
mod refs;
mod index;
mod workspace;
mod lockfile;
mod util;
mod commands;
use commands::{execute, get_app, CommandContext};

fn main() {
    let ctx = CommandContext {
        dir: env::current_dir().unwrap(),
        env: &env::vars().collect::<HashMap<String, String>>(),
        options: None,
        stdin: io::stdin(),
        stdout: io::stdout(),
        stderr: io::stderr(),
    };

    let matches = get_app().get_matches();

    match execute(matches, ctx) {
        Ok(_) => (),
        Err(msg) => {
            io::stderr().write_all(msg.as_bytes()).unwrap();
            std::process::exit(128);
        }
    }
    
    println!("Yes!");
}
