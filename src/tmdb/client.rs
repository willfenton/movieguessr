use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error};

use crate::tmdb::models::{TMDbFindMovieResponse, TMDbFindMovieResult, TMDbGetMovieResponse, TMDbMovie};

pub struct TMDbClient {
    api_key: String,
    agent: Agent,
}

impl TMDbClient {
    pub fn new(api_key: String) -> Self {
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        Self { api_key, agent }
    }

    pub fn find_movie(&self, imdb_id: &str) -> Result<Option<TMDbFindMovieResult>, Error> {
        let path = format!("https://api.themoviedb.org/3/find/{}", imdb_id);
        let response: TMDbFindMovieResponse = self
            .agent
            .get(&path)
            .query("api_key", &self.api_key)
            .query("external_source", "imdb_id")
            .call()?
            .into_json()
            .unwrap();

        match response {
            TMDbFindMovieResponse::Error(_) => todo!(),
            TMDbFindMovieResponse::Success(mut results) => {
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

    pub fn get_movie(&self, tmdb_id: i64) -> Result<TMDbGetMovieResponse, Error> {
        let path = format!("https://api.themoviedb.org/3/movie/{}", tmdb_id);

        let body: String = self
            .agent
            .get(&path)
            .query("api_key", &self.api_key)
            .query("append_to_response", "credits,keywords")
            .call()?
            .into_string()
            .unwrap();

        // println!("{}", body);

        // let response: TMDbGetMovieResponse = self
        //     .agent
        //     .get(&path)
        //     .query("api_key", &self.api_key)
        //     .query("append_to_response", "credits,keywords")
        //     .call()?
        //     .into_json()
        //     .unwrap();

        let response: TMDbMovie = serde_json::from_str(&body).unwrap();

        Ok(TMDbGetMovieResponse::Success(response))
    }
}
