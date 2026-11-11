//! Tests for the `yulia` driver.
//!
//! Every branch of `src/main.rs` is exercised: the flag paths, the input-path
//! path, and every failure path. The driver's behaviour is pinned here so a
//! later change cannot silently alter the CLI contract.

mod harness;
mod input;

use harness::{USAGE, run, stdout};

#[test]
fn no_arguments_prints_usage_and_succeeds() {
    let output = run(&[] as &[&std::ffi::OsStr]).expect("spawn yulia");
    assert!(output.status.success());
    assert_eq!(stdout(&output), USAGE);
}

#[test]
fn help_flag_prints_usage_and_succeeds() {
    let output = run(&["--help"]).expect("spawn yulia");
    assert!(output.status.success());
    assert_eq!(stdout(&output), USAGE);
}

#[test]
fn short_help_flag_matches_long() {
    let short = run(&["-h"]).expect("spawn yulia");
    let long = run(&["--help"]).expect("spawn yulia");
    assert_eq!(stdout(&short), stdout(&long));
}

#[test]
fn version_flag_reports_the_crate_version() {
    let output = run(&["--version"]).expect("spawn yulia");
    assert!(output.status.success());
    assert_eq!(
        stdout(&output),
        format!("yulia {} (license: MPL-2.0)\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn short_version_flag_matches_long() {
    let short = run(&["-V"]).expect("spawn yulia");
    let long = run(&["--version"]).expect("spawn yulia");
    assert_eq!(stdout(&short), stdout(&long));
}

#[test]
fn unknown_option_fails_with_a_hint() {
    let output = run(&["--definitely-not-a-flag"]).expect("spawn yulia");
    assert!(!output.status.success());
    let err = harness::stderr(&output);
    assert!(err.contains("unknown option"), "stderr: {err}");
    assert!(err.contains("try 'yulia --help'"), "stderr: {err}");
}
