use thiserror::Error;

#[derive(Debug, Error)]
pub enum BotejaoError {
    #[error("Invalid date {invalid_date:?}. Parse error: {parse_error:?})")]
    InvalidDate {
        invalid_date: String,
        parse_error: String,
    },
    #[error("Invalid meal kind: {0}")]
    InvalidMealKind(String),
    #[error("Network Error: {0}")]
    NetworkError(String),
}
