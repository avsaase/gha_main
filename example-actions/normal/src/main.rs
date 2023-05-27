use std::env;

use gha_main::{gha_main, gha_output, GitHubActionResult};

#[gha_main]
fn main() -> GitHubActionResult {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let parsed_u32 = input.parse::<u32>()?;
    let multiline_string = "This is a\nmultiline text";

    gha_output!(parsed_u32);
    gha_output!(multiline_string);
    Ok(())
}
