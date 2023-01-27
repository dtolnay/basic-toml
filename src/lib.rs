//! A library for parsing and producing data in [TOML] format using [Serde].
//!
//! TOML is designed to be "a config file format for humans": minimal and easy
//! to read due to obvious semantics.
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
//! The TOML format is widely used throughout the Rust community for
//! configuration, notably being used by [Cargo], Rust's package manager.
//!
//! [TOML]: https://toml.io
//! [Serde]: https://serde.rs
//! [Cargo]: https://crates.io
//!
//! # Deserialization
//!
//! ```
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
//! # Serialization
//!
//! ```
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

#![deny(missing_docs)]
#![allow(
    clippy::bool_to_int_with_if,
    clippy::manual_let_else,
    clippy::manual_range_contains,
    clippy::match_like_matches_macro,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::type_complexity,
    clippy::uninlined_format_args,
    clippy::unwrap_or_else_default
)]

mod de;
mod error;
mod ser;
mod tokens;

pub use crate::de::{from_slice, from_str};
pub use crate::error::Error;
pub use crate::ser::to_string;
