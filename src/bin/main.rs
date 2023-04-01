
// Parses arguments and starts the game

use std::{env::args, process::exit};
use bracket_terminal::prelude::*;

use tikis::debug::DebugMode;
use tikis::{SCREEN_HEIGHT, SCREEN_WIDTH};
use tikis::state::State;

const HELP: &str = "
tikis version pre-alpha

USAGE: cargo run -- [OPTIONS]

OPTIONS:
    -h, --help          display this message
    -s, --seed          select seed for world generation
    --chunk             display chunk locations
    --dist <dist>       choose simulation distance
";

fn main() {
    let mut debug_mode = DebugMode::default();
    let mut args = args().into_iter();
    let mut seed = rand::random();
    let mut sim_distance = 40;

    while let Some(parameter) = args.next() {
        match parameter.as_str() {
            "-h" | "--help" => { print!("{}", HELP); exit(0); },
            "-s" | "--seed" => { seed = args.next().unwrap().parse().unwrap(); },
            "--chunk" => { debug_mode.display_chunk = true; },
            "--dist" => { sim_distance = args.next().unwrap().parse().unwrap(); },
            _ => {}
        }
    }

    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Tikis - Pre-alpha")
        .with_fps_cap(30.)
        .build().expect("error setting up Bterm");

    let gs = State::new(seed, debug_mode, sim_distance);

    main_loop(context, gs).unwrap();
}