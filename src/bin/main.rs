
// Parses arguments and starts the game

use std::{env::args, process::exit};
use bracket_terminal::prelude::*;

use tikis::{SCREEN_HEIGHT, SCREEN_WIDTH};
use tikis::state::State;

const HELP: &str = "
tikis version pre-alpha

USAGE: cargo run -- [OPTIONS]

OPTIONS:
    -h, --help          display this message
";

fn main() {
    let mut args = args().into_iter();
    while let Some(parameter) = args.next() {
        match parameter.as_str() {
            "-h" | "--help" => { print!("{}", HELP); exit(0) },
            _ => {}
        }
    }

    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Tikis - Pre-alpha")
        .with_fps_cap(30.)
        .build().expect("error setting up Bterm");

    let gs = State::new();

    main_loop(context, gs).unwrap();
}