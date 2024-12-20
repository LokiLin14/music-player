use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    // path to the json file containing data about the playlist
    filepath : PathBuf,
    // vector of all the songs listed in songs.json
    songs : Vec<Song>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    // absolute path to the file for the mp3 or similar for the song
    song_filepath : PathBuf,
    // optional absolute path for the image to display when playing the song
    image_filepath : Option<PathBuf>
}

#[derive(Debug)]
struct State {
    // path to folder to app folder (folder that contains playlist.json, songs.json, and files)
    app_folder : PathBuf,
    // vector that stores all the playlists obtained by deserializing playlists.json
    playlists : Vec<Playlist>,
    // vector that stores all the songs obtained by deserializing songs.json
    songs : Vec<Song>
}

impl State {
    fn save(&self) {
        todo!()
    }

    fn new(app_path : &Path) -> Self {
        todo!()
    }
}