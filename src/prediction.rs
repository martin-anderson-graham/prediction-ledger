pub mod prediction {
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

    #[derive(Debug, Default)]
    pub struct Prediction {
        pub description: String,
        pub certainty: f32,
    }

    impl Prediction {
        pub fn new(description: &str, certainty: f32) -> Result<Self, PredictionParseError> {
            match Prediction::validate_certainty(certainty) {
                true => Ok(Self {
                    description: description.to_string(),
                    certainty,
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
    }
}

#[cfg(test)]
mod prediction_tests {
    use crate::prediction::prediction::Prediction;

    #[test]
    fn test_invalid_certainty_values() {
        let negative_certainty_prediction = Prediction::new("", -0.3);
        assert!(negative_certainty_prediction.is_err());

        let greater_than_one_certainty_prediction = Prediction::new("", 1.3);
        assert!(greater_than_one_certainty_prediction.is_err());

        let valid_certainty_predection = Prediction::new("", 0.4);
        assert!(valid_certainty_predection.is_ok());
    }
}
