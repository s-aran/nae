

use rustop::opts;
use std::path;

mod natural_sort;


fn main() {
    println!("Hello, world!");

    let mut v = vec!["100", "1", "10", "2", "222", "0", "30"];
    v.sort();

    for i in v.iter() {
        println!("{}", i);
    }

    let target = path::PathBuf::from("./");
    // enumFiles(&target);
}

fn enumFiles(target: &path::PathBuf) {
    let files = target.read_dir().unwrap();

    for dir_entry in files {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();

        if path.is_dir() {
            enumFiles(&path);
        } else {
            println!("{}", path.to_str().unwrap());
        }
    }
}
