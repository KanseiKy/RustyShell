use std::path::Path;
use std::env;

pub fn cd(dir: std::str::SplitAsciiWhitespace) -> () {
    let current_dir = dir.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(current_dir);

    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e)
    }
}