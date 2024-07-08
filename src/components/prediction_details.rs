use std::{io, vec};

use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Padding, Paragraph},
};
use time::format_description;

use crate::{app::app::App, prediction::prediction::Prediction};

use super::Component;

#[derive(Default)]
pub struct PredictionDetails {
    prediction: Option<Prediction>,
}

impl PredictionDetails {
    pub fn new(prediction: Option<&Prediction>) -> Self {
        Self {
            prediction: match prediction {
                Some(prediction) => Some(prediction.clone()),
                None => None,
            },
        }
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
        _app_state: &App,
    ) -> color_eyre::eyre::Result<()> {
        let constraints: Vec<Constraint> = vec![
            // for the block title
            Constraint::Length(1),
            // for the prediction title
            Constraint::Length(3),
            // for the description
            Constraint::Length(3),
            // for the certainty
            Constraint::Length(3),
            // for the created date
            Constraint::Length(3),
            // for the due date
            Constraint::Length(3),
            Constraint::Fill(1),
        ];

        f.render_widget(Block::bordered().borders(Borders::ALL), area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        let date_format = format_description::parse("[year]-[month]-[day]").unwrap();

        let (title, description, certainty, created, due) = match &self.prediction {
            None => (
                "None".to_string(),
                "None".to_string(),
                "None".to_string(),
                "None".to_string(),
                "None".to_string(),
            ),
            Some(prediction) => {
                let formated_due = match prediction.due {
                    Some(date) => date.format(&date_format).unwrap_or("None".to_string()),
                    None => "None".to_string(),
                };
                (
                    prediction.title.clone(),
                    prediction.description.clone(),
                    prediction.certainty.to_string(),
                    prediction
                        .created
                        .format(&date_format)
                        .unwrap_or("None".to_string()),
                    formated_due,
                )
            }
        };

        let title_formatted = Text::from(vec![
            Line::from("Title:".white().bold()).into(),
            Line::from(title).into(),
        ]);
        f.render_widget(
            Paragraph::new(title_formatted)
                .block(Block::default().padding(Padding::new(2, 2, 0, 1))),
            chunks[1],
        );

        let description_formatted = Text::from(vec![
            Line::from("Description:".white().bold()).into(),
            Line::from(description).into(),
        ]);
        f.render_widget(
            Paragraph::new(description_formatted)
                .block(Block::default().padding(Padding::new(2, 2, 0, 1))),
            chunks[2],
        );

        let certainty_formatted = Text::from(vec![
            Line::from("Certainty:".white().bold()).into(),
            Line::from(certainty).into(),
        ]);
        f.render_widget(
            Paragraph::new(certainty_formatted)
                .block(Block::default().padding(Padding::new(2, 2, 0, 1))),
            chunks[3],
        );

        let created_formatted = Text::from(vec![
            Line::from("Created:".white().bold()).into(),
            Line::from(created).into(),
        ]);
        f.render_widget(
            Paragraph::new(created_formatted)
                .block(Block::default().padding(Padding::new(2, 2, 0, 1))),
            chunks[4],
        );

        let due_formatted = Text::from(vec![
            Line::from("Due:".white().bold()).into(),
            Line::from(due).into(),
        ]);
        f.render_widget(
            Paragraph::new(due_formatted).block(Block::default().padding(Padding::new(2, 2, 0, 1))),
            chunks[5],
        );

        f.render_widget(Paragraph::new("Prediction details").centered(), chunks[0]);
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn handle_key_events(&mut self, _key_event: KeyEvent) {}
}
