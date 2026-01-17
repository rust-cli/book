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
fn find_content_in_file_of_tmp_dir() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = assert_fs::TempDir::new()?;

    let child_dir = cwd.child("nested/child_dir");
    let child_file = child_dir.child("sample.txt");

    child_file.write_str("The first\ntest file.\nLast line of first file.")?;

    // Files can be nested several levels within the temporary directory
    assert!(child_file.path().ends_with("nested/child_dir/sample.txt"));

    cargo_bin_cmd!("grrs")
        // Execute in the temporary directory
        .current_dir(cwd.path())
        .arg("first")
        .arg(child_file.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "The first\nLast line of first file.",
        ));

    Ok(())
}
