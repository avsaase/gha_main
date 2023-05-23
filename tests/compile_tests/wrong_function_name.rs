use gha_main::{gha_main, gha_output, GitHubActionResult};

#[gha_main]
fn not_main() -> GitHubActionResult {
    let value = "value";
    Ok(gha_output!(value))
}
