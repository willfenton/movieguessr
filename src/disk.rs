use serde_json::{from_str, to_string, to_string_pretty};
use std::io::Error;
use std::path::PathBuf;
use tokio::fs::{create_dir, read_to_string, write};

use crate::movie::Movie;

#[derive(Debug, Clone)]
pub struct Disk {
    data_dir: PathBuf,
}

impl Disk {
    pub async fn new() -> Self {
        let mut data_dir = dirs::data_dir().unwrap();
        data_dir.push("movieguessr/");

        if !&data_dir.exists() {
            create_dir(&data_dir).await.unwrap();
        }

        Self { data_dir }
    }

    fn path_for(&self, imdb_id: &str) -> PathBuf {
        let mut path = self.data_dir.clone();
        path.push(format!("{}.json", imdb_id));
        path
    }

    pub async fn get_movie(&self, imdb_id: &str) -> Option<Movie> {
        let path = self.path_for(imdb_id);
        match path.exists() {
            true => {
                let file_contents = read_to_string(path).await.unwrap();
                let movie: Movie = from_str(&file_contents).expect("deserialize Movie");
                Some(movie)
            }
            false => None,
        }
    }

    pub async fn write_movie(&self, movie: &Movie) -> Result<(), Error> {
        let path = self.path_for(&movie.imdb_id);
        let serialized_movie = to_string(movie).unwrap();
        write(path, serialized_movie).await
    }
}
