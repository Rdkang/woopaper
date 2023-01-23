#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rand::seq::SliceRandom;
use std::process::Command;
use std::vec;
use walkdir::WalkDir;

fn main() {
    let path = "/home/rdkang/Pictures/Wallpapers/";

    // lists all files and not directory
    let mut files: Vec<walkdir::DirEntry> = Vec::new();
    for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            // println!("{}", file.path().display());
            files.push(file);
        }
    }

    let choice: &walkdir::DirEntry = files.choose(&mut rand::thread_rng()).unwrap();
    // print!("{:?}", choice);
    set_wallpaper(choice);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn set_wallpaper(path: &walkdir::DirEntry) {
    Command::new("gsettings")
        .args(&[
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &path.path().display().to_string(),
        ])
        .output()
        .unwrap();
}
