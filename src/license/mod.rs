use std::{fs, io::Result, path::PathBuf};

mod generator;
use crate::cli::Options;
use generator::LicenseGen;

#[derive(Debug, Copy, Clone)]
pub enum LicenseType {
    Mit,
    Apache2,
    GPLv2,
    GPLv3,
    AGPLv3,
    LGPLv2,
    LGPLv3,
    ISC,
    BSD0,
    BSD2,
    BSD3,
    Mozilla,
    Unlicense,
    Zlib,
    Dwtfywt,
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
            LicenseType::GPLv2 => LicenseGen::gplv2(),
            LicenseType::GPLv3 => LicenseGen::gplv3(),
            LicenseType::AGPLv3 => LicenseGen::agplv3(),
            LicenseType::LGPLv2 => LicenseGen::lgplv2(),
            LicenseType::LGPLv3 => LicenseGen::lgplv3(),
            LicenseType::ISC => LicenseGen::isc(opts.name),
            LicenseType::BSD0 => LicenseGen::bsd0(opts.name),
            LicenseType::BSD2 => LicenseGen::bsd2(opts.name),
            LicenseType::BSD3 => LicenseGen::bsd3(opts.name),
            LicenseType::Mozilla => LicenseGen::mozilla(),
            LicenseType::Unlicense => LicenseGen::unlicense(),
            LicenseType::Zlib => LicenseGen::zlib(opts.name),
            LicenseType::Dwtfywt => LicenseGen::dwtfywt(),
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
