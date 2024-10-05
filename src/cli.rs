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

    /// Filename for the License. Defaults to LICENSE
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

    /// GNU Lesser General Public License v2.1
    Lgplv2 {
        #[command(flatten)]
        options: Options,
    },

    /// GNU Lesser General Public License v3.0
    Lgplv3 {
        #[command(flatten)]
        options: Options,
    },

    /// GNU Affero General Public License v3.0
    Agplv3 {
        #[command(flatten)]
        options: Options,
    },

    /// ISC License
    Isc {
        #[command(flatten)]
        options: Options,
    },

    /// BSD Zero Clause License
    Bsd0 {
        #[command(flatten)]
        options: Options,
    },

    /// BSD 2-Clause "Simplified" License
    Bsd2 {
        #[command(flatten)]
        options: Options,
    },

    /// BSD 3-Clause "New" or "Revised" License
    Bsd3 {
        #[command(flatten)]
        options: Options,
    },

    /// Mozilla Public License 2.0
    Mozilla {
        #[command(flatten)]
        options: Options,
    },

    /// The Unlicense
    Unlicense {
        #[command(flatten)]
        options: Options,
    },

    /// zlib License
    Zlib {
        #[command(flatten)]
        options: Options,
    },

    /// Do What The F*ck You Want To Public License
    Dwtfywt {
        #[command(flatten)]
        options: Options,
    },
}
