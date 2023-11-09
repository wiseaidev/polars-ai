use polars::prelude::*;
// TODO: use regex::Regex;
use std::error::Error;
// TODO: use eval::eval;
use serde::Serialize;

// Custom struct for serializing the DataFrame
#[derive(Serialize)]
struct SerializedDataFrame {
    rows: usize,
    columns: usize,
}

/// Displays a DataFrame with an option to output in JSON format.
///
/// # Arguments
///
/// * `df` - The DataFrame to display.
/// * `json_format` - Indicates whether to display the DataFrame in JSON format.
///
/// # Returns
///
/// Returns `Ok(())` on success or an error if an error occurs.
pub fn display_dataframe(df: &DataFrame, json_format: bool) -> Result<(), Box<dyn Error>> {
    if json_format {
        // Serialize the DataFrame to a custom struct and then to JSON.
        let serialized_df = SerializedDataFrame {
            rows: df.height(),
            columns: df.width(),
        };
        let json_str = serde_json::to_string_pretty(&serialized_df)?;
        println!("{}", json_str);
    } else {
        println!("\x1b[1;92m{}\x1b[0m", "📊 DataFrame:");
        println!("\x1b[1;94m{:?}\x1b[0m", df);
    }
    Ok(())
}

/// Displays an AI response.
///
/// # Arguments
///
/// * `response` - The AI response to display.
pub fn display_ai_response(response: &str) {
    println!("\n🤖 AI Response:\n");
    println!("{}", response);
    // TODO: execute the code block
    // Iterate over matches and execute the code blocks
    // for code_block in re.captures_iter(&net_ai_response) {
    //     println!("{:?}", code_block);
    //     let rust_code = code_block.get(1).unwrap().as_str();
    //     match eval(&rust_code) {
    //         Ok(_) => println!("Successfully executed Rust code block:\n{}", rust_code),
    //         Err(e) => eprintln!("Error executing Rust code block:\n{}", e),
    //     }
    // }
}
