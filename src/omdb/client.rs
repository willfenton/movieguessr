use std::time::Duration;

use crate::omdb::models::OMDbGetMovieResponse;

pub struct OMDbClient {
    api_key: String,
    client: reqwest::Client,
}

impl OMDbClient {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder().timeout(Duration::from_secs(5)).build().unwrap();
        Self { api_key, client }
    }

    pub async fn get_movie(&self, imdb_id: &str) -> OMDbGetMovieResponse {
        let query_params: [(&str, &str); 3] = [("apikey", &self.api_key), ("i", imdb_id), ("plot", "short")];
        let response: OMDbGetMovieResponse = self
            .client
            .get("https://www.omdbapi.com")
            .query(&query_params)
            .send()
            .await
            .unwrap()
            .json()
            .await
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

        response
    }
}
