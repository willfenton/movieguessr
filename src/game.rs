use crate::movie::Movie;
use crate::movie_manager::MovieManager;
use crossterm::cursor::{MoveRight, MoveTo, MoveToNextLine};
use crossterm::event::{Event, EventStream, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::All;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use futures::StreamExt;
use futures_timer::Delay;
use std::fs::read;
use std::io::{stdout, Error};
use std::thread::sleep;
use std::time::Duration;
use tokio::select;

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

    pub async fn print_events(&self) {
        enable_raw_mode().unwrap();

        let mut reader = EventStream::new();
        let mut delay = Delay::new(Duration::from_secs(1));

        'event_loop: loop {
            select! {
                _ = &mut delay => {
                    println!(".");
                    delay.reset(Duration::from_secs(1));
                },
                maybe_event = reader.next() => {
                    match maybe_event {
                        Some(Ok(event)) => {
                            match event {
                                Event::Key(key_event) => {
                                    println!("{key_event:?}");

                                    // if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c')

                                    if event == Event::Key(KeyCode::Esc.into()) {
                                        break 'event_loop;
                                    }
                                },
                                Event::Paste(pasted_string) => {
                                    println!("Pasted \"{}\"", pasted_string);
                                },
                                Event::Resize(w, h) => {
                                    println!("Resized to ({},{})", w, h);
                                },
                                _ => {}
                            }
                        }
                        Some(Err(e)) => println!("Error: {:?}\r", e),
                        None => break,
                    }
                }
            }
        }
    }
}
