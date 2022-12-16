use serde::{Deserialize, Serialize};
use tracing::{event, span, Level};

use crate::disk::Disk;
use crate::movie::Movie;
use crate::omdb::client::OMDbClient;
use crate::omdb::models::{OMDbGetMovieResponse, OMDbMovie};
use crate::tmdb::client::TMDbClient;
use crate::tmdb::models::{TMDbGetMovieResponse, TMDbMovie};

pub struct MovieManager {
    disk: Disk,
    omdb_client: OMDbClient,
    tmdb_client: TMDbClient,
}

impl MovieManager {
    pub async fn new(omdb_api_key: String, tmdb_api_key: String) -> Self {
        Self {
            disk: Disk::new().await,
            omdb_client: OMDbClient::new(omdb_api_key),
            tmdb_client: TMDbClient::new(tmdb_api_key),
        }
    }

    // TODO: better error handling
    pub async fn get_movie(&self, imdb_id: &str) -> Movie {
        // let span = span!(Level::DEBUG, "get_movie", imdb_id = imdb_id).entered();

        // we might already have the movie downloaded
        if let Some(movie) = self.disk.get_movie(imdb_id).await {
            event!(Level::DEBUG, "loaded from disk");
            // span.exit();
            return movie;
        }

        let omdb_movie = match self.omdb_client.get_movie(imdb_id).await {
            OMDbGetMovieResponse::Success(movie) => movie,
            OMDbGetMovieResponse::Error(error) => panic!("{error:?}"),
        };

        let tmdb_id = self.tmdb_client.find_movie(imdb_id).await.unwrap().id;
        let tmdb_movie = match self.tmdb_client.get_movie(tmdb_id).await {
            TMDbGetMovieResponse::Success(movie) => movie,
            TMDbGetMovieResponse::Error(error) => panic!("{error:?}"),
        };

        let movie = Movie {
            imdb_id: String::from(imdb_id),
            omdb: omdb_movie,
            tmdb: tmdb_movie,
        };

        self.disk.write_movie(&movie).await.unwrap();

        // span.exit();

        movie
    }
}
