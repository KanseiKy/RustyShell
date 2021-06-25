use std::path::Path;
use std::env;

pub fn cd(mut dir: std::vec::IntoIter<String>) -> () {
    let current_dir = dir.next().unwrap_or("./".to_owned());
    let root = Path::new(&current_dir);

    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e)
    }
}