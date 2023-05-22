#[gha_main::gha_main]
fn not_main() -> Result<String, Box<dyn std::error::Error>> {
    Ok("value".to_string())
}
