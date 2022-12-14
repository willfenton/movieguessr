use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
pub struct TMDbMovieCredits {
    pub cast: Vec<TMDbMovieCastMember>,
    pub crew: Vec<TMDbMovieCrewMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovieCastMember {
    pub adult: bool,
    pub gender: Option<i64>,
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
pub struct TMDbMovieCrewMember {
    pub adult: bool,
    pub gender: Option<i64>,
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
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

// https://developers.themoviedb.org/3/movies/get-movie-details
// https://developers.themoviedb.org/3/movies/get-movie-credits
// https://developers.themoviedb.org/3/movies/get-movie-keywords
#[derive(Serialize, Deserialize, Debug)]
pub struct TMDbMovie {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<TMDbMovieCollection>,
    pub budget: i64,
    pub genres: Vec<TMDbMovieGenre>,
    pub homepage: Option<String>,
    pub id: i64,
    pub imdb_id: Option<String>,
    pub original_language: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub production_companies: Vec<TMDbMovieProductionCompany>,
    pub production_countries: Vec<TMDbMovieProductionCountry>,
    pub release_date: String,
    pub revenue: i64,
    pub runtime: Option<i64>,
    pub spoken_languages: Vec<TMDbMovieSpokenLanguage>,
    pub status: String,
    pub tagline: Option<String>,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
    pub credits: TMDbMovieCredits,
    pub keywords: TMDbMovieKeywords,
}
