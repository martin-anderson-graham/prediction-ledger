use crate::app::app::App;
use crate::components::Component;
use prediction_ledger_core::Prediction;
use ratatui::prelude::*;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};

pub trait TuiPrediction {
    fn get_formatted_created_date(&self) -> String;
    fn get_formatted_due_date(&self) -> String;
}

impl TuiPrediction for Prediction {
    fn get_formatted_created_date(&self) -> String {
        let created_format = time::macros::format_description!("[month]/[day]/[year]");
        self.created.format(&created_format).unwrap()
    }

    fn get_formatted_due_date(&self) -> String {
        let due_format = time::macros::format_description!("[month]/[day]/[year]");
        match self.due {
            Some(value) => value.format(&due_format).unwrap(),
            None => "None".to_string(),
        }
    }
}

impl Component for Prediction {
    fn draw(
        &mut self,
        f: &mut Frame<'_>,
        area: Rect,
        _app_state: &App,
    ) -> color_eyre::eyre::Result<()> {
        let prediction_text = Text::from(Line::from(vec![
            "- ".blue().bold().into(),
            self.get_description().green().into(),
            " - Certainty: ".bold().yellow().into(),
            self.get_certainty().yellow().into(),
            " - Created: ".bold().red().into(),
            self.get_formatted_created_date().into(),
            " - Due: ".bold().green().into(),
            self.get_formatted_due_date().into(),
        ]));
        f.render_widget(
            Paragraph::new(prediction_text)
                .wrap(Wrap { trim: false })
                .block(Block::default().padding(Padding::horizontal(2))),
            area,
        );
        Ok(())
    }
}
