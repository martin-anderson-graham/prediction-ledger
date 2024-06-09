use std::io;

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

use crate::prediction::prediction::Prediction;

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
    ) -> color_eyre::eyre::Result<()> {
        let mut constraints: Vec<Constraint> = vec![
            // for the title
            Constraint::Length(1),
        ];

        f.render_widget(Block::default().borders(Borders::ALL), area);
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
            let prediction_text = Text::from(Line::from(vec![format!(
                "{}- {}",
                i + 1,
                prediction.title.clone()
            )
            .blue()
            .bold()
            .into()]));
            f.render_widget(
                Paragraph::new(prediction_text)
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
