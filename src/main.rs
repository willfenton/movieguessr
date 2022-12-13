#![allow(clippy::large_enum_variant, clippy::result_large_err, dead_code, unused_imports, unused_variables)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

mod disk;
mod omdb;
mod tmdb;

use crate::disk::Disk;
use crate::omdb::{OMDbClient, OMDbGetMovieResponse, OMDbMovie};
use crate::tmdb::{TMDbClient, TMDbGetMovieResponse, TMDbMovie};

/// Movie guessing game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IMDb ID of the movie to fetch
    #[arg(short, long)]
    imdb_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    imdb_id: String,
    omdb: OMDbMovie,
    tmdb: TMDbMovie,
}

fn main() {
    let args = dbg!(Args::parse());
    let disk = dbg!(Disk::new());

    let omdb_api_key = fs::read_to_string("omdb-apikey.txt").unwrap();
    let tmdb_api_key = fs::read_to_string("tmdb-apikey.txt").unwrap();

    let omdb_client = OMDbClient::new(omdb_api_key);
    let tmdb_client = TMDbClient::new(tmdb_api_key);

    // dbg!(&disk.get_movie(&args.imdb_id));

    let omdb_get_movie_response = omdb_client.get_movie(&args.imdb_id).unwrap();
    let tmdb_find_movie_result = tmdb_client.find_movie(&args.imdb_id).unwrap().unwrap();
    let tmdb_get_movie_response = tmdb_client.get_movie(tmdb_find_movie_result.id).unwrap();

    // println!("{tmdb_movie:#?}");

    if let TMDbGetMovieResponse::Success(tmdb) = tmdb_get_movie_response {
        if let OMDbGetMovieResponse::Success(omdb) = omdb_get_movie_response {
            let movie = Movie {
                imdb_id: String::from(&args.imdb_id),
                omdb,
                tmdb,
            };
            disk.write_movie(&movie).unwrap();
        }
    }

    // disk.write_movie()

    // match omdb_response {
    //     OMDbResponse::Success(omdb) => {
    //         let movie = Movie {
    //             imdb_id: String::from(&args.imdb_id),
    //             omdb,
    //         };
    //         disk.write_movie(&movie).unwrap();
    //     }
    //     OMDbResponse::Error(_) => {}
    // }

    // println!("{response:#?}");
}
