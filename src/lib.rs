//! # Write GitHub Actions in Rust!
//!
//! This crate provides a few convenience macros and types to make it easier to
//! write GitHub Actions in Rust. How to use:
//! 1. Annotate your `main()` function with `#[gha_main]`.
//! 2. Add return type `GitHubActionResult`.
//! 3. Use the `?` operator to propagate errors.
//! 3. Return outputs (anything that implements `Display`) wrapped in
//! `gha_output!()` to return it the action runner so that it can be used in
//! later workflow steps or other actions.
//!
//! Example usage:
//! ```no_run
//! use std::env;
//! use gha_main::{gha_main, gha_output, GitHubActionResult};
//!
//! #[gha_main]
//! fn main() -> GitHubActionResult {
//!     let args: Vec<String> = env::args().collect();
//!     let input = &args[1];
//!     let parsed_u32 = input.parse::<u32>()?;
//!     gha_output!(parsed_u32)
//! }
//! ```
//!
//! Values returned in `gha_output!()` are returned to the runner with the
//! output name equal to the Rust variable name. In the example above,
//! if the action is called with input `"5"`, the `parsed_u32` output
//! will be set to `5`.
//!
//! Errors propagated via the `?` operator are returned to the runner as the
//! `error` output. The error values are formatted using [anyhow::Error]'s
//! `Display` implementation.

pub use gha_main_proc_macro::gha_main;

/// Return type for `main()`
pub type GitHubActionResult = anyhow::Result<String>;

/// Return your computed output variables wrapped in this macro to return them
/// to the runner
///
/// Values returned in `gha_output!()` are returned to the runner with the
/// output name equal to the Rust variable name. For example:
/// ```ignore
/// let one = 1;
/// gha_output!(one) // Action output `one` set to 1
/// ```
///
/// Multiple values can be returned as well:
/// ```ignore
/// let one = 1;
/// let two = 2;
/// gha_output!(one, two) // Action output `one` set to 1, `two` set to 2
/// ```
#[macro_export]
macro_rules! gha_output {
    ($($value:ident),+) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}={}", stringify!($value), &$value.to_string()));
            )+
            Ok(v.join(","))
        }
    };
}
