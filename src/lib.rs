//! Rust binding of [minimaxir's Big List of Naughty Strings][blns]: strings
//! which have a high probability of causing issues when used as user-input data.
//!
//! [`BLNS`] holds the whole list:
//!
//! ```
//! for ns in naughty_strings::BLNS {
//!     println!("{}", ns);
//! }
//! ```
//!
//! The list is split into the categories of the upstream file, so tests can
//! focus on one kind of input:
//!
//! ```
//! use naughty_strings::categories::SQL_INJECTION;
//!
//! for ns in SQL_INJECTION.strings {
//!     println!("{}", ns);
//! }
//!
//! for category in naughty_strings::CATEGORIES {
//!     println!("{}: {} strings", category.name, category.strings.len());
//! }
//! ```
//!
//! [blns]: https://github.com/minimaxir/big-list-of-naughty-strings

#![no_std]

/// A section of the list, as delimited in upstream's `blns.txt`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Category {
    /// Upstream's title of the section, e.g. `"SQL Injection"`.
    pub name: &'static str,
    /// Upstream's description of the section.
    pub description: &'static str,
    /// The strings of the section, in list order.
    pub strings: &'static [&'static str],
}

include!("blns.rs");
