use std::env;
extern crate gha_main;
use gha_main::{gha_main, gha_result, GitHubActionResult};

#[gha_main]
fn main() -> GitHubActionResult {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let parsed_u32 = input.parse::<u32>()?;

    Ok(gha_result!(parsed_u32))
}
