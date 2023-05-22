use std::env;
extern crate gha_main;
use gha_main::{gha_main, gha_result, GitHubActionResult};
use ureq::serde_json;

#[gha_main]
fn main() -> GitHubActionResult {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    println!("Making GET request to {}", &url);
    let response = ureq::get(url).call()?;
    let json: serde_json::Value = response.into_json::<serde_json::Value>()?;

    Ok(gha_result!(json))
}
