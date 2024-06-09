pub mod prediction {
    use crate::components::Component;
    use ratatui::prelude::*;
    use ratatui::text::Line;
    use ratatui::widgets::{Block, Padding, Paragraph, Wrap};
    use time::OffsetDateTime;

    #[derive(Debug)]
    pub struct PredictionParseError {
        message: String,
    }
    impl PredictionParseError {
        fn new(message: String) -> Self {
            PredictionParseError { message }
        }
    }
    impl std::fmt::Display for PredictionParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }
    impl std::error::Error for PredictionParseError {}

    #[derive(Debug, Clone)]
    pub struct Prediction {
        pub title: String,
        description: String,
        certainty: f32,
        created: OffsetDateTime,
        due: Option<OffsetDateTime>,
    }

    impl Prediction {
        pub fn new(
            title: &str,
            description: &str,
            certainty: f32,
        ) -> Result<Self, PredictionParseError> {
            let now = OffsetDateTime::now_local().unwrap();

            match Prediction::validate_certainty(certainty) {
                true => Ok(Self {
                    title: title.to_string(),
                    description: description.to_string(),
                    certainty,
                    created: now,
                    due: None,
                }),
                false => Err(PredictionParseError::new(format!(
                    "Certainty values must be between 0 and 1 inclusive - received {}",
                    certainty
                ))),
            }
        }

        fn validate_certainty(certainty: f32) -> bool {
            return certainty >= 0.0 && certainty <= 1.0;
        }

        pub fn get_certainty(&self) -> String {
            self.certainty.to_string().clone()
        }

        pub fn get_description(&self) -> String {
            self.description.to_string().clone()
        }

        pub fn get_formatted_created_date(&self) -> String {
            let created_format = time::macros::format_description!("[month]/[day]/[year]");
            self.created.format(&created_format).unwrap()
        }

        pub fn get_formatted_due_date(&self) -> String {
            let due_format = time::macros::format_description!("[month]/[day]/[year]");
            match self.due {
                Some(value) => value.format(&due_format).unwrap(),
                None => "None".to_string(),
            }
        }
    }
    impl Component for Prediction {
        fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> color_eyre::eyre::Result<()> {
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
}

#[cfg(test)]
mod prediction_tests {
    use crate::prediction::prediction::Prediction;

    #[test]
    fn test_invalid_certainty_values() {
        let negative_certainty_prediction = Prediction::new("", "", -0.3);
        assert!(negative_certainty_prediction.is_err());

        let greater_than_one_certainty_prediction = Prediction::new("", "", 1.3);
        assert!(greater_than_one_certainty_prediction.is_err());

        let valid_certainty_predection = Prediction::new("", "", 0.4);
        assert!(valid_certainty_predection.is_ok());
    }
}
