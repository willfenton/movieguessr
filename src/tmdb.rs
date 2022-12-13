use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error};

pub struct TMDbClient {
    api_key: String,
    agent: Agent,
}

impl TMDbClient {
    pub fn new(api_key: String) -> TMDbClient {
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        TMDbClient { api_key, agent }
    }

    pub fn find_movie(self, imdb_id: &str) -> Result<Option<TMDbFindMovieResult>, Error> {
        let path = format!("https://api.themoviedb.org/3/find/{}", imdb_id);
        let response: TMDbFindResponse = self
            .agent
            .get(&path)
            .query("api_key", &self.api_key)
            .query("external_source", "imdb_id")
            .call()?
            .into_json()
            .unwrap();

        match response {
            TMDbFindResponse::Error(_) => todo!(),
            TMDbFindResponse::Success(mut results) => {
                if results.movie_results.len() != 1 {
                    todo!();
                }
                let movie_result = results.movie_results.pop().unwrap();
                let tmdb_id = movie_result.id;
                println!("Resolved IMDb ID {} to TMDb ID {}", imdb_id, movie_result.id);
                Ok(Some(movie_result))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TMDbFindResponse {
    Error(TMBbError),
    Success(TMDbFindResults),
}

// https://developers.themoviedb.org/3/find/find-by-id
#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbFindMovieResult {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub id: i64,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub release_date: String,
    pub poster_path: Option<String>,
    pub popularity: f64,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbFindResults {
    pub movie_results: Vec<TMDbFindMovieResult>,
    pub person_results: Vec<()>,
    pub tv_results: Vec<()>,
    pub tv_episode_results: Vec<()>,
    pub tv_season_results: Vec<()>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMBbError {
    pub status_message: String,
    pub status_code: i64,
}
