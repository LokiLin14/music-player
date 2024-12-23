use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{error, UnboundedSender};

use super::Component;
use crate::actions::Action;

#[derive(Default)]
pub struct PlayingMenu {
    command_tx: Option<UnboundedSender<Action>>,
}

impl PlayingMenu {
    pub fn new() -> Self{
        Self {command_tx: None}
    }
}

impl Component for PlayingMenu {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), ()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    // fn register_config_handler(&mut self, config: Config) -> Result<(), ()> {
    //     // self.config = config;
    //     Ok(())
    // }

    fn update(&mut self, action: Action) -> Result<Option<Action>, ()> {
        todo!()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<(), ()> {
        frame.render_widget(Paragraph::new("hello world"), area);
        Ok(())
    }
}

