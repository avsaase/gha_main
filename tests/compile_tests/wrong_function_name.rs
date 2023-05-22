use gha_main::{gha_main, output, GitHubActionResult};

#[gha_main]
fn not_main() -> GitHubActionResult {
    let value = "value";
    Ok(output!(value))
}
