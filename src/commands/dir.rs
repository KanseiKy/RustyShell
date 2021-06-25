// use std::path::PathBuf;
use std::fs;

pub fn dir(dir: std::str::SplitWhitespace) -> () {
    let target = dir.peekable().peek().map_or("./", |x| *x);
    let paths = fs::read_dir(target).unwrap();


    for path in paths {
        println!("{}", path.unwrap().file_name().to_string_lossy())
    }
}