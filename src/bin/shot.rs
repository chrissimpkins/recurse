use std::io::{self, Write};
use std::process;

use anyhow::Result;

use shot::run;

fn main() -> Result<()> {
    if let Err(error) = run() {
        let _ = writeln!(io::stderr(), "Error: {}", error);
        process::exit(1);
    }
    process::exit(0);
}
