use std::fs;

pub fn dir(mut dir: std::vec::IntoIter<String>) -> () {
    let target = dir.next().unwrap_or("./".to_owned());
    let paths = fs::read_dir(target).unwrap();


    for path in paths {
        println!("{}", path.unwrap().file_name().to_string_lossy())
    }
}