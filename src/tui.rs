// use crate::components::*;
use ratatui::prelude::*;
use ratatui::layout::Layout;

use crate::components::{self, playing_menu::PlayingMenu, song_menu, Component};

enum Selection {
    SongMenu { line_num: u32 },
    PlaylistMenu { line_num: u32 },
    PlayingMenu(),
}

pub enum Events {
    Init,
    Quit,
}

pub struct TUI {
    pub terminal: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,
    selection: Selection,
    layout: Layout,
}

impl TUI {
    pub fn new() -> Self {
        let layout = Layout::default();
        TUI{
            terminal: ratatui::init(),
            selection: Selection::SongMenu { line_num: 0 },
            layout,
        }
    }

    // pub fn init() {
    // }

    // pub fn run()

    pub fn quit(&mut self) {
        ratatui::restore();
    }
}
