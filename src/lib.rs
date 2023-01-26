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
//! name = "toml"
//! version = "0.4.2"
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
//!     let config: Config = toml::from_str(r#"
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
//!     let toml = toml::to_string(&config).unwrap();
//! }
//! ```
//!
//! [TOML]: https://github.com/toml-lang/toml
//! [Cargo]: https://crates.io/
//! [`serde`]: https://serde.rs/

#![doc(html_root_url = "https://docs.rs/toml/0.5")]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
// Makes rustc abort compilation if there are any unsafe blocks in the crate.
// Presence of this annotation is picked up by tools such as cargo-geiger
// and lets them ensure that there is indeed no unsafe code as opposed to
// something they couldn't detect (e.g. unsafe added via macro expansion, etc).
#![forbid(unsafe_code)]

pub mod ser;
#[doc(no_inline)]
pub use crate::ser::{to_string, to_string_pretty, to_vec, Serializer};
pub mod de;
#[doc(no_inline)]
pub use crate::de::{from_slice, from_str, Deserializer};
mod tokens;

// Just for rustdoc
#[allow(unused_imports)]
use core::str::FromStr;
