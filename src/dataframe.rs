use crate::error::OpenAIResponseError;
use crate::nlp::get_openai_response;
// TODO: use polars::frame::group_by::GroupBy;
use polars::prelude::*;
use serde_json::Value;
// TODO: use std::collections::HashMap;
use std::error::Error;

/// Extracts code blocks from a given text.
///
/// # Arguments
///
/// * `text` - The text containing code blocks.
///
/// # Returns
///
/// A concatenated string containing code blocks.
///
/// # Examples
///
/// ```
/// let markdown_text = "Some text before the code block...```Rust\n// Rust code...\n```Some text after the code block...";
/// let code_block = extract_code_blocks(markdown_text);
/// println!("Extracted Code Block:\n{}", code_block);
/// ```
fn extract_code_blocks(text: &str) -> String {
    let mut code_block = String::new();
    let mut in_code_block = false;

    for line in text.lines() {
        if line.starts_with("```") {
            if in_code_block {
                in_code_block = false;
            } else {
                in_code_block = true;
            }
        } else if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        }
    }

    code_block
}

/// Represents a data frame with AI-related functionality.
pub struct AIDataFrame {
    /// The inner Polars DataFrame.
    pub inner_df: DataFrame,
    /// The last executed AI command.
    pub last_command: Option<String>,
}

impl AIDataFrame {
    /// Creates a new `AIDataFrame` instance.
    ///
    /// # Arguments
    ///
    /// * `inner_df` - The inner Polars DataFrame.
    ///
    /// # Returns
    ///
    /// A new `AIDataFrame` instance.
    pub fn new(inner_df: DataFrame) -> Self {
        AIDataFrame {
            inner_df,
            last_command: None,
        }
    }

    /// Asynchronously asks an AI-related query and stores the last command.
    ///
    /// # Arguments
    ///
    /// * `query` - The AI-related query to ask.
    ///
    /// # Returns
    ///
    /// A result containing the AI response.
    pub async fn ask(&mut self, query: &str) -> Result<String, Box<dyn Error>> {
        let ai_response = get_openai_response(query, &self.inner_df).await.unwrap();

        let ai_result: Value = serde_json::from_str((*ai_response).into()).unwrap();
        if let Some(content) = ai_result["choices"][0]["message"]["content"].as_str() {
            let code_block = extract_code_blocks(content);
            self.last_command = Some(code_block.clone());
            Ok(code_block)
        } else {
            Err(Box::new(OpenAIResponseError::new(
                "AI response content is empty or not a string.",
            )) as Box<dyn Error>)
        }
    }

    /// Retrieves the shape of the underlying DataFrame.
    ///
    /// # Returns
    ///
    /// A tuple representing the shape as `(number_of_rows, number_of_columns)`.
    pub fn shape(&self) -> (usize, usize) {
        self.inner_df.shape()
    }

    /// Retrieves the number of columns in the underlying DataFrame.
    ///
    /// # Returns
    ///
    /// The number of columns.
    pub fn width(&self) -> usize {
        self.inner_df.width()
    }

    /// Retrieves the number of rows in the underlying DataFrame.
    ///
    /// # Returns
    ///
    /// The number of rows.
    pub fn height(&self) -> usize {
        self.inner_df.height()
    }

    /// Selects specific columns from the DataFrame.
    ///
    /// # Arguments
    ///
    /// * `columns` - A list of column names to select.
    ///
    /// # Returns
    ///
    /// A result containing the new DataFrame with selected columns.
    pub fn select(&self, columns: &[String]) -> Result<DataFrame, PolarsError> {
        self.inner_df.select(columns)
    }

    /// Performs a cross join with another DataFrame.
    ///
    /// # Arguments
    ///
    /// * `other` - The DataFrame to perform the cross join with.
    /// * `suffix` - An optional suffix for columns with the same name in both DataFrames.
    /// * `slice` - An optional slice specification for rows.
    ///
    /// # Returns
    ///
    /// A result containing the resulting DataFrame.
    pub fn cross_join(
        &self,
        other: &DataFrame,
        suffix: Option<&str>,
        slice: Option<(i64, usize)>,
    ) -> Result<DataFrame, PolarsError> {
        self.inner_df.cross_join(other, suffix, slice)
    }
}
