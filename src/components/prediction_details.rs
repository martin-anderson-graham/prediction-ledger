use std::io;

use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Borders, Paragraph};

use super::Component;

#[derive(Default)]
pub struct PredictionDetails {}

impl PredictionDetails {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for PredictionDetails {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn draw(
        &mut self,
        f: &mut ratatui::prelude::Frame<'_>,
        area: ratatui::prelude::Rect,
    ) -> color_eyre::eyre::Result<()> {
        f.render_widget(
            Paragraph::new("Prediction details")
                .centered()
                .block(Block::default().borders(Borders::ALL)),
            area,
        );
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn handle_key_events(&mut self, _key_event: KeyEvent) {}
}
