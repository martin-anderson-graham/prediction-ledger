pub mod app {
    use crate::components::column;
    use crate::components::Component;
    use crate::prediction::prediction::Prediction;
    use crate::tui;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::{
        prelude::*,
        symbols::border,
        widgets::{block::*, *},
    };
    use std::io;

    #[derive(Debug)]
    enum ScreenMode {
        PredictionList,
    }

    impl Default for ScreenMode {
        fn default() -> Self {
            ScreenMode::PredictionList
        }
    }

    pub struct App {
        exit: bool,
        mode: ScreenMode,
        predictions: Vec<Prediction>,
        components: Vec<Box<dyn Component>>,
    }
    impl App {
        /// runs the application's main loop until the user quits
        pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::eyre::Result<()> {
            while !self.exit {
                terminal.draw(|frame| {
                    for component in self.components.iter_mut() {
                        let _ = component.draw(frame, frame.size());
                    }
                })?;
                self.handle_events()?;
            }
            Ok(())
        }

        fn handle_events(&mut self) -> io::Result<()> {
            if event::poll(std::time::Duration::from_millis(16))? {
                match event::read()? {
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        self.handle_key_event(key_event)
                    }
                    _ => {}
                };
            }
            Ok(())
        }

        fn handle_key_event(&mut self, key_event: KeyEvent) {
            match self.mode {
                ScreenMode::PredictionList => match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    _ => {}
                },
            }
        }

        fn exit(&mut self) {
            self.exit = true;
        }
    }

    impl Default for App {
        fn default() -> Self {
            App {
                exit: false,
                mode: ScreenMode::PredictionList,
                predictions: vec![
                    Prediction::new("The cheese will be good", 0.4).unwrap(),
                    Prediction::new("I will win the lotto", 0.1).unwrap(),
                ],
                components: vec![Box::new(
                    Prediction::new("The cheese will be good", 0.4).unwrap(),
                )],
            }
        }
    }
}
