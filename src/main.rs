extern crate classic_rogue;

use std::process;

fn main() {
    match classic_rogue::init_game() {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
