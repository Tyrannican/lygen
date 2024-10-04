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
        LygenCommand::Agplv3 { options } => License::new(options, LicenseType::AGPLv3),
        LygenCommand::Lgplv2 { options } => License::new(options, LicenseType::LGPLv2),
        LygenCommand::Lgplv3 { options } => License::new(options, LicenseType::LGPLv3),
        LygenCommand::Isc { options } => License::new(options, LicenseType::ISC),
        LygenCommand::Bsd0 { options } => License::new(options, LicenseType::BSD0),
        LygenCommand::Bsd2 { options } => License::new(options, LicenseType::BSD2),
        LygenCommand::Bsd3 { options } => License::new(options, LicenseType::BSD3),
        LygenCommand::Mozilla { options } => License::new(options, LicenseType::Mozilla),
        LygenCommand::Unlicense { options } => License::new(options, LicenseType::Unlicense),
        LygenCommand::Zlib { options } => License::new(options, LicenseType::Zlib),
        LygenCommand::Dwtfywt { options } => License::new(options, LicenseType::Dwtfywt),
    };

    if let Err(e) = license.write() {
        eprintln!("error writing license to disk - {e}");
    }

    Ok(())
}
