//
// EPITECH PROJECT, 2023
// rustoban
// File description:
// main
//

mod parser;
mod game;

use std::env;
use crate::game::rustoban::Game;
use crate::parser::rustobanparser::Parser;
use std::process;
use std::panic;

fn main() {
    #[warn(unused_mut)]
    #[warn(unused_variables)]

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./rustoban [map]");
        process::exit(84);
    }
    let map = args[1].clone();
    let parser  = match panic::catch_unwind(|| Parser::new(map)) {
        Ok(Ok(parser)) => parser,
        _ => {
            eprintln!("Failed to create parser, error in map.");
            process::exit(84);
        }
    };
    let mut game = Game::new(parser.map_data(), parser.all_goals(),
                                    parser.nb_boxes(), parser.player());
    // game.debug();
    let code = game.run();
    process::exit(code);
}
