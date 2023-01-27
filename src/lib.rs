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
//! use semver::{Version, VersionReq};
//! use serde_derive::Deserialize;
//! use std::collections::BTreeMap as Map;
//!
//! #[derive(Deserialize)]
//! struct Manifest {
//!     package: Package,
//!     #[serde(default)]
//!     dependencies: Map<String, VersionReq>,
//! }
//!
//! #[derive(Deserialize)]
//! struct Package {
//!     name: String,
//!     version: Version,
//!     #[serde(default)]
//!     authors: Vec<String>,
//! }
//!
//! fn main() {
//!     let manifest: Manifest = basic_toml::from_str(r#"
//!         [package]
//!         name = "basic-toml"
//!         version = "0.0.0"
//!         authors = ["Alex Crichton <alex@alexcrichton.com>"]
//!
//!         [dependencies]
//!         serde = "^1.0"
//!     "#).unwrap();
//!
//!     assert_eq!(manifest.package.name, "basic-toml");
//!     assert_eq!(manifest.package.version, Version::new(0, 0, 0));
//!     assert_eq!(manifest.package.authors, ["Alex Crichton <alex@alexcrichton.com>"]);
//!     assert_eq!(manifest.dependencies["serde"].to_string(), "^1.0");
//! }
//! ```
//!
//! # Serialization
//!
//! ```
//! use semver::{Version, VersionReq};
//! use serde_derive::Serialize;
//! use std::collections::BTreeMap as Map;
//!
//! #[derive(Serialize)]
//! struct Manifest {
//!     package: Package,
//!     dependencies: Map<String, VersionReq>,
//! }
//!
//! #[derive(Serialize)]
//! struct Package {
//!     name: String,
//!     version: Version,
//!     authors: Vec<String>,
//! }
//!
//! fn main() {
//!     let manifest = Manifest {
//!         package: Package {
//!             name: "basic-toml".to_owned(),
//!             version: Version::new(0, 0, 0),
//!             authors: vec!["Alex Crichton <alex@alexcrichton.com>".to_owned()],
//!         },
//!         dependencies: {
//!             let mut dependencies = Map::new();
//!             dependencies.insert("serde".to_owned(), "^1.0".parse().unwrap());
//!             dependencies
//!         },
//!     };
//!
//!     let toml = basic_toml::to_string(&manifest).unwrap();
//!     print!("{}", toml);
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
