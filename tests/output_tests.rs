use std::{env::set_var, fs::remove_file};

use gha_main::{gha_main, gha_output, GitHubActionResult};
use temp_env::with_var;
use uuid::Uuid;

#[test]
fn result_ok() {
    let output_file = output_file();

    with_var("GITHUB_OUTPUT", Some(&output_file), || {
        #[gha_main]
        fn main() -> GitHubActionResult {
            let output = "Success!";
            Ok(gha_output!(output))
        }

        main();

        assert_output("output=Success!", &output_file);
    });
}

#[test]
fn result_error() {
    let output_file = output_file();

    with_var("GITHUB_OUTPUT", Some(&output_file), || {
        #[gha_main]
        fn main() -> GitHubActionResult {
            let input = "one";
            let parsed = input.parse::<u8>()?;
            Ok(gha_output!(parsed))
        }

        main();

        assert_output("error=invalid digit found in string", &output_file);
    });
}

fn output_file() -> String {
    let output_file = format!("tests/github_output-{}", Uuid::new_v4().to_string());
    set_var("GITHUB_OUTPUT", &output_file);
    output_file
}

fn assert_output(value: &str, output_file: &str) {
    let output_file_contents = std::fs::read_to_string(output_file).unwrap();
    remove_file(output_file).expect("Test output file could no be deleted");
    assert_eq!(output_file_contents, value);
}
