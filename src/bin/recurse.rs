use std::io::{self, Write};
use std::process;

use anyhow::Result;

use recurse::run;

fn main() -> Result<()> {
    match run() {
        Ok(_) => {
            process::exit(0);
        }
        Err(error) => {
            let _ = writeln!(io::stderr(), "Error: {}", error);
            process::exit(1);
        }
    }
}
