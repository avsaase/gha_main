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
///     gha_result!(parsed)
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn gha_main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let ident = &input_fn.sig.ident;

    verify_main(ident);

    TokenStream::from(quote! {
        fn main() {
            #input_fn

            let output = std::env::var("GITHUB_OUTPUT").unwrap_or("github_output".to_string());

            use std::fs::write;
            match #ident() {
                Ok(value) => write(output, value.to_string()).unwrap(),
                Err(error) => {
                    write(output, format!("error={}", error)).unwrap();
                    eprintln!("Action failed with error: {}", error);
                },
            }
        }
    })
}

fn verify_main(ident: &Ident) {
    if ident != "main" {
        abort!(ident, "function must be called `main`");
    }
}
