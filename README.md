# rust-naughty-strings
Rust binding of [minimaxir's Big List Of Naughty Strings][minimaxir-blns].

The whole list is the array `naughty_strings::BLNS: &[&str]`:
```rust
fn main() {
    for ns in naughty_strings::BLNS {
        println!("{}", ns);
    }
}
```

The strings are also grouped by the categories of the upstream list, such as
`naughty_strings::categories::SQL_INJECTION`, and `naughty_strings::CATEGORIES`
lists all of them:
```rust
use naughty_strings::categories::SQL_INJECTION;

fn main() {
    for ns in SQL_INJECTION.strings {
        println!("{}", ns);
    }
}
```

The crate is `#![no_std]` and has no dependencies.

## *crates.io*
The crate [is on *crates.io*][crate].

## Generate the library
The list is generated with `cargo run -p make-lib [PATH-TO-BLNS-REPO]`
(`PATH-TO-BLNS-REPO` defaults to [minimaxir's repo][PATH-TO-BLNS-REPO]).
This regenerates [*src/blns.rs*](src/blns.rs), which [*src/lib.rs*](src/lib.rs)
includes. The generator is a separate workspace member, so its dependencies are
not pulled in by users of the crate.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Contributions are welcome, Just open an issue or make a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[PATH-TO-BLNS-REPO]: https://github.com/minimaxir/big-list-of-naughty-strings.git
[crate]: https://crates.io/crates/naughty-strings
[minimaxir-blns]: https://github.com/minimaxir/big-list-of-naughty-strings
