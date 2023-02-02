#![feature(let_chains)]

use crate::os_generic::config_dir;

mod cli;
mod files;
mod os_generic;

fn main() {
    match std::fs::create_dir(config_dir()) {
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => {
            panic!("unable to create config dir")
        }
        _ => ()
    }
    cli::Cli::start(".");
}
