use assert_cmd::Command;

#[test]
fn single_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("machine-communication-stdin")?;

    cmd.arg("haiku.txt");
    cmd.assert().success().stdout(String::from("7\n"));

    Ok(())
}

#[test]
fn stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("machine-communication-stdin")?;

    cmd.arg("-")
        .write_stdin("Hello, world! My first name is Standard, my last name is In.\n");
    cmd.assert().success().stdout(String::from("12\n"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("machine-communication-stdin")?;

    cmd.arg("tests/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"))

    Ok(())
}
