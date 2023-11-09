# Polars AI 📊

[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Jupyter Notebook](https://img.shields.io/badge/Jupyter-Notebook-blue.svg?logo=Jupyter&logoColor=orange)](https://jupyter.org/)
[![Share On Reddit](https://img.shields.io/badge/share%20on-reddit-red?style=flat-square&logo=reddit)](https://reddit.com/submit?url=https://github.com/wiseaidev/polars-ai&amp;title=A%20CLI%20and%20a%20library%20for%20interacting%20with%20Polars%20DataFrames%20using%20natural%20language%20queries%20and%20AI.)
[![Share On Ycombinator](https://img.shields.io/badge/share%20on-hacker%20news-orange?style=flat-square&logo=ycombinator&amp;w=256&amp;q=751x)](https://news.ycombinator.com/submitlink?u=https://github.com/wiseaidev/polars-ai&amp;t=A%20CLI%20and%20a%20library%20for%20interacting%20with%20Polars%20DataFrames%20using%20natural%20language%20queries%20and%20AI.)
[![Share On Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?style=flat-square&logo=twitter&amp;w=128&amp;q=751x)](https://twitter.com/share?url=https://github.com/wiseaidev/polars-ai&amp;text=A%20CLI%20and%20a%20library%20for%20interacting%20with%20Polars%20DataFrames%20using%20natural%20language%20queries%20and%20AI.)
[![Share On Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?style=flat-square&logo=facebook&amp;w=256&amp;q=751x)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/wiseaidev/polars-ai)
[![Share On Linkedin](https://img.shields.io/badge/share%20on-linkedin-3949AB?style=flat-square&logo=linkedin&amp;w=256&amp;q=751x)](https://www.linkedin.com/shareArticle?url=https://github.com/wiseaidev/polars-ai&amp;title=A%20CLI%20and%20a%20library%20for%20interacting%20with%20Polars%20DataFrames%20using%20natural%20language%20queries%20and%20AI.)

Polars AI represents a pioneering utility featuring a command-line interface (CLI) complemented by a sophisticated crate/library. It empowers you to engage in conversational interactions with your [Polars](https://github.com/pola-rs/polars) DataFrames, harnessing the capabilities of AI for data analysis. Polars AI seamlessly integrates the formidable prowess of OpenAI's GPT-3.5 Turbo, thereby augmenting and optimizing data exploration and manipulation tasks.

Polars AI allows you to:

1. Chat with your Polars DataFrames using plain text queries.
1. Perform data analysis tasks such as filtering, aggregating through AI-generated Rust code.
1. Visualize data using charts and plots (coming soon).

## Table of Contents 📚

- [Installation](#installation) 🚀
- [Getting Started](#getting-started) 🏁
- [Usage](#usage) 🧑‍💻
- [Examples](#examples) 💡
- [Contributing](#contributing) 🤝
- [License](#license) 📜

## Installation 🚀

### Install from source

To use Polars AI, you'll need to follow these installation steps:

1. Install Rust (if not already installed) by following the instructions at [Rust Install](https://www.rust-lang.org/tools/install).

1. Fork the repository on GitHub:
   - Click the "Fork" button on the top right of the GitHub repository page.

1. Clone the Polars AI repository to your local machine:

   ```sh
   $ git clone https://github.com/yourusername/polars-ai.git
   ```

1. Build the project using Rust's package manager, Cargo:

   ```sh
   $ cd polars-ai
   $ cargo build --release
   ```

1. Run the OpenAI API key:

   ```sh
   $ export OPENAI_API_KEY=sk-
   ```

1. Run the CLI:

   ```sh
   $ ./target/release/polars-ai help
   ```

Sure, here's the section split into two subsections titled "Install from source" and "Install using Cargo":

### Install using Cargo

To use Polars AI, you can also install it using Cargo, the Rust package manager:

1. Build the project using Rust's package manager, Cargo:

   ```sh
   $ cargo install polars-ai
   ```

1. Run the OpenAI API key:

   ```sh
   $ export OPENAI_API_KEY=sk-
   ```

1. Run the CLI:

   ```sh
   $ polars-ai help
   ```

## Getting Started 🏁

Before you begin, make sure you have a Polars DataFrame that you want to analyze and interact with. Polars AI works with Polars DataFrames, so ensure that you have the necessary data loaded.

## Usage 🧑‍💻

### Chatting with Your DataFrames

With Polars AI, you can chat with your DataFrames using plain text queries. Simply enter your question or query when prompted by the CLI. For example:

```sh
$ export OPENAI_API_KEY=sk-

$ polars-ai input -f examples/datasets/flights.csv show
📊 DataFrame:
shape: (18, 7)
┌────────────┬───────────┬─────────┬─────────────────┬───────────────┬──────────┬──────────┐
│ DayofMonth ┆ DayOfWeek ┆ Carrier ┆ OriginAirportID ┆ DestAirportID ┆ DepDelay ┆ ArrDelay │
│ ---        ┆ ---       ┆ ---     ┆ ---             ┆ ---           ┆ ---      ┆ ---      │
│ i64        ┆ i64       ┆ str     ┆ i64             ┆ i64           ┆ i64      ┆ i64      │
╞════════════╪═══════════╪═════════╪═════════════════╪═══════════════╪══════════╪══════════╡
│ 19         ┆ 5         ┆ DL      ┆ 11433           ┆ 13303         ┆ -3       ┆ 1        │
│ 19         ┆ 5         ┆ DL      ┆ 14869           ┆ 12478         ┆ 0        ┆ -8       │
│ 19         ┆ 5         ┆ DL      ┆ 14057           ┆ 14869         ┆ -4       ┆ -15      │
│ 19         ┆ 5         ┆ DL      ┆ 15016           ┆ 11433         ┆ 28       ┆ 24       │
│ …          ┆ …         ┆ …       ┆ …               ┆ …             ┆ …        ┆ …        │
│ 19         ┆ 5         ┆ DL      ┆ 10397           ┆ 12451         ┆ 71       ┆ null     │
│ 19         ┆ 5         ┆ DL      ┆ 12451           ┆ 10397         ┆ 75       ┆ null     │
│ 19         ┆ 5         ┆ DL      ┆ 12953           ┆ 10397         ┆ -1       ┆ null     │
│ 19         ┆ 5         ┆ DL      ┆ 11433           ┆ 12953         ┆ -3       ┆ null     │
└────────────┴───────────┴─────────┴─────────────────┴───────────────┴──────────┴──────────┘

$ polars-ai input -f examples/datasets/flights.csv ask -q 'What is the average of the first column?'

🤖 AI Response:

use polars::prelude::*;

fn analyze_data(dfs: Vec<DataFrame>) -> Result<DataFrame> {
    let df = &dfs[0];

    let avg_first_column = df
        .select(&[col("DayofMonth")])
        .expect("Column 'DayofMonth' must exist")
        .mean()
        .unwrap()
        .select(&[col("mean")])
        .unwrap();

    let top_carriers = df
        .groupby(&[col("Carrier")])
        .expect("Column 'Carrier' must exist")
        .mean()
        .unwrap()
        .sort(&[col("mean")], false)
        .expect("Column 'mean' must exist")
        .head(Some(5))
        .select(&[col("Carrier")])
        .unwrap();

    let result_df = df
        .join(&top_carriers, &[col("Carrier")], &[col("Carrier")], JoinType::Inner)
        .expect("Column 'Carrier' must exist")
        .sort(&[col("DayofMonth")], false)
        .expect("Column 'DayofMonth' must exist")
        .head(Some(5));

    let final_result = result_df
        .select(&[col("Carrier"), col("DayofMonth")])
        .unwrap();

    Ok(final_result)
}

let result = analyze_data(dfs);
println!("{}", result);
```

Now, based on the query above, you can run the Rust code.

### Data Analysis Workflow

The generated Rust code follows a structured data analysis workflow:

1. **Prepare:** Preprocess and clean the data if required.
1. **Process:** Manipulate the data for analysis (e.g., grouping, filtering, aggregating).
1. **Analyze:** Conduct the analysis.
1. **Output:** Return results in various formats.

You can modify the generated code to customize your analysis.

## Examples 💡

Refer to [the examples folder](examples) to use Polars AI to analyze your data. Polars AI will generate Rust code to perform eda on the data.

## Contributing 🤝

We welcome contributions to Polars AI! If you'd like to contribute to this project, please follow these steps:

1. Fork the repository on GitHub:
   - Click the "Fork" button on the top right of the GitHub repository page.

1. Create a new branch for your feature or bug fix:
   - Use the following Git command to create a new branch:

     ```sh
     $ git checkout -b feature-or-bugfix-branch
     ```

1. Make your changes and commit them:
   - Edit the files in your local repository and use the following Git commands to commit your changes:

     ```sh
     $ git add .
     $ git commit -m "Your commit message here"
     ```

1. Create a pull request with a clear description of your changes:
   - Push your branch to your forked repository on GitHub and then create a pull request from there.

     ```sh
     $ git push origin feature-or-bugfix-branch
     ```
   - Visit your forked repository on GitHub, and you'll see an option to create a pull request for the branch you just pushed.

## License 📜

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
