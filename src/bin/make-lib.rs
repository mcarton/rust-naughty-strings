extern crate rustc_serialize;

use rustc_serialize::json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

const NAUGHTY_REPO_URL : &'static str = &"https://github.com/minimaxir/big-list-of-naughty-strings.git";

fn main() {
    let path = std::env::args().nth(1).unwrap_or(NAUGHTY_REPO_URL.into());

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir).join("big-list-of-naughty-strings");

    Command::new("git").args(&["clone", &path, out_dir.to_str().unwrap()])
                       .status().unwrap();

    let blns = {
        let mut blns = File::open(out_dir.join("blns.json")).unwrap();
        let mut buffer = String::new();
        blns.read_to_string(&mut buffer).unwrap();
        buffer
    };

    std::fs::remove_dir_all(out_dir).unwrap();

    let blns : Vec<String> = json::decode(&blns).unwrap();

    let mut output = File::create("src/lib.rs").unwrap();
    writeln!(output, "/// Big list of naughty strings").unwrap();
    writeln!(output, "pub const BLNS: [&'static str; {}] = [", blns.len()).unwrap();

    for ns in blns {
        writeln!(output, "    {:?},", ns).unwrap();
    }

    write!(output, "];").unwrap();

}
