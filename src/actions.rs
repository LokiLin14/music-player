use crate::model::*;
pub enum Action {
    Play,
    Pause,
    SelectSong(Song),
    SelectPlaylist(Playlist),
}
