use clap::Parser;
use std::fs;

mod omdb;

use crate::omdb::OmdbClient;

/// Movie guessing game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IMDB ID of the movie to fetch
    #[arg(short, long)]
    imdb_id: String,
}

fn main() {
    let args = Args::parse();

    let omdb_api_key = fs::read_to_string("omdb-apikey.txt").unwrap();
    let omdb_client = OmdbClient::new(omdb_api_key);

    let response = omdb_client.get_movie(&args.imdb_id).unwrap();
    println!("{response:#?}");
}
