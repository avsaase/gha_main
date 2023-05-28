#![doc = include_str!("../README.md")]

use gha_main_core::gha_main_core;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

/// Add this macro to your `main()` function to return outputs and errors to
/// the action runner
///
/// Example usage:
/// ```rust
/// use gha_main::{gha_main, gha_result, GitHubActionResult};
///
/// #[gha_main]
/// fn main() -> GitHubActionResult {
///     let parsed = "5".parse::<i32>()?;
///     gha_result!(parsed);
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn gha_main(args: TokenStream, item: TokenStream) -> TokenStream {
    gha_main_core(args.into(), item.into()).into()
}
