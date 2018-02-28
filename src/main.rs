extern crate docopt;
extern crate media_import;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

use docopt::Docopt;
use media_import::MediaImport;

const USAGE: &'static str = "
Media Import.

Usage:
  media_import <path>
  media_import (-h | --help)

Options:
  -h --help     Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_path: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let mi = MediaImport::new();
    let path = PathBuf::from(args.arg_path);
    mi.import(&path).expect("importing failed");
}
