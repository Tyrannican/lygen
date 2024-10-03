use std::{fs, io::Result, path::PathBuf};

mod generator;
use crate::cli::Options;
use generator::LicenseGen;

#[derive(Debug, Copy, Clone)]
pub enum LicenseType {
    Mit,
    Apache2,
}

#[derive(Debug)]
pub struct License {
    contents: String,
    output: PathBuf,
}

impl License {
    pub fn new(opts: Options, license: LicenseType) -> Self {
        let contents = match license {
            LicenseType::Mit => LicenseGen::mit(opts.name),
            LicenseType::Apache2 => LicenseGen::apache2(),
        };

        let out_dir = match opts.output_dir {
            Some(od) => PathBuf::from(od),
            None => PathBuf::from("."),
        };

        let output = out_dir.join(opts.filename);

        Self { contents, output }
    }

    pub fn write(self) -> Result<()> {
        fs::write(self.output, self.contents)
    }
}
