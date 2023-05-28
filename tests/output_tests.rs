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
            gha_output!(output);
            Ok(())
        }

        let res = main();

        assert!(res.is_ok());
        // assert_output("output=Success!\n", &output_file);
        assert_output("output<<.*\nSuccess!\n.*\n", &output_file);
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
            gha_output!(parsed);
            Ok(())
        }

        let res = main();

        assert!(res.is_err());
        assert_output(
            "error<<.*\ninvalid digit found in string\n.*\n",
            &output_file,
        );
    });
}

#[test]
fn multiple_outputs() {
    let output_file = output_file();

    with_var("GITHUB_OUTPUT", Some(&output_file), || {
        #[gha_main]
        fn main() -> GitHubActionResult {
            let one = 1;
            let two = 2;
            gha_output!(one);
            gha_output!(two);
            Ok(())
        }

        let res = main();

        assert!(res.is_ok());
        assert_output("one<<.*\n1\n.*\ntwo<<.*\n2\n.*\n", &output_file);
    });
}

#[test]
fn multiline_output() {
    let output_file = output_file();

    with_var("GITHUB_OUTPUT", Some(&output_file), || {
        #[gha_main]
        fn main() -> GitHubActionResult {
            let multiline_string = "This is a\nmultiline string";
            gha_output!(multiline_string);
            Ok(())
        }

        let res = main();

        assert!(res.is_ok());
        assert_output(
            "multiline_string<<.*\nThis is a\nmultiline string\n.*\n",
            &output_file,
        );
    });
}

fn output_file() -> String {
    let output_file = format!("tests/github_output-{}", Uuid::new_v4().to_string());
    set_var("GITHUB_OUTPUT", &output_file);
    output_file
}

fn assert_output(output_regex: &str, output_file: &str) {
    let regex = regex::Regex::new(output_regex).unwrap();
    let output_file_contents = std::fs::read_to_string(output_file).unwrap();
    remove_file(output_file).expect("Test output file could no be deleted");
    assert!(regex.is_match(&output_file_contents));
}
