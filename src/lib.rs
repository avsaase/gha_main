#![doc = include_str!("../README.md")]

pub use gha_main_core::{gha_output, GitHubActionResult};
pub use gha_main_proc_macro::gha_main;

#[doc(hidden)]
pub extern crate anyhow;
#[doc(hidden)]
pub extern crate lazy_static;
