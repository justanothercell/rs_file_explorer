use crate::files::collect_items;

mod cli;
mod files;
mod os_generic;

fn main() {
    collect_items(".");
    //cli::Cli::start();
}
