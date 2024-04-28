pub mod app {
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
        Counter,
        Reset,
    }

    impl Default for ScreenMode {
        fn default() -> Self {
            ScreenMode::Counter
        }
    }

    #[derive(Debug, Default)]
    pub struct App {
        counter: u8,
        exit: bool,
        mode: ScreenMode,
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
                ScreenMode::Counter => match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    KeyCode::Char('z') => self.mode = ScreenMode::Reset,
                    KeyCode::Left => self.decrement_counter(),
                    KeyCode::Right => self.increment_counter(),
                    _ => {}
                },
                ScreenMode::Reset => match key_event.code {
                    KeyCode::Char('r') => self.reset_counter(),
                    KeyCode::Char('z') => self.mode = ScreenMode::Counter,
                    _ => {}
                },
            }
        }

        fn exit(&mut self) {
            self.exit = true;
        }
        fn increment_counter(&mut self) {
            self.counter += 1;
        }

        fn decrement_counter(&mut self) {
            self.counter -= 1;
        }

        fn reset_counter(&mut self) {
            self.counter = 0;
        }
    }

    impl Widget for &App {
        fn render(self, area: Rect, buf: &mut Buffer) {
            Clear.render(area, buf);
            let title = Title::from(" Counter App Tutorial ".bold());

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

            let counter_text = Text::from(vec![Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow(),
            ])]);

            let reset_text =
                Text::from(vec![Line::from(
                    vec!["Press z to reset the counter".into()],
                )]);

            Paragraph::new(match self.mode {
                ScreenMode::Counter => counter_text,
                ScreenMode::Reset => reset_text,
            })
            .centered()
            .block(block)
            .render(area, buf);
        }
    }
}

#[cfg(test)]
mod render_tests {
    use crate::app::app::App;
    use ratatui::prelude::*;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }
}
