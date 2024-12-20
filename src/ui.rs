enum Selection {
    SongMenu { line_num: u32 },
    PlaylistMenu { line_num: u32 },
    PlayingMenu(),
}

pub struct UiState {
    selection: Selection,
}

impl UiState {
    pub fn new() -> Self {
        UiState{
            selection: Selection::SongMenu { line_num: 0 }
        }
    }
}
