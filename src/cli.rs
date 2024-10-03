use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(about = "Common License Generator to create LICENSE files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: LygenCommand,
}

#[derive(Parser, Debug)]
pub struct Options {
    /// Output directory for the License. Defaults to the working directory
    #[arg(short, long)]
    pub output_dir: Option<String>,

    /// Creator name to use within the License (if applicable)
    #[arg(short, long)]
    pub name: Option<String>,

    /// Name to call the License. Defaults to LICENSE
    #[arg(short, long, default_value_t = String::from("LICENSE"))]
    pub filename: String,
}

#[derive(Subcommand, Debug)]
pub enum LygenCommand {
    /// MIT License
    Mit {
        #[command(flatten)]
        options: Options,
    },

    /// Apache License 2.0
    Apache2 {
        #[command(flatten)]
        options: Options,
    },

    /// GNU General Public License v2.0
    Gplv2 {
        #[command(flatten)]
        options: Options,
    },

    /// GNU General Public License v3.0
    Gplv3 {
        #[command(flatten)]
        options: Options,
    },
}
