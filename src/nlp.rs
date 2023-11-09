use polars::frame::DataFrame;
use polars::prelude::NamedFrom;
use polars::series::Series;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::error::Error;

/// Asynchronously retrieves an AI response from OpenAI based on a query and a DataFrame.
///
/// This function sends a user prompt, which includes the DataFrame information and a query, to
/// OpenAI's chat completions API. It then parses the AI response and returns it as a string.
///
/// # Arguments
///
/// * `query` - The query or question to ask the AI.
/// * `df` - The DataFrame to include in the AI prompt.
///
/// # Returns
///
/// Returns a result containing the AI response as a string or an error if any occurs.
pub async fn get_openai_response(query: &str, df: &DataFrame) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let endpoint = "https://api.openai.com/v1/chat/completions";

    let default_import = "use polars::frame::DataFrame;";
    let save_charts_path = "./charts/";
    let engine_df_name = "DataFrame";

    let prompt = format!(
        r###"
        You have been given access to a Polars DataFrame along with its associated columns headers:

        {}

        You are tasked with enhancing the following Rust code for data analysis. The code follows a structured approach:

        ```rust
        {}

        // Data Analysis Workflow
        // 1. Prepare: Preprocess and clean the data if required.
        // 2. Process: Manipulate the data for analysis (e.g., grouping, filtering, aggregating).
        // 3. Analyze: Conduct the analysis (if requested, save charts to {}/temp_chart.png, but do not display them).
        // 4. Output: Return a dictionary with the following structure:
        //    - type (possible values: "text", "number", "dataframe", "plot")
        //    - value (a string, a DataFrame, or the path to a plot, but not a dictionary)
        // Example output: 
        // 
        fn analyze_data(dfs: Vec<{}>) -> {} {{
            // Your code for data analysis goes here (do not include comments)
            let df = &dfs[0];

            // these methods should be in the following order, don't remove expect, also insert the column name inside an array []
            let top_carriers = df
                .groupby(['column name goes here'])
                .expect('Columns must exist!')
                .mean()
                .unwrap()
                .sort(['[column name goes here]_[aggregation function goes here. example: mean]'], false, false)
                .expect('[column name goes here]__[aggregation function goes here. example: mean] must exist')
                .head(Some('[limit number of rows. example 5]'))
                .select(&['[column name goes here]']).unwrap();

            // these methods should be in the following order, don't remove expect, also insert the column name inside an array []
            let result_df = df
                .join(&top_carriers, ['[shared column name goes here]'], ['[shared column name goes here]'], JoinType::Inner.into())
                .expect('[shared column name goes here] must exist')
                .sort(['[column name goes here]'], false, false)
                .expect('[column name goes here] must exist')
                .head(Some('[limit number of rows. example 5]'));

            result_df
                .select(&['[column name goes here]', '[another column name goes here]'])
                .unwrap()
        }}

        // Initialize a result variable
        let result = analyze_data(df);
        println!("{{}}", result);

        Now, based on the user's latest query in the conversation, update the Rust code:

        {}
        
        The revised code should reflect the user's latest question and incorporate the necessary modifications for data analysis. Please ensure that the comments are removed from the code, and the code structure remains intact.
    "###,
        df, default_import, save_charts_path, engine_df_name, engine_df_name, query
    );

    let response = client
        .post(endpoint)
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
            ),
        )
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "user",
                    "content": prompt,
                }
            ],
            "temperature": 0.7
        }))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Ok(response)
}

/// Parses an AI response from OpenAI into a DataFrame.
///
/// This function takes an AI response as a JSON string and extracts the text content.
/// It then splits the content into lines and creates a DataFrame with a single Series containing those lines.
///
/// # Arguments
///
/// * `query` - The AI response as a JSON string.
///
/// # Returns
///
/// Returns a result containing the parsed DataFrame or an error if parsing fails.
async fn _parse_openai_response(query: &str) -> Result<DataFrame, Box<dyn Error>> {
    let ai_response: Value = serde_json::from_str(query)?;
    println!("{:?}", query);
    // Extract the text field from the JSON and split it by line into a Vec<&str>
    if let Some(text) = ai_response["choices"][0]["message"]["content"].as_str() {
        // Split the text by lines into a Vec<&str>
        let lines: Vec<&str> = text.split('\n').collect();

        // Create a Series from the lines
        let series = Series::new("data", lines);

        // Create a DataFrame from the Series
        let df = DataFrame::new(vec![series])?;

        // Return the DataFrame
        Ok(df)
    } else {
        // Handle the case where the JSON does not contain the expected field
        Err("JSON response does not contain 'choices.message' field".into())
    }
}
