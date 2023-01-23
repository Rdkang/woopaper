#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rand::seq::SliceRandom;
use std::vec;
use walkdir::WalkDir;
use wallpaper;

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
    print!("{:?}", choice);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn set_wallpaper() {
    // println!("{:?}", wallpaper::get());
    // wallpaper::set_from_path("/home/rdkang/Pictures/Wallpapers/islandDay.jpg").unwrap();
    // println!("{:?}", wallpaper::get());
}
