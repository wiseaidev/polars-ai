use std::error::Error;
use std::fmt;

/// Custom error type for OpenAI response errors.
#[derive(Debug)]
pub struct OpenAIResponseError {
    /// Error message.
    message: String,
}

impl OpenAIResponseError {
    /// Creates a new instance of `OpenAIResponseError`.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message.
    ///
    /// # Returns
    ///
    /// A new instance of `OpenAIResponseError`.
    pub fn new(message: &str) -> Self {
        OpenAIResponseError {
            message: message.to_string(),
        }
    }
}

impl Error for OpenAIResponseError {}

impl fmt::Display for OpenAIResponseError {
    /// Formats the error for display.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A `fmt::Result`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OpenAI Response Error: {}", self.message)
    }
}
