pub mod app {
    use crate::components::prediction_details::PredictionDetails;
    use crate::components::prediction_list::PredictionList;
    use crate::components::Component;
    use crate::components::{column, graph::Graph};
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
            let mut prediction_list_component = PredictionList::new();
            let mut graph_component = Graph::new();
            let mut prediction_details_component = PredictionDetails::new();

            while !self.exit {
                terminal.draw(|frame| {
                    let area = frame.size();
                    let layout = Layout::horizontal([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .spacing(2);
                    let [left_area, right_area] = layout.areas(area);
                    let right_layout =
                        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
                    let [graph_box_area, info_box_area] = right_layout.areas(right_area);

                    let _ = prediction_list_component.draw(frame, left_area);
                    let _ = graph_component.draw(frame, graph_box_area);
                    let _ = prediction_details_component.draw(frame, info_box_area);
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
