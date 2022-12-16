use crate::movie::Movie;
use crate::movie_manager::MovieManager;
use crossterm::cursor::{MoveRight, MoveTo, MoveToNextLine};
use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::All;
use std::io::{stdout, Error};
use std::thread::sleep;
use std::time::Duration;

pub struct Game {
    movie_manager: MovieManager,
}

impl Game {
    pub fn new(movie_manager: MovieManager) -> Self {
        Self { movie_manager }
    }

    pub async fn play(&self, imdb_id: &str) -> Result<(), Error> {
        let movie = self.movie_manager.get_movie(imdb_id).await;

        // reset
        // execute!(stdout(), Clear(All), MoveTo(0, 0))?;

        // execute!(
        //     stdout(),
        //     MoveToNextLine(1),
        //     MoveRight(1),
        //     Print(format!("{} ({})", movie.tmdb.title, movie.omdb.year)),
        //     SetAttribute(Attribute::Reset)
        // )?;
        // execute!(
        //     stdout(),
        //     MoveToNextLine(1),
        //     MoveRight(1),
        //     Print(format!("Directed by {}", movie.omdb.director)),
        //     SetAttribute(Attribute::Reset)
        // )?;
        // execute!(
        //     stdout(),
        //     MoveToNextLine(1),
        //     MoveRight(1),
        //     Print(format!("Featuring {}", movie.omdb.actors)),
        //     SetAttribute(Attribute::Reset)
        // )?;
        //
        // execute!(stdout(), MoveToNextLine(2), Print(movie.omdb.plot), SetAttribute(Attribute::Reset))?;
        // if movie.tmdb.overview.is_some() {
        //     execute!(stdout(), MoveToNextLine(2), Print(movie.tmdb.overview.unwrap()), SetAttribute(Attribute::Reset))?;
        // }
        // if movie.tmdb.tagline.is_some() {
        //     execute!(stdout(), MoveToNextLine(2), Print(movie.tmdb.tagline.unwrap()), SetAttribute(Attribute::Reset))?;
        // }

        movie.print();
        sleep(Duration::from_secs(1));

        Ok(())
    }
}
