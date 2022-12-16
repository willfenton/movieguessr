use std::time::Duration;

use crate::tmdb::models::{TMDbFindMovieResponse, TMDbFindMovieResult, TMDbGetMovieResponse, TMDbMovie};

pub struct TMDbClient {
    api_key: String,
    client: reqwest::Client,
}

impl TMDbClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
        Self { api_key, client }
    }

    pub async fn find_movie(&self, imdb_id: &str) -> Option<TMDbFindMovieResult> {
        let path = format!("https://api.themoviedb.org/3/find/{}", imdb_id);
        let query_params: [(&str, &str); 2] = [("api_key", &self.api_key), ("external_source", "imdb_id")];
        let response: TMDbFindMovieResponse = self.client.get(path).query(&query_params).send().await.unwrap().json().await.unwrap();

        match response {
            TMDbFindMovieResponse::Error(_) => todo!(),
            TMDbFindMovieResponse::Success(mut results) => {
                if results.movie_results.len() != 1 {
                    todo!();
                }
                let movie_result = results.movie_results.pop().unwrap();
                let tmdb_id = movie_result.id;
                println!("Resolved IMDb ID {} to TMDb ID {}", imdb_id, movie_result.id);
                Some(movie_result)
            }
        }
    }

    pub async fn get_movie(&self, tmdb_id: i64) -> TMDbGetMovieResponse {
        let path = format!("https://api.themoviedb.org/3/movie/{}", tmdb_id);

        let query_params: [(&str, &str); 2] = [("api_key", &self.api_key), ("append_to_response", "credits,keywords")];
        let response: TMDbGetMovieResponse = self.client.get(path).query(&query_params).send().await.unwrap().json().await.unwrap();

        response
    }
}
