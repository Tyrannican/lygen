# LyGen - Software License Generator

Tired of constantly copy/pasting software licenses?

This will let you create some of the most common ones directly!

## Supported Licenses

This is the list of supported licenses:

* MIT License

* Apache License 2.0

* GNU General Public License 2.0 (GPLv2)

* GNU General Public License 3.0 (GPLv3)

* GNU Lesser General Public License v2.1 (LGPLv2)

* GNU Lesser General Public License v3.0 (LGPLv3)

* GNU Affero General Public License v3.0 (AGPLv3)

* ISC License

* BSD Zero Clause License

* BSD 2-Clause "Simplified" License

* BSD 3-Clause "New" or "Revised" License

* Mozilla Public License 2.0

* The Unlicense

* zlib License

* Do What The F*ck You Want To Public License

## Installation

You can install via Cargo or download from the Releases tab for your platform:

```bash
cargo install lygen
```

## Usage

```
Common License Generator to create LICENSE files

Usage: lygen <COMMAND>

Commands:
  mit        MIT License
  apache2    Apache License 2.0
  gplv2      GNU General Public License v2.0
  gplv3      GNU General Public License v3.0
  lgplv2     GNU Lesser General Public License v2.1
  lgplv3     GNU Lesser General Public License v3.0
  agplv3     GNU Affero General Public License v3.0
  isc        ISC License
  bsd0       BSD Zero Clause License
  bsd2       BSD 2-Clause "Simplified" License
  bsd3       BSD 3-Clause "New" or "Revised" License
  mozilla    Mozilla Public License 2.0
  unlicense  The Unlicense
  zlib       zlib License
  dwtfywt    Do What The F*ck You Want To Public License
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Subcommands

Each license shared the same subcommands. For example:

```
MIT License

Usage: lygen mit [OPTIONS]

Options:
  -o, --output-dir <OUTPUT_DIR>  Output directory for the License. Defaults to the working directory
  -n, --name <NAME>              Creator name to use within the License (if applicable)
  -f, --filename <FILENAME>      Filename for the License. Defaults to LICENSE [default: LICENSE]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Legal Notice

These Licenses are provided "as is" from their relevant sources with zero modification.

Double check the license you want to use is appropriate for your use-case.
