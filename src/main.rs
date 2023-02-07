#![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use imagesize::size;
use notify_rust::Notification;
use opener::open;
use rand::seq::SliceRandom;
use std::fmt;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Choice,
}

#[derive(Subcommand)]
enum Choice {
    #[command(arg_required_else_help = true)]
    Wallpaper {
        // #[arg(value_name = "Command")]
        option: Commands,
    },
    #[command(arg_required_else_help = true)]
    Open { option: OpenChoices },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Commands {
    /// sets a random wallpaper
    Random,
    /// Shows the current wallpaper
    Status,
    /// Puts current wallpaper in the trash
    Trash,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum OpenChoices {
    /// Opens current wallpaper in file manager
    Manager,
    /// Opens 20 wallpapers in sxiv
    //TOOD:
    Sxiv,
    /// Opens image in default image viewer
    Viewer,
}
fn main() {
    print(get_wallpaper().blue());
    let path: &str = "/home/rdkang/Pictures/Wallpapers/";
    let width: usize = 1080;
    let height: usize = 1920;

    let arguments = Cli::parse();
    match arguments.command {
        Choice::Wallpaper { option } => match option {
            Commands::Random => {
                // TODO: make this into a function
                // lists all files and not directory

                let mut files: Vec<walkdir::DirEntry> = Vec::new();
                for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
                    if file.metadata().unwrap().is_file() {
                        files.push(file);
                    }
                }

                let choice = files.choose(&mut rand::thread_rng()).unwrap();
                if image_size_check(choice.path().display().to_string(), width, height) {
                    set_wallpaper(choice);
                    set_wallpaper_mode(WallpaperMode::Zoom);
                } else {
                    // TODO:  refactor to recursion
                    notify("Image is not the right size", &get_wallpaper());
                }
            }
            Commands::Status => notify_current(),
            Commands::Trash => trash_file(get_wallpaper()),
        },
        Choice::Open { option } => match option {
            OpenChoices::Manager => open_in_file_manger(get_wallpaper()),
            OpenChoices::Sxiv => open_file(get_wallpaper()),
            OpenChoices::Viewer => open_file(get_wallpaper()),
        },
    }
}

fn image_size_check(path: String, width_min: usize, height_min: usize) -> bool {
    let path_temp = path.clone();
    let (width, height) = match size(path) {
        Ok(dim) => (dim.width, dim.height),
        Err(why) => panic!("Error getting image size: {why}"),
    };

    if width <= width_min {
        let message = format!(
            "<b>{}</b> in <b>{}</b> Width is too small",
            get_filename(path_temp.clone()),
            get_parent_folder(path_temp.clone())
        );
        notify(&message, &path_temp);
        false
    } else if height <= height_min {
        let message = format!(
            "<b>{}</b> in <b>{}</b> Height is too small",
            get_filename(path_temp.clone()),
            get_parent_folder(path_temp.clone())
        );
        notify(&message, &path_temp);
        false
    } else {
        true
    }
}

fn open_in_file_manger(file: String) {
    Command::new("nautilus").args([file]).output().unwrap();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn notify_current() {
    let message = format!("<b>{}</b> in <b>{}</b>", get_filename(), get_parent_folder());
    notify(&message, &get_wallpaper());
}

fn print(text: ColoredString) {
    println!("{text}");
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
        .strip_suffix('\'')
        .unwrap()
        .strip_prefix('\'')
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

fn trash_file(file: String) {
    let file_temp = file.clone();
    match trash::delete(file) {
        Ok(_fc) => {
            print(format!("Sucess putting {} in the trash", file_temp.magenta()).green());

            let message = format!("trashed {} in {}", get_filename(), get_parent_folder());
            // FIX: show image of deleted
            notify(&message, &file_temp)
        }
        Err(error) => panic!("{error} trouble trashing file"),
    }
    // TODO: make it set new wallpaper after removing
}

fn notify(body: &str, image: &str) {
    Notification::new()
        .summary("Woopaper")
        .appname("Woopaper")
        .body(body)
        .hint(notify_rust::Hint::Transient(true))
        .icon("org.gnome.wallpaper")
        .image_path(image)
        .action("trash", "Put image in trash")
        .action("manager", "Open in file manager")
        .action("open", "Open in image viewer")
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "trash" => trash_file(get_wallpaper()),
            "manager" => open_in_file_manger(get_wallpaper()),
            "open" => open_file(get_wallpaper()),
            _ => print("default".blue()),
        });
}
