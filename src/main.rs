extern crate mp3_metadata;
#[macro_use] extern crate structopt;
extern crate glob;

use std::path::{Path, PathBuf};
use std::fs::rename;

use structopt::StructOpt;

use mp3_metadata::read_from_file;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "t", default_value = "%n - %a.mp3", long = "template")]
    template: String,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>
}

fn main() {
    let options = Options::from_args();
    
    if options.files.len() == 0 {
        println!("Must submit file!");
        return;
    } else {
        let options = Options {
            files: expand_globs(options.files),
            ..options
        };

        iterate(options);
    }
}

fn expand_globs(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut expanded = Vec::new();
    
    for path in paths {
        if has_glob(&path) {
            for entry in glob::glob(path.to_str().unwrap()).unwrap() {
                expanded.push(entry.unwrap());
            }
        } else {
            expanded.push(path);
        }
    }
    
    expanded
}

fn has_glob(p: &Path) -> bool {
    p.to_str()
        .map(|s| s.chars().any(|c| c == '*'))
        .unwrap_or(false)
}

fn iterate(Options {template, files}: Options) {
    for f in files {
        if rename_file(&f, &template) {
            println!("Renamed {:?}", f);
        } else {
            println!("Could not rename {:?}", f);
        }
    }
}

fn rename_file(file: &Path, _template: &str) -> bool {
    read_from_file(file).ok()
        .map(|meta| meta.optional_info)
        .and_then(|opts| {
            let t_n = opts.into_iter()
                .filter_map(|opt| if let (Some(t), Some(n)) = (opt.title, opt.track_number) {
                    Some((t, n))
                } else {
                    None
                })
                .next();

            if let Some((tag, n)) = t_n {
                Some((tag, n))
            } else {
                None
            }
        })
        .map(|(title, num)| if let Some(pos) = num.find('/') {
            let total = *&num[(pos+1)..]
                .trim_matches(|c: char| !c.is_digit(10));
            
            let num = num[..pos].trim_matches(|c: char| !c.is_digit(10));

            (title, "0".repeat(total.len() - num.len()) + num)
        } else {
            (title, num)
        })
        .map(|(title, num)| rename(
            file, 
            format!(
                "{} - {}.mp3", 
                num, 
                title.trim_matches(char::is_control).trim()
            )
        ).is_ok())
        .unwrap_or(false)
}