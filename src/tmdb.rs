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
        TMDbClient {
            api_key,
            agent: AgentBuilder::new()
                .timeout_read(Duration::from_secs(5))
                .timeout_write(Duration::from_secs(5))
                .build(),
        }
    }

    pub fn get_movie(self, imdb_id: &str) -> Result<String, Error> {
        let path = format!("https://api.themoviedb.org/3/find/{}", imdb_id);
        let response: String = self
            .agent
            .get(&path)
            .query("api_key", &self.api_key)
            .query("external_source", "imdb_id")
            .call()?
            .into_string()
            .unwrap();

        // TODO: this doesn't actually get the full movie

        Ok(response)
    }
}
