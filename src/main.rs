#![allow(warnings)]

use polars::prelude::*;
// TODO: use polars::prelude::LazyCsvReader;
use clap::Parser;
use polars_ai::cli::{Cli, Commands, InputCommands};
use polars_ai::dataframe::AIDataFrame;
use polars_ai::utils::display_ai_response;
use polars_ai::utils::display_dataframe;

// TODO: the ai should spit something like this function based on the latest version of Polars
fn _analyze_data(dfs: Vec<DataFrame>) -> DataFrame {
    let df = &dfs[0];

    let top_carriers = df
        .group_by(&["Carrier"])
        .expect("Columns must exist!")
        .mean()
        .unwrap()
        .sort(["DepDelay_mean"], false, false)
        .expect("DepDelay_mean must exist")
        .head(Some(5))
        .select(&["Carrier"])
        .unwrap();

    let result_df = df
        .join(
            &top_carriers,
            &["Carrier"],
            &["Carrier"],
            JoinType::Inner.into(),
        )
        .expect("Carrier must exist")
        .sort(["DepDelay"], false, false)
        .expect("DepDelay must exist")
        .head(Some(5));

    result_df.select(&["Carrier", "DepDelay"]).unwrap()
}

#[tokio::main]
/// The entry point for the Polars AI CLI application.
///
/// # Returns
///
/// Returns `Ok(())` on success or an error if an error occurs.
async fn main() -> Result<(), Box<PolarsError>> {
    // Parse command-line arguments.
    let args = Cli::parse();
    match args.command {
        Some(Commands::Input(file)) => {
            match file.file_name {
                Some(ref file_name) => {
                    // Load a DataFrame from a CSV file.
                    let df_result = CsvReader::from_path(file_name)?.finish();
                    let df = match df_result {
                        Ok(df) => df,
                        Err(e) => {
                            eprintln!("Error loading CSV: {}", e);
                            return Err(Box::new(e));
                        }
                    };
                    match file.command {
                        InputCommands::Show(_show) => {
                            // TODO: Handle JSON display option.
                            let _ = display_dataframe(&df, false);
                        }

                        InputCommands::Ask(ask) => {
                            match ask.query {
                                Some(ref query) => {
                                    // Create a new AI DataFrame and ask a question.
                                    let mut ai_df = AIDataFrame::new(df);

                                    // Call the ask function to generate and execute code based on the query.
                                    let result = ai_df.ask(query).await.unwrap();

                                    // Display the AI response.
                                    display_ai_response(&result);
                                }
                                None => {
                                    eprintln!(
                                        "\x1b[1;91m{}\x1b[0m",
                                        "Missing 'query' argument for 'ask' command."
                                    )
                                }
                            };
                        }
                    };
                }
                None => {
                    println!("Please provide a data file name");
                }
            };
        }
        None => println!(
            "\x1b[1;91m{}\x1b[0m",
            "Unknown command. Use 'help' for usage instructions."
        ),
        None => todo!(),
    };

    Ok(())
}
