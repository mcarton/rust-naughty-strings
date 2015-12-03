# rust-naughty-strings
Rust binding of [minimaxir's Big List Of Naughty
Strings](https://github.com/minimaxir/big-list-of-naughty-strings)

The crate provide only one array `naughty_strings::BLNS : [&'static str; N]`:
```rust
extern crate naughty_strings;

fn main() {
    for ns in naughty_strings::BLNS.iter() {
        println!("{}", ns);
    }
}

```

Note that because those strings are naughty, I can't even upload a `rustdoc`
generated documentation: `rustdoc` does not escape the various `<script>` in
the list and the doc is unusable.

## Generate the library
The library is generated with `cargo run --bin make-lib [PATH-TO-BLNS-REPO]`
(`PATH-TO-BLNS-REPO` defaults to
[minimaxir's repo](https://github.com/minimaxir/big-list-of-naughty-strings.git)).
This generates the sole file [*src/lib.rs*](src/lib.rs).
