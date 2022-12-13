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
        let response: TMDbGetMovieResponse = self
            .agent
            .get(&path)
            .query("api_key", &self.api_key)
            .query("append_to_response", "credits,keywords")
            .call()?
            .into_json()
            .unwrap();

        Ok(response)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TMDbFindMovieResponse {
    Error(TMBbError),
    Success(TMDbFindResults),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TMDbGetMovieResponse {
    Error(TMBbError),
    Success(TMDbMovie),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieKeywords {
    pub keywords: Vec<TMDbMovieKeyword>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieKeyword {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieCrewMember {
    pub adult: bool,
    pub gender: i64,
    pub id: i64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub credit_id: String,
    pub department: String,
    pub job: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieCastMember {
    pub adult: bool,
    pub gender: i64,
    pub id: i64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub cast_id: i64,
    pub character: String,
    pub credit_id: String,
    pub order: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieCredits {
    pub cast: Vec<TMDbMovieCastMember>,
    pub crew: Vec<TMDbMovieCrewMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieSpokenLanguage {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieProductionCompany {
    pub id: i64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieGenre {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieCollection {
    pub id: i64,
    pub name: String,
    pub poster_path: String,
    pub backdrop_path: String,
}

// https://developers.themoviedb.org/3/movies/get-movie-details
// https://developers.themoviedb.org/3/movies/get-movie-credits
// https://developers.themoviedb.org/3/movies/get-movie-keywords
#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovie {
    pub adult: bool,
    pub backdrop_path: String,
    pub belongs_to_collection: TMDbMovieCollection,
    pub budget: i64,
    pub genres: Vec<TMDbMovieGenre>,
    pub homepage: String,
    pub id: i64,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub production_companies: Vec<TMDbMovieProductionCompany>,
    pub production_countries: Vec<TMDbMovieProductionCountry>,
    pub release_date: String,
    pub revenue: i64,
    pub runtime: i64,
    pub spoken_languages: Vec<TMDbMovieSpokenLanguage>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
    pub credits: TMDbMovieCredits,
    pub keywords: TMDbMovieKeywords,
}
