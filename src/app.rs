pub mod app {
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

    #[derive(Debug)]
    pub struct App {
        exit: bool,
        mode: ScreenMode,
        predictions: Vec<Prediction>,
    }
    impl App {
        /// runs the application's main loop until the user quits
        pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
            while !self.exit {
                terminal.draw(|frame| self.render_frame(frame))?;
                self.handle_events()?;
            }
            Ok(())
        }

        fn render_frame(&self, frame: &mut Frame) {
            frame.render_widget(self, frame.size());
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

    impl Widget for &App {
        fn render(self, area: Rect, buf: &mut Buffer) {
            Clear.render(area, buf);
            let title = Title::from(" Prediction Ledger ".bold().white().on_blue());

            let instructions = Title::from(Line::from(vec![
                " Decrement ".into(),
                "<Left>".blue().bold().into(),
                " Increment ".into(),
                "<Right>".blue().bold().into(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]));

            let block = Block::default()
                .title(title.alignment(Alignment::Center))
                .title(
                    instructions
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_set(border::THICK);

            let prediction_text = Text::from(
                self.predictions
                    .iter()
                    .map(|prediction| {
                        Line::from(vec![
                            "Prediction: ".blue().bold().into(),
                            prediction.get_description().green().into(),
                            " - Certainty: ".bold().yellow().into(),
                            prediction.get_certainty().yellow().into(),
                            " - Created: ".bold().red().into(),
                            prediction.get_formatted_created_date().into(),
                            " - Due: ".bold().green().into(),
                            prediction.get_formatted_due_date().into(),
                        ])
                    })
                    .collect::<Vec<_>>(),
            );

            Paragraph::new(match self.mode {
                ScreenMode::PredictionList => prediction_text,
            })
            .centered()
            .block(block)
            .render(area, buf);
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
            }
        }
    }
}
