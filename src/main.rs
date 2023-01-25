#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use colored::*;
use opener::open;
use rand::seq::SliceRandom;
use std::fmt;
use std::process::Command;
use trash;
use walkdir::WalkDir;

fn main() {
    let path = "/home/rdkang/Pictures/Wallpapers/";

    // lists all files and not directory
    let mut files: Vec<walkdir::DirEntry> = Vec::new();
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            // println!("{}", file.path().display());
            files.push(file);
        }
    }

    let choice: &walkdir::DirEntry = files.choose(&mut rand::thread_rng()).unwrap();
    set_wallpaper(choice);
    set_wallpaper_mode(WallpaperMode::Wallpaper);
    get_wallpaper();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn print(text: ColoredString) {
    println!("{}", text)
}

fn set_wallpaper(path: &walkdir::DirEntry) {
    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri-dark",
            &path.path().display().to_string(),
        ])
        .output()
        .unwrap();
}

fn get_wallpaper() -> String {
    let current_wallpaper = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.background", "picture-uri-dark"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&current_wallpaper.stdout).to_string()
}

enum WallpaperMode {
    None,
    Wallpaper,
    Centered,
    Scaled,
    Stretched,
    Zoom,
    Spanned,
}

impl fmt::Display for WallpaperMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // when call .to_string() on it will make it this path string
            WallpaperMode::None => write!(f, "none"),
            WallpaperMode::Wallpaper => write!(f, "wallpaper"),
            WallpaperMode::Centered => write!(f, "centered"),
            WallpaperMode::Scaled => write!(f, "scaled"),
            WallpaperMode::Stretched => write!(f, "stretched"),
            WallpaperMode::Zoom => write!(f, "zoom"),
            WallpaperMode::Spanned => write!(f, "spanned"),
        }
    }
}

fn set_wallpaper_mode(mode: WallpaperMode) {
    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background ",
            "picture-options",
            &mode.to_string(),
        ])
        .output()
        .unwrap();
}

fn open_file(file: String) {
    open(file).unwrap();
}

fn delete_file(file: String) {
    trash::delete(file).unwrap();
}
