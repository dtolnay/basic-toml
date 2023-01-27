//! A [TOML]-parsing library
//!
//! This library implements a [TOML] v0.5.0 compatible parser,
//! primarily supporting the [`serde`] library for encoding/decoding
//! various types in Rust.
//!
//! TOML itself is a simple, ergonomic, and readable configuration format:
//!
//! ```toml
//! [package]
//! name = "basic-toml"
//! version = "0.0.0"
//! authors = ["Alex Crichton <alex@alexcrichton.com>"]
//!
//! [dependencies]
//! serde = "1.0"
//! ```
//!
//! The TOML format tends to be relatively common throughout the Rust community
//! for configuration, notably being used by [Cargo], Rust's package manager.
//!
//! ## Deserialization and Serialization
//!
//! An example of deserializing with TOML is:
//!
//! ```rust
//! use serde_derive::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Config {
//!     ip: String,
//!     port: Option<u16>,
//!     keys: Keys,
//! }
//!
//! #[derive(Deserialize)]
//! struct Keys {
//!     github: String,
//!     travis: Option<String>,
//! }
//!
//! fn main() {
//!     let config: Config = basic_toml::from_str(r#"
//!         ip = '127.0.0.1'
//!
//!         [keys]
//!         github = 'xxxxxxxxxxxxxxxxx'
//!         travis = 'yyyyyyyyyyyyyyyyy'
//!     "#).unwrap();
//!
//!     assert_eq!(config.ip, "127.0.0.1");
//!     assert_eq!(config.port, None);
//!     assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
//!     assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
//! }
//! ```
//!
//! You can serialize types in a similar fashion:
//!
//! ```rust
//! use serde_derive::Serialize;
//!
//! #[derive(Serialize)]
//! struct Config {
//!     ip: String,
//!     port: Option<u16>,
//!     keys: Keys,
//! }
//!
//! #[derive(Serialize)]
//! struct Keys {
//!     github: String,
//!     travis: Option<String>,
//! }
//!
//! fn main() {
//!     let config = Config {
//!         ip: "127.0.0.1".to_string(),
//!         port: None,
//!         keys: Keys {
//!             github: "xxxxxxxxxxxxxxxxx".to_string(),
//!             travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
//!         },
//!     };
//!
//!     let toml = basic_toml::to_string(&config).unwrap();
//! }
//! ```
//!
//! [TOML]: https://github.com/toml-lang/toml
//! [Cargo]: https://crates.io/
//! [`serde`]: https://serde.rs/

#![deny(missing_docs)]
#![allow(
    clippy::bool_to_int_with_if,
    clippy::manual_let_else,
    clippy::manual_range_contains,
    clippy::match_like_matches_macro,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_borrow,
    clippy::needless_borrowed_reference,
    clippy::needless_doctest_main,
    clippy::needless_lifetimes,
    clippy::needless_pass_by_value,
    clippy::redundant_else,
    clippy::redundant_field_names,
    clippy::result_large_err,
    clippy::semicolon_if_nothing_returned,
    clippy::similar_names,
    clippy::single_char_add_str,
    clippy::suspicious_to_owned,
    clippy::type_complexity,
    clippy::uninlined_format_args,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns,
    clippy::unwrap_or_else_default,
    clippy::wrong_self_convention
)]

mod de;
mod error;
mod ser;
mod tokens;

pub use crate::de::{from_slice, from_str};
pub use crate::error::Error;
pub use crate::ser::to_string;
