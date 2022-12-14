use serde::{Deserialize, Serialize};

use crate::disk::Disk;
use crate::omdb::client::OMDbClient;
use crate::omdb::models::{OMDbGetMovieResponse, OMDbMovie};
use crate::tmdb::client::TMDbClient;
use crate::tmdb::models::{TMDbGetMovieResponse, TMDbMovie};

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub imdb_id: String,
    pub omdb: OMDbMovie,
    pub tmdb: TMDbMovie,
}

pub struct MovieManager {
    disk: Disk,
    omdb_client: OMDbClient,
    tmdb_client: TMDbClient,
}

impl MovieManager {
    pub fn new(omdb_api_key: String, tmdb_api_key: String) -> MovieManager {
        MovieManager {
            disk: Disk::new(),
            omdb_client: OMDbClient::new(omdb_api_key),
            tmdb_client: TMDbClient::new(tmdb_api_key),
        }
    }

    // TODO: better error handling
    pub fn get_movie(&self, imdb_id: &str) -> Movie {
        // we might already have the movie downloaded
        if let Some(movie) = self.disk.get_movie(imdb_id) {
            return movie;
        }

        let omdb_movie = match self.omdb_client.get_movie(imdb_id).unwrap() {
            OMDbGetMovieResponse::Success(movie) => movie,
            OMDbGetMovieResponse::Error(error) => panic!("{error:?}"),
        };

        let tmdb_id = self.tmdb_client.find_movie(imdb_id).unwrap().unwrap().id;

        let tmdb_movie = match self.tmdb_client.get_movie(tmdb_id).unwrap() {
            TMDbGetMovieResponse::Success(movie) => movie,
            TMDbGetMovieResponse::Error(error) => panic!("{error:?}"),
        };

        let movie = Movie {
            imdb_id: String::from(imdb_id),
            omdb: omdb_movie,
            tmdb: tmdb_movie,
        };

        self.disk.write_movie(&movie).unwrap();

        movie
    }
}
