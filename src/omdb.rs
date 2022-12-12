use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use ureq::{Agent, AgentBuilder, Error};

pub struct OmdbClient {
    api_key: String,
    agent: Agent,
}

impl OmdbClient {
    pub fn new(api_key: String) -> OmdbClient {
        OmdbClient {
            api_key,
            agent: AgentBuilder::new()
                .timeout_read(Duration::from_secs(5))
                .timeout_write(Duration::from_secs(5))
                .build(),
        }
    }

    pub fn get_movie(self, imdb_id: &str) -> Result<OmdbResponse, Error> {
        let response: OmdbResponse = self
            .agent
            .get("http://www.omdbapi.com")
            .query("apikey", &self.api_key)
            .query("i", imdb_id)
            .query("plot", "full")
            .call()?
            .into_json()
            .unwrap();

        match &response {
            OmdbResponse::Success(movie) => {
                if movie.extra.len() > 0 {
                    println!("Extra fields in OMDB response for {}: {:?}", imdb_id, movie.extra);
                }
            }
            OmdbResponse::Error(error) => {
                println!("Error response from OMDB for {}: {}", imdb_id, error.error)
            }
        }

        Ok(response)
    }
}

// Serde will try to match the data against each variant in order,
// and the first one that deserializes successfully is the one returned
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OmdbResponse {
    Error(OmdbError),
    Success(OmdbMovie),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OmdbMovie {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Rated")]
    pub rated: String,
    #[serde(rename = "Released")]
    pub released: String,
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "Genre")]
    pub genre: String,
    #[serde(rename = "Director")]
    pub director: String,
    #[serde(rename = "Writer")]
    pub writer: String,
    #[serde(rename = "Actors")]
    pub actors: String,
    #[serde(rename = "Plot")]
    pub plot: String,
    #[serde(rename = "Language")]
    pub language: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Awards")]
    pub awards: String,
    #[serde(rename = "Poster")]
    pub poster: String,
    #[serde(rename = "Ratings")]
    pub ratings: Vec<OmdbMovieRating>,
    #[serde(rename = "Metascore")]
    pub metascore: String,
    #[serde(rename = "imdbRating")]
    pub imdb_rating: String,
    #[serde(rename = "imdbVotes")]
    pub imdb_votes: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "DVD")]
    pub dvd: String,
    #[serde(rename = "BoxOffice")]
    pub box_office: String,
    #[serde(rename = "Production")]
    pub production: String,
    #[serde(rename = "Website")]
    pub website: String,
    #[serde(rename = "Response")]
    pub response: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OmdbMovieRating {
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OmdbError {
    #[serde(rename = "Response")]
    pub response: String,
    #[serde(rename = "Error")]
    pub error: String,
}
