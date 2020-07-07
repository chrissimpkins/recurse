use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

// =================================
// Walk subcommand integration tests
// =================================

#[test]
fn walk_inpath_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("recurse")?;

    cmd.arg("walk").arg("testfiles/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("no such file or directory"))
        .code(1);

    Ok(())
}
