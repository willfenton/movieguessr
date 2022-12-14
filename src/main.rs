#![allow(clippy::large_enum_variant, clippy::result_large_err, dead_code, unused_imports, unused_variables)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use tracing::{event, Level};

mod disk;
mod game;
mod movie;
mod movie_manager;
mod omdb;
mod tmdb;

use crate::disk::Disk;
use crate::game::Game;
use crate::movie::Movie;
use crate::movie_manager::MovieManager;

/// Movie guessing game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IMDb ID of the movie to fetch
    #[arg(short, long)]
    imdb_id: String,
}

fn main() {
    tracing_subscriber::fmt::init();

    // let args = dbg!(Args::parse());

    let omdb_api_key = fs::read_to_string("omdb-apikey.txt").unwrap();
    let tmdb_api_key = fs::read_to_string("tmdb-apikey.txt").unwrap();

    let movie_manager = MovieManager::new(omdb_api_key, tmdb_api_key);

    let game = Game::new(movie_manager);

    let file_contents = fs::read_to_string("data/lists/imdb-popular-100.txt").unwrap();
    let imdb_top_250: Vec<&str> = file_contents.lines().collect();

    for imdb_id in imdb_top_250 {
        game.play(imdb_id);
    }
}

// loop {
//     get movie
//     clear screen
//     print movie info
//     loop {
//         get input
//         if input matches title, break
//     }
// }
