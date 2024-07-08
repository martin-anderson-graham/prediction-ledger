use std::io;

use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::app::App;

use super::Component;

#[derive(Default)]
pub struct Graph {}

impl Graph {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Graph {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn draw(
        &mut self,
        f: &mut ratatui::prelude::Frame<'_>,
        area: ratatui::prelude::Rect,
        _app_state: &App,
    ) -> color_eyre::eyre::Result<()> {
        f.render_widget(Block::default().borders(Borders::ALL), area);
        f.render_widget(Paragraph::new("Graph area").centered(), area);
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn handle_key_events(&mut self, _key_event: KeyEvent) {}
}
