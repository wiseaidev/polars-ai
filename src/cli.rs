use clap::Args;
use clap::{Parser, Subcommand};

#[derive(Parser)]
/// The top-level command-line interface for the Polars AI CLI.
///
/// # Examples
///
/// ```
/// // Create and configure the CLI.
/// let cli = Cli::parse();
///
/// // Run the CLI.
/// match cli.command {
///     Some(command) => match command {
///         // Handle subcommands here.
///         Commands::Input(input) => {
///             // Process input commands.
///         }
///     },
///     None => {
///         // Handle the absence of subcommands.
///     }
/// }
/// ```
#[command(name = "polars-ai")]
#[command(author = "Mahmoud Harmouch <oss@wiseai.dev>")]
#[command(version = "0.0.2")]
#[command(propagate_version = true)]
#[command(about = "\x1b[1;92m📊 Simple AI powered CLI for working with Polars DataFrames 📊\x1b[0m", long_about = None)]
pub struct Cli {
    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Represents available subcommands for the CLI.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Subcommand for handling input operations.
    Input(Input),
}

/// Represents input-related commands.
#[derive(Debug, Args)]
pub struct Input {
    /// Input CSV file.
    #[clap(short, long)]
    pub file_name: Option<String>,
    /// The specific input command to execute.
    #[command(subcommand)]
    pub command: InputCommands,
}

/// Represents subcommands for input-related operations.
#[derive(Subcommand, Debug)]
pub enum InputCommands {
    /// Subcommand for showing DataFrame options.
    Show(OptionValue),
    /// Subcommand for asking AI-related queries.
    Ask(Query),
}

/// Represents options for displaying DataFrames.
#[derive(Debug, Args)]
pub struct OptionValue {
    /// Display the DataFrame in JSON or DataFrame format.
    #[clap(short, long)]
    pub option: Option<String>,
}

/// Represents a query to be asked to the AI.
#[derive(Debug, Args)]
pub struct Query {
    /// The question or query to ask to the AI.
    #[clap(short, long)]
    pub query: Option<String>,
}
