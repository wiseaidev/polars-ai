//! # Polars AI 📊
//!
//! Polars AI represents a pioneering utility featuring a command-line interface (CLI) complemented by a sophisticated crate/library. It empowers you to engage in conversational interactions with your [Polars](https://github.com/pola-rs/polars) DataFrames, harnessing the capabilities of AI for data analysis. Polars AI seamlessly integrates the formidable prowess of OpenAI's GPT-3.5 Turbo, thereby augmenting and optimizing data exploration and manipulation tasks.
//!
//! Polars AI allows you to:
//!
//! 1. Chat with your Polars DataFrames using plain text queries.
//! 1. Perform data analysis tasks such as filtering, aggregating through AI-generated Rust code.
//! 1. Visualize data using charts and plots (coming soon).
//!
//! ## Installation 🚀
//!
//! To use Polars AI, you can also install it using Cargo, the Rust package manager:
//!
//! 1. Build the project using Rust's package manager, Cargo:
//!
//!    ```sh
//!    $ cargo install polars-ai
//!    ```
//!
//! 1. Run the CLI:
//!
//!    ```sh
//!    $ polars-ai help
//!    ```
//!
//! ## Getting Started 🏁
//!
//! Before you begin, make sure you have a Polars DataFrame that you want to analyze and interact with. Polars AI works with Polars DataFrames, so ensure that you have the necessary data loaded.
//!
//! ## Usage 🧑‍💻
//!
//! ### Chatting with Your DataFrames
//!
//! With Polars AI, you can chat with your DataFrames using plain text queries. Simply enter your question or query when prompted by the CLI. For example:
//!
//! ```sh
//! $ polars-ai input -f examples/datasets/flights.csv show
//! ```
//!
//! Now, based on the query above, you can run the Rust code.
//!
//! ### Data Analysis Workflow
//!
//! The generated Rust code follows a structured data analysis workflow:
//!
//! 1. **Prepare:** Preprocess and clean the data if required.
//! 1. **Process:** Manipulate the data for analysis (e.g., grouping, filtering, aggregating).
//! 1. **Analyze:** Conduct the analysis.
//! 1. **Output:** Return results in various formats.
//!
//! You can modify the generated code to customize your analysis.
//!
//! ## Examples 💡
//!
//! Refer to [the examples folder](examples) to use Polars AI to analyze your data. Polars AI will generate Rust code to perform eda on the data.
//!
//! ## Contributing 🤝
//!
//! We welcome contributions to Polars AI! If you'd like to contribute to this project, please follow these steps:
//!
//! 1. Fork the repository on GitHub:
//!    - Click the "Fork" button on the top right of the GitHub repository page.
//!
//! 1. Create a new branch for your feature or bug fix:
//!    - Use the following Git command to create a new branch:
//!
//!      ```sh
//!      $ git checkout -b feature-or-bugfix-branch
//!      ```
//!
//! 1. Make your changes and commit them:
//!    - Edit the files in your local repository and use the following Git commands to commit your changes:
//!
//!      ```sh
//!      $ git add .
//!      $ git commit -m "Your commit message here"
//!      ```
//!
//! 1. Create a pull request with a clear description of your changes:
//!    - Push your branch to your forked repository on GitHub and then create a pull request from there.
//!
//!      ```sh
//!      $ git push origin feature-or-bugfix-branch
//!      ```
//!
//!    - Visit your forked repository on GitHub, and you'll see an option to create a pull request for the branch you just pushed.
//!
//! ## License 📜
//!
//! This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

pub mod cli;
pub mod dataframe;
pub mod error;
pub mod nlp;
pub mod utils;
