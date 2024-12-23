use crate::components::Component;
use crate::tui::TUI;
use crate::components::playing_menu::PlayingMenu;
use crate::components::{self, playing_menu::PlayingMenu, song_menu, Component};
use crate::tui;

enum Selection {
    SongMenu { line_num: u32 },
    PlaylistMenu { line_num: u32 },
    PlayingMenu(),
}

pub struct App {
    selection: Selection,
    components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new() -> Self {
        let playing_menu: PlayingMenu = PlayingMenu::new();
        // todo!("adding in other components");
        let components: Vec<Box<dyn Component>> = vec![Box::new(playing_menu)];
        Self {
            selection: Selection::SongMenu{line_num: 0},
            components,
        }
    }

    pub async fn run(&mut self, tui: &mut TUI) {
        let tui = tui::TUI::new();
        tui.terminal.draw(|frame| {
            self.components.into_iter()
                .map(|component| component.draw(frame, frame.area())?)
        });
    }
}
