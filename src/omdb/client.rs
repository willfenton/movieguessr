use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error};

use crate::omdb::models::OMDbGetMovieResponse;

pub struct OMDbClient {
    api_key: String,
    agent: Agent,
}

impl OMDbClient {
    pub fn new(api_key: String) -> OMDbClient {
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();
        OMDbClient { api_key, agent }
    }

    pub fn get_movie(&self, imdb_id: &str) -> Result<OMDbGetMovieResponse, Error> {
        // let body: String = self
        //     .agent
        //     .get("https://www.omdbapi.com")
        //     .query("apikey", &self.api_key)
        //     .query("i", imdb_id)
        //     .query("plot", "short")
        //     .call()?
        //     .into_string()
        //     .unwrap();

        // println!("{}", body);

        // let response: OMDbGetMovieResponse = serde_json::from_str(&body).unwrap();

        let response: OMDbGetMovieResponse = self
            .agent
            .get("https://www.omdbapi.com")
            .query("apikey", &self.api_key)
            .query("i", imdb_id)
            .query("plot", "short")
            .call()?
            .into_json()
            .unwrap();

        match &response {
            OMDbGetMovieResponse::Success(movie) => {
                if !movie.extra.is_empty() {
                    println!("Extra fields in OMDB response for {}: {:?}", imdb_id, movie.extra);
                }
            }
            OMDbGetMovieResponse::Error(error) => {
                println!("Error response from OMDB for {}: {}", imdb_id, error.error)
            }
        }

        Ok(response)
    }
}
