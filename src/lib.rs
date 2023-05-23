//! # Write GitHub Actions in Rust!
//!
//! This crate provides a few convenience macros and types to make it easier to
//! write GitHub Actions in Rust. How to use:
//! 1. Annotate your `main()` function with `#[gha_main]`.
//! 2. Add return type `GitHubActionResult`.
//! 3. Use the `?` operator to propagate errors.
//! 3. Return output variables (anything that implements `Display`) wrapped in
//! `gha_output!()` to return it the action runner to be used in later workflow
//! steps or other actions.
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
//! output variable name equal to the Rust variable name. In the example
//! above, if the action is called with input "5", the `parsed_u32` output
//! variable will be set to 5.
//!
//! Errors propagated via the `?` operator are returned to the runner as the
//! `error` output variable. The error values are formatted using
//! [anyhow::Error]'s `Display` implementation.

// use std::fmt::Display;

pub use gha_main_proc_macro::gha_main;

/// Return type for `main()`
// pub type GitHubActionResult = anyhow::Result<GitHubOutputs>;
pub type GitHubActionResult = anyhow::Result<String>;

// /// Construct via [gha_output!].
// pub struct GitHubOutput {
//     key: String,
//     value: String,
// }

// impl Display for GitHubOutput {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}={}", self.key, self.value)
//     }
// }

// impl GitHubOutput {
//     /// Create a new [GitHubOutput].
//     pub fn new(key: &str, value: impl Display) -> Self {
//         Self {
//             key: key.to_string(),
//             value: value.to_string(),
//         }
//     }
// }

// /// Construct via [gha_output!].
// pub struct GitHubOutputs {
//     outputs: Vec<GitHubOutput>,
// }

// impl GitHubOutputs {
//     /// Create new [GitHubOutputs].
//     pub fn multiple(outputs: Vec<GitHubOutput>) -> Self {
//         Self { outputs }
//     }
// }

// impl Display for GitHubOutputs {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let outputs = self
//             .outputs
//             .iter()
//             .map(|o| o.to_string())
//             .collect::<Vec<_>>();
//         write!(f, "{}", outputs.join(","))
//     }
// }

// impl From<GitHubOutputs> for GitHubActionResult {
//     fn from(value: GitHubOutputs) -> Self {
//         Ok(value)
//     }
// }

/// Return your computed output variables wrapped in this macro to return them
/// to the runner
///
///
#[macro_export]
macro_rules! gha_output {
    ($($value:ident),+) => {
        {
            // let mut v = Vec::new();
            // $(
            //     v.push(gha_main::GitHubOutput::new(stringify!($value), &$value.to_string()));
            // )+
            // gha_main::GitHubOutputs::multiple(v).into()
            let mut v = Vec::new();
            $(
                v.push(format!("{}={}", stringify!($value), &$value.to_string()));
            )+
            Ok(v.join(","))
        }
    };
}
