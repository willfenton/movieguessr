use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Serde will try to match the data against each variant in order,
// and the first one that deserializes successfully is the one returned
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OMDbGetMovieResponse {
    Error(OMDbError),
    Success(OMDbMovie),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OMDbMovie {
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
    pub ratings: Vec<OMDbMovieRating>,
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
    pub extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OMDbMovieRating {
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OMDbError {
    #[serde(rename = "Response")]
    pub response: String,
    #[serde(rename = "Error")]
    pub error: String,
}
