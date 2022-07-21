<div align="center">
 <p><strong>insta-cmd: command line testing extension for insta</strong></p>
</div>

[![Build Status](https://github.com/mitsuhiko/insta-cmd/workflows/Tests/badge.svg?branch=main)](https://github.com/mitsuhiko/insta-cmd/actions?query=workflow%3ATests)
[![Crates.io](https://img.shields.io/crates/d/insta-cmd.svg)](https://crates.io/crates/insta-cmd)
[![License](https://img.shields.io/github/license/mitsuhiko/insta-cmd)](https://github.com/mitsuhiko/insta-cmd/blob/main/LICENSE)
[![rustc 1.56.1](https://img.shields.io/badge/rust-1.57.0%2B-orange.svg)](https://img.shields.io/badge/rust-1.57.0%2B-orange.svg)
[![Documentation](https://docs.rs/insta-cmd/badge.svg)](https://docs.rs/insta-cmd)

## Introduction

This is an experimental extension to insta for command line app testing.

```rust
use std::process::Command;
use insta_cmd::assert_cmd_snapshot;

assert_cmd_snapshot!(Command::new("echo").arg("Hello World!"));
```

## License and Links

- [Project Website](https://insta.rs/)
- [Documentation](https://docs.rs/insta-cmd/)
- [Issue Tracker](https://github.com/mitsuhiko/insta-cmd/issues)
- License: [Apache-2.0](https://github.com/mitsuhiko/insta-cmd/blob/main/LICENSE)
