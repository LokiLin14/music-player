use std::collections::VecDeque;
use crate::model::Song;

struct MusicPlayer {
    song_queue: VecDeque<Song>,
    current_song: Song,
    // todo!("current position in song"),
}