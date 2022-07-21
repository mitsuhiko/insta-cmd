//! `insta-cmd` is an extension to [insta](https://insta.rs/) that lets you snapshot
//! a command that produces (text) output to stdout and stderr.  It takes a
//! [`Command`](std::process::Command) from the standard library, runs it and
//! snapshots the output alongside the exit code.
//!
//! ```no_run
//! use std::process::Command;
//! use insta_cmd::assert_cmd_snapshot;
//!
//! assert_cmd_snapshot!(Command::new("echo").arg("Hello World!"));
//! ```
//!
//! ## Testing Binaries
//!
//! If you want to test binaries from your own project you can use the
//! [`get_cargo_bin`] and [`get_cargo_example`] functions to retrieve the path to
//! your binary.  Note that it's unlikely that cargo will have built the binary
//! under normal circumstances so you will have to run ``cargo build --bin my-bin``
//! or ``cargo build --example my-example`` before.
//!
//! Afterwards you can test it like this:
//!
//! ```no_run
//! use std::process::Command;
//! use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
//!
//! assert_cmd_snapshot!(Command::new(get_cargo_bin("hello")).arg("first arg"));
//! ```
#[doc(hidden)]
#[macro_use]
mod macros;

mod cargo;
mod spawn;

pub use crate::cargo::{get_cargo_bin, get_cargo_example};
pub use crate::spawn::{Spawn, StdinCommand};

pub use std::process::Command;

#[doc(hidden)]
pub mod _macro_support {
    pub use super::spawn::Spawn;
    pub use insta;
}

#[test]
fn test_basic() {
    assert_cmd_snapshot!(["/bin/echo", "Hello World"]);
}

#[test]
fn test_command() {
    assert_cmd_snapshot!(Command::new("echo").arg("Just some stuff"));
}

#[test]
fn test_stdin() {
    assert_cmd_snapshot!(StdinCommand::new("cat", "Hello World!"));
}

#[test]
fn test_failure() {
    assert_cmd_snapshot!(["false"]);
}
