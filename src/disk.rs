use serde_json::{to_string, to_string_pretty};
use std::fs::{create_dir, read_to_string, write};
use std::io::Error;
use std::path::PathBuf;

use crate::movie::Movie;

#[derive(Debug)]
pub struct Disk {
    data_dir: PathBuf,
}

impl Disk {
    pub fn new() -> Self {
        let mut data_dir = dirs::data_dir().unwrap();
        data_dir.push("movieguessr/");

        if !&data_dir.exists() {
            create_dir(&data_dir).unwrap();
        }

        Self { data_dir }
    }

    fn path_for(&self, imdb_id: &str) -> PathBuf {
        let mut path = self.data_dir.clone();
        path.push(format!("{}.json", imdb_id));
        path
    }

    pub fn get_movie(&self, imdb_id: &str) -> Option<Movie> {
        let path = self.path_for(imdb_id);
        match path.exists() {
            true => {
                let file_contents = read_to_string(path).unwrap();
                let movie: Movie = serde_json::from_str(&file_contents).expect("deserialize Movie");
                Some(movie)
            }
            false => None,
        }
    }

    pub fn write_movie(&self, movie: &Movie) -> Result<(), Error> {
        let path = self.path_for(&movie.imdb_id);
        let serialized_movie = to_string(movie).unwrap();
        write(path, serialized_movie)
    }
}
