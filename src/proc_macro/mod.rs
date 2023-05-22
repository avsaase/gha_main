use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::{abort, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn, ReturnType, Type};

/// Add this attribute macro on your `main` function and change the return type
/// to `gha_main::GitHubActionResult`. You can then use the `?` operator for
/// conveniet error handling in your Action code.
///
/// The macro takes care of writing the result to the file referenced in
/// `$GITHUB_OUPUT` (if this environment variable is set by the GitHub Action
/// runner) or to file `github_output`.
#[proc_macro_attribute]
#[proc_macro_error]
pub fn gha_main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let ident = &input_fn.sig.ident;

    verify_main(ident);
    // verify_return_type(&input_fn.sig.output);

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
                    std::process::exit(1);
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

fn verify_return_type(return_type: &ReturnType) {
    let correct_return_type = if let ReturnType::Type(_, ty) = return_type {
        if let Type::Path(type_path) = *ty.clone() {
            type_path.into_token_stream().to_string() == "anyhow::Result<gha_main::GitHubOutputs>"
        } else {
            false
        }
    } else {
        false
    };

    if !correct_return_type {
        abort!(
            return_type,
            "Return type must be `gha_main::GitHubActionResult`"
        );
    }
}
