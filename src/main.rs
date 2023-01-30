#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]

use colored::*;
use notify_rust::Notification;
use opener::open;
use rand::seq::SliceRandom;
use std::fmt;
use std::path::Path;
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
    // set_wallpaper(choice);
    // set_wallpaper_mode(WallpaperMode::Wallpaper);
    print(get_wallpaper().blue());
    print(get_filename().red());

    let wallpaper: String = get_wallpaper();
    let current: &Path = Path::new(&wallpaper);

    let message = format!("current wallpaper is {} in {}", get_filename(), get_parent_folder());
    notify(&message);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn notify_current() {
    let message = format!("<b>{}</b> in <b>{}</b>", get_filename(), get_parent_folder());
    notify(&message, &get_wallpaper());
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
    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
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
    String::from_utf8_lossy(&current_wallpaper.stdout)
        .trim()
        .strip_suffix("'")
        .unwrap()
        .strip_prefix("'")
        .unwrap()
        .to_string()
}

fn get_filename() -> String {
    let current_wallpaper = get_wallpaper();
    return Path::new(&current_wallpaper)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
}

fn get_parent_folder() -> String {
    let current_wallpaper = get_wallpaper();
    return Path::new(&current_wallpaper)
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
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

fn notify(body: &str, image: &str) {
    Notification::new()
        .summary("Woopaper")
        .appname("Woopaper")
        .body(body)
        .icon("org.gnome.wallpaper")
        .image_path(image)
        .show()
        .unwrap();
}
