use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}


fn runs(args: &[&str], expected: &str) -> TestResult {
    let expected = fs::read_to_string(expected)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}


#[test]
fn hello1() -> TestResult {
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1_no_newline() -> TestResult {
    runs(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2() -> TestResult {
    runs(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
