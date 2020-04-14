use std::io::{self, Write};
use std::process;

use anyhow::Result;

use shot::run;

fn main() -> Result<()> {
    match run() {
        Ok(stdout) => {
            println!("{}", stdout);
            process::exit(0);
        }
        Err(error) => {
            let _ = writeln!(io::stderr(), "Error: {}", error);
            process::exit(1);
        }
    }
}
