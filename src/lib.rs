//! Rust binding of [minimaxir's Big List of Naughty Strings][blns]: strings
//! which have a high probability of causing issues when used as user-input data.
//!
//! ```
//! for ns in naughty_strings::BLNS {
//!     println!("{}", ns);
//! }
//! ```
//!
//! [blns]: https://github.com/minimaxir/big-list-of-naughty-strings

#![no_std]

include!("blns.rs");
