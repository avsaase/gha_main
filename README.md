# gha-main

## Write GitHub Actions in Rust!

This crate provides two convenience macros to make it easier to write GitHub Actions in Rust.

How to use:
1. Annotate your `main()` function with `#[gha_main]`.
2. Add return type `GitHubActionResult`.
3. Use the `?` operator to propagate errors.
4. Return ouputs (anything that implements `Display`) to the action runner
with the `gha_output!()` macro so they can be used in later workflow steps
or other actions.

Example usage:
```rust,no_run
use std::env;
use gha_main::{gha_main, gha_output, GitHubActionResult};

#[gha_main]
fn main() -> GitHubActionResult {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let parsed_u32 = input.parse::<u32>()?;
    gha_output!(parsed_u32);
    Ok(())
}
```

Values wrapped in `gha_output!()` are returned to the runner with the output name equal to the Rust variable name. In the example above, if the action is called with input `"5"`, the `parsed_u32` output will be set to `5`.

Errors propagated via the `?` operator are returned to the runner as the `error` output. The error values are formatted using [anyhow::Error]'s `Display` implementation.

## Example actions
The [/example-actions](https://github.com/avsaase/gha_main/tree/master/example-actions) folder contains several examples that demonstrate how GitHub Actions can be written in Rust.

License: Apache-2.0 OR MIT
