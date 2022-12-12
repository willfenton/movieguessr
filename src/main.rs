#![allow(clippy::large_enum_variant, clippy::result_large_err, dead_code)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

mod disk;
mod omdb;

use crate::disk::Disk;
use crate::omdb::{OMDbClient, OMDbMovie, OMDbResponse};

/// Movie guessing game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IMDB ID of the movie to fetch
    #[arg(short, long)]
    imdb_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    imdb_id: String,
    omdb: OMDbMovie,
}

fn main() {
    let args = dbg!(Args::parse());
    let disk = dbg!(Disk::new());

    let omdb_api_key = fs::read_to_string("omdb-apikey.txt").unwrap();
    let omdb_client = OMDbClient::new(omdb_api_key);

    // dbg!(&disk.get_movie(&args.imdb_id));

    let response = omdb_client.get_movie(&args.imdb_id).unwrap();

    match response {
        OMDbResponse::Success(omdb) => {
            let movie = Movie {
                imdb_id: String::from(&args.imdb_id),
                omdb,
            };
            disk.write_movie(&movie).unwrap();
        }
        OMDbResponse::Error(_) => {}
    }

    // println!("{response:#?}");
}
