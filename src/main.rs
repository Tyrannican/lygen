mod cli;
mod license;

use clap::Parser;
use std::io::Result;

use cli::{Cli, LygenCommand};
use license::{License, LicenseType};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let license = match cli.commands {
        LygenCommand::Mit { options } => License::new(options, LicenseType::Mit),
        LygenCommand::Apache2 { options } => License::new(options, LicenseType::Apache2),
        LygenCommand::Gplv2 { options } => License::new(options, LicenseType::GPLv2),
        LygenCommand::Gplv3 { options } => License::new(options, LicenseType::GPLv3),
    };

    if let Err(e) = license.write() {
        eprintln!("error writing license to disk - {e}");
    }

    Ok(())
}
