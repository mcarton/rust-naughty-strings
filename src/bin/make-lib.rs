extern crate serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

const NAUGHTY_REPO_URL : &'static str = &"https://github.com/minimaxir/big-list-of-naughty-strings.git";

fn main() {
    let path = std::env::args().nth(1).unwrap_or(NAUGHTY_REPO_URL.into());

    let out_dir = std::env::var("OUT_DIR").unwrap_or_else(|_| "/tmp".into());
    let out_dir = Path::new(&out_dir).join("big-list-of-naughty-strings");

    Command::new("git").args(&["clone", &path, out_dir.to_str().unwrap()])
                       .status().unwrap();

    let blns = {
        let blns = File::open(out_dir.join("blns.json")).unwrap();
        std::io::BufReader::new(blns)
    };

    std::fs::remove_dir_all(out_dir).unwrap();

    let blns : Vec<String> = serde_json::from_reader(blns).unwrap();

    let mut output = File::create("src/lib.rs").unwrap();

    writeln!(output,
        "/// Big list of naughty strings.\n\
         pub const BLNS: &'static [&'static str] = &["
    ).unwrap();

    for ns in blns {
        writeln!(output, "    {:?},", ns).unwrap();
    }

    writeln!(output, "];").unwrap();
}
