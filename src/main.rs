extern crate mp3_metadata;
#[macro_use] extern crate structopt;
extern crate glob;

use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "t", default_value = "%n - %a.mp3", long = "template")]
    template: String,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>
}

fn main() {
    let options: Options = Options::from_args();
    
    if options.files.len() == 0 {
        println!("Must submit file!");
        return;
    } else {
        for f in options.files {
            println!("{}", has_glob(&f));
        }
    }
}

fn has_glob(p: &Path) -> bool {
    p.to_str()
        .map(|s| s.chars().any(|c| c == '*'))
        .unwrap_or(false)
}