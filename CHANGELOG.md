# Unreleased

* Add `Category`, `CATEGORIES` and the `categories` module, one constant per
  section of the upstream list (e.g. `categories::SQL_INJECTION`).
* The crate is now `#![no_std]`.
* The generator moved to the `make-lib` workspace member, so `serde_json` is
  no longer a dependency of the crate.
* Use the 2021 edition.
* Generate the list from upstream's canonical `blns.txt` instead of the stale
  `blns.json`. This adds `constructor` and the Arabic `الكل في المجمو عة (5)`,
  fixes RTL and Ogham strings that had stray trailing characters, and numbers
  the XSS payloads (`alert(0)` to `alert(225)`) so a triggered alert identifies
  its payload.

# 0.2.4

Update the list.

# 0.2.3

Update the list.

# 0.2.2

Relicense under MIT or Apache-2.0.

# 0.2.1

Create documentation.

# 0.2.0

Fix potential type problem. See #1.
