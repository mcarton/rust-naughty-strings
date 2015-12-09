# rust-naughty-strings
Rust binding of [minimaxir's Big List Of Naughty Strings][minimaxir-blns].

The crate only provides one array `naughty_strings::BLNS : &'static [&'static str]`:
```rust
extern crate naughty_strings;

fn main() {
    for ns in naughty_strings::BLNS {
        println!("{}", ns);
    }
}

```

## *crates.io*
The crate [is on *crates.io*][crate].

## Documentation
The documentation is available [here][doc].

Note that because those strings are so naughty, `rustdoc` did not handle the
documentation of the crate correctly!

## Generate the library
The library is generated with `cargo run --bin make-lib [PATH-TO-BLNS-REPO]`
(`PATH-TO-BLNS-REPO` defaults to [minimaxir's repo][PATH-TO-BLNS-REPO]).
This generates the sole file [*src/lib.rs*](src/lib.rs).

[PATH-TO-BLNS-REPO]: https://github.com/minimaxir/big-list-of-naughty-strings.git
[crate]: https://crates.io/crates/naughty-strings
[doc]: https://mcarton.github.io/rust-naughty-strings/doc-latest/naughty_strings/
[minimaxir-blns]: https://github.com/minimaxir/big-list-of-naughty-strings
