use crate::movie::Movie;
use crate::movie_manager::MovieManager;
use crossterm::cursor::MoveTo;
use crossterm::execute;
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

    pub fn play(&self, imdb_id: &str) -> Result<(), Error> {
        let movie = self.movie_manager.get_movie(imdb_id);

        // reset
        execute!(stdout(), Clear(All), MoveTo(0, 0))?;

        movie.print();
        sleep(Duration::from_secs(1));

        Ok(())
    }
}
