extern crate mp3_metadata;
#[macro_use] extern crate structopt;
extern crate glob;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "t", default_value = "%n - %a.mp3", long = "template")]
    template: String,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);
}
