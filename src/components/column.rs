use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::Component;

#[derive(Default)]
pub struct Column {}

impl Column {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Column {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn draw(
        &mut self,
        _f: &mut ratatui::prelude::Frame<'_>,
        _area: ratatui::prelude::Rect,
    ) -> color_eyre::eyre::Result<()> {
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // fn handle_events(&mut self, event:) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_events(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {}
            _ => {}
        }
    }
}
