use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

const BIN_NAME: &'static str = "main";

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd
        .arg("foobar").arg("test/file/doesnt/exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<()> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd
        .arg("test").arg(file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}