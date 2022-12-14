#![allow(clippy::large_enum_variant, clippy::result_large_err, dead_code, unused_imports, unused_variables)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

mod disk;
mod movie_manager;
mod omdb;
mod tmdb;

use crate::disk::Disk;
use crate::movie_manager::{Movie, MovieManager};

/// Movie guessing game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IMDb ID of the movie to fetch
    #[arg(short, long)]
    imdb_id: String,
}

fn main() {
    // let args = dbg!(Args::parse());

    let omdb_api_key = fs::read_to_string("omdb-apikey.txt").unwrap();
    let tmdb_api_key = fs::read_to_string("tmdb-apikey.txt").unwrap();

    let movie_manager = MovieManager::new(omdb_api_key, tmdb_api_key);

    let file_contents = fs::read_to_string("data/lists/imdb-top-250.txt").unwrap();
    let imdb_top_250: Vec<&str> = file_contents.lines().collect();

    let movies: Vec<Movie> = imdb_top_250.iter().map(|imdb_id| movie_manager.get_movie(imdb_id)).collect();

    movies
        .iter()
        .for_each(|movie| println!("{} ({}) [IMDb:{}, TMDb:{}]", movie.tmdb.title, movie.omdb.year, movie.imdb_id, movie.tmdb.id));
}
