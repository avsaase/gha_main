#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use std::fs::File;
use std::io::Write;
use syn::{parse2, parse_quote, Item, ItemFn};

/// Return type for `main()`
pub type GitHubActionResult = anyhow::Result<()>;

#[doc(hidden)]
pub fn gha_main_core(args: TokenStream, item: TokenStream) -> TokenStream {
    if !args.is_empty() {
        abort!(args, "anyinput does not take any arguments.");
    }

    let inner_main_fn = match parse2::<ItemFn>(item) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };
    let inner_main_ident = inner_main_fn.sig.ident.clone();

    if inner_main_fn.sig.ident != "main" {
        abort!(inner_main_ident, "function must be called `main`");
    }

    let lazy_statics = initialize_lazy_statics();
    let outer_main_fn = transform_main_fn(inner_main_fn, &inner_main_ident);

    quote! {
        #lazy_statics
        #outer_main_fn
    }
}

fn transform_main_fn(inner_main_fn: ItemFn, inner_main_ident: &Ident) -> ItemFn {
    parse_quote! {
        fn main() -> gha_main::anyhow::Result<()> {
            #inner_main_fn

            // If an error was propagated from the inner function, write it to the output file
            if let Err(error) = #inner_main_ident() {
                gha_output!(error);
                gha_main::anyhow::bail!("Action failed with error: {}", error);
            }

            Ok(())
        }
    }
}

fn initialize_lazy_statics() -> Item {
    parse_quote! {
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
    }
}

/// Wrap your computed output variables in this macro to return them to the
/// GitHub Action runner
///
/// Values returned in `gha_output!()` are returned to the runner with the
/// output name equal to the Rust variable name. For example:
/// ```ignore
/// let one = 1;
/// gha_output!(one); // Output `one` set to 1
/// ```
///
/// Multiple values can be returned by calling the macro multiple times.
#[macro_export]
macro_rules! gha_output {
    ($value:ident) => {
        let key = stringify!($value);
        let value = $value.to_string();
        $crate::write_output(key, value, &mut OUTPUT_FILE.lock().unwrap());
    };
}

#[doc(hidden)]
pub fn write_output(key: &str, value: String, output_file: &mut File) {
    let delimiter = uuid::Uuid::new_v4();
    std::writeln!(
        output_file,
        "{}<<{}\n{}\n{}",
        key,
        delimiter,
        value,
        delimiter,
    )
    .expect("Failed to write output");
}
