use serde_json::to_string_pretty;
use std::fs::{create_dir, read_to_string, write};
use std::io::Error;
use std::path::PathBuf;

use crate::Movie;

#[derive(Debug)]
pub struct Disk {
    data_dir: PathBuf,
}

impl Disk {
    pub fn new() -> Disk {
        let mut data_dir = dirs::data_dir().unwrap();
        data_dir.push("movieguessr/");

        if !&data_dir.exists() {
            create_dir(&data_dir).unwrap();
        }

        Disk { data_dir }
    }

    fn path_for(self, imdb_id: &str) -> PathBuf {
        let mut path = self.data_dir;
        path.push(format!("{}.json", imdb_id));
        dbg!(&path);
        path
    }

    pub fn get_movie(self, imdb_id: &str) -> Option<String> {
        let path = self.path_for(imdb_id);
        match path.exists() {
            true => Some(read_to_string(path).unwrap()),
            false => None,
        }
    }

    pub fn write_movie(self, movie: &Movie) -> Result<(), Error> {
        let path = self.path_for(&movie.imdb_id);
        write(path, to_string_pretty(movie).unwrap())
    }
}
