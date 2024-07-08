pub mod app {
    use crate::components::graph::Graph;
    use crate::components::prediction_details::PredictionDetails;
    use crate::components::prediction_list::PredictionList;
    use crate::components::Component;
    use crate::prediction::prediction::Prediction;
    use crate::tui;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::prelude::*;
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
        pub predictions: Vec<Prediction>,
        pub focused_prediction_id: Option<u64>,
    }
    impl App {
        /// runs the application's main loop until the user quits
        pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::eyre::Result<()> {
            let focused_prediction = match self.focused_prediction_id {
                Some(id) => self.predictions.iter().find(|p| p.id == id),
                None => None,
            };

            let mut prediction_list_component = PredictionList::new(Some(self.predictions.clone()));
            let mut graph_component = Graph::new();
            let mut prediction_details_component = PredictionDetails::new(focused_prediction);

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

                    let _ = prediction_list_component.draw(frame, left_area, &self);
                    let _ = graph_component.draw(frame, graph_box_area, &self);
                    let _ = prediction_details_component.draw(frame, info_box_area, &self);
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
            let dummy_predictions = vec![
                Prediction::new(0, "Culinary prediction", "The cheese will be good", 0.4).unwrap(),
                Prediction::new(1, "Economic prediction", "I will win the lotto", 0.1).unwrap(),
            ];

            App {
                exit: false,
                mode: ScreenMode::PredictionList,
                focused_prediction_id: Some(dummy_predictions.get(0).unwrap().id),
                predictions: dummy_predictions,
            }
        }
    }
}
