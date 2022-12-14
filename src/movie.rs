use serde::{Deserialize, Serialize};

use crate::omdb::models::OMDbMovie;
use crate::tmdb::models::{TMDbMovie, TMDbMovieCrewMember};

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    pub imdb_id: String,
    pub omdb: OMDbMovie,
    pub tmdb: TMDbMovie,
}

impl Movie {
    pub fn print(&self) {
        println!("Title: {}", self.tmdb.title);
        println!("Plot: {}", self.omdb.plot);
        println!("Year: {}", self.omdb.year);
        println!("IMDb ID: {}", self.imdb_id);
        println!("TMDb ID: {}", self.tmdb.id);

        match self.tmdb.credits.crew.iter().find(|crew_member| crew_member.job == "Director") {
            None => println!("No director"),
            Some(director) => println!("Director: {}", director.name),
        };

        println!();
    }
}
