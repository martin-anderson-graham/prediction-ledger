use std::io;

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding, Paragraph, Wrap,
    },
};

use crate::{app::app::App, prediction::prediction::Prediction};

use super::Component;

pub enum Action {
    Noop,
    SetPredictions(Vec<Prediction>),
}

#[derive(Default)]
pub struct PredictionList {
    predictions: Vec<Prediction>,
}

impl PredictionList {
    pub fn new(predictions: Option<Vec<Prediction>>) -> Self {
        Self {
            predictions: predictions.unwrap_or(vec![]),
        }
    }
}

impl PredictionList {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::Noop => return,
            Action::SetPredictions(predictions) => self.predictions = predictions.to_vec(),
        };
    }
}

impl Component for PredictionList {
    fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn draw(
        &mut self,
        f: &mut ratatui::prelude::Frame<'_>,
        area: ratatui::prelude::Rect,
        app_state: &App,
    ) -> color_eyre::eyre::Result<()> {
        let mut constraints: Vec<Constraint> = vec![
            // for the title
            Constraint::Length(1),
        ];

        let instructions = Title::from(Line::from(vec![
            "<j/k> Up/down".blue().into(),
            "-".into(),
            "<esc/q> Exit".red().into(),
        ]));

        f.render_widget(
            Block::bordered()
                .title(
                    instructions
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL),
            area,
        );
        constraints.extend(
            self.predictions
                .iter()
                .map(|_| Constraint::Length(1))
                .collect::<Vec<_>>(),
        );
        constraints.push(Constraint::Fill(1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        f.render_widget(Paragraph::new("Prediction list").centered(), chunks[0]);

        for (i, prediction) in self.predictions.iter().enumerate() {
            let is_focused: bool = app_state
                .focused_prediction_id
                .is_some_and(|id| id == prediction.id);

            let prediction_text = format!("{}- {}", i + 1, prediction.title.clone());

            let predection_text_colored = match is_focused {
                true => prediction_text.white().on_blue(),
                false => prediction_text.blue(),
            };

            let prediction_text_formatted =
                Text::from(Line::from(vec![predection_text_colored.bold().into()]));
            f.render_widget(
                Paragraph::new(prediction_text_formatted)
                    .wrap(Wrap { trim: false })
                    .block(Block::default().padding(Padding::horizontal(2))),
                chunks[i + 1],
            );
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn handle_key_events(&mut self, _key_event: KeyEvent) {}
}
