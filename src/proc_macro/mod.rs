use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

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
pub fn gha_main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let ident = &input_fn.sig.ident;

    verify_main(ident);

    TokenStream::from(quote! {
        gha_main::lazy_static::lazy_static! {
            static ref OUTPUT: String =
                std::env::var("GITHUB_OUTPUT").unwrap_or("github_output".to_string());
            static ref OUTPUT_FILE: std::sync::Mutex<std::fs::File> =
                std::sync::Mutex::new(std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&*OUTPUT)
                    .expect("Failed to create or open output file"));
        }

        fn main() -> gha_main::anyhow::Result<()> {
            #input_fn

            // If an error was propagated from the inner function, write it to the output file
            if let Err(error) = #ident() {
                gha_output!(error);
                gha_main::anyhow::bail!("Action failed with error: {}", error);
            }

            Ok(())
        }
    })
}

fn verify_main(ident: &Ident) {
    if ident != "main" {
        abort!(ident, "function must be called `main`");
    }
}
