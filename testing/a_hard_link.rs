use crate::files::collect_items;

mod cli;
mod files;
mod os_generic;

fn main() {
    for item in collect_items("testing") {
        println!("{}", item.render())
    }
    //cli::Cli::start();
}
