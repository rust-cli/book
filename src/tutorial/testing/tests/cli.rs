use assert_cmd::cargo::*; // Import cargo_bin_cmd! macro and methods
use predicates::prelude::*; // Used for writing assertions

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("grrs");

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

use assert_fs::prelude::*;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("grrs");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn find_content_with_cwd_in_tmp_dir() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = assert_fs::TempDir::new()?;
    let child_dir = tmp_dir.child("child_dir");
    let file = child_dir.child("sample.txt");
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    cargo_bin_cmd!("grrs")
        .current_dir(&tmp_dir)
        .arg("test")
        .arg("sample.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    cargo_bin_cmd!("grrs")
        .current_dir(&child_dir)
        .arg("test")
        .arg("sample.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
