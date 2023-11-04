#![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use imagesize::size;
use notify_rust::Notification;
use opener::open;
use rand::seq::SliceRandom;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{read_to_string, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
extern crate confy;
use flexi_logger::{Duplicate, FileSpec, Logger};
use skim::prelude::*;
use std::io::{BufRead, Write};
use std::io::{BufReader, Cursor};

#[macro_use]
extern crate serde_derive;

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
    /// Setting wallpaper operations
    Wallpaper {
        // #[arg(value_name = "Command")]
        option: WallpaperChoices,
    },
    #[command(arg_required_else_help = true)]
    /// Opening the current wallpaper operations
    Open { option: OpenChoices },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum WallpaperChoices {
    /// sets a random wallpaper
    Random,
    /// Shows the current wallpaper
    Status,
    /// Puts current wallpaper in the trash
    Trash,
    /// Shows all wallpapers in a fzf for you to choose
    Fzf,
    /// Adds the current wallpaper to favorite list
    Favorite,
    /// Fuzzy find through all favorited wallpapers and set it as wallpaper
    FzfFavorite,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum OpenChoices {
    /// Opens current wallpaper in file manager
    Manager,
    //TOOD:
    /// Opens 20 wallpapers in sxiv
    Sxiv,
    /// Opens image in default image viewer
    Viewer,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfyConfig {
    path: PathBuf,
    height: usize,
    width: usize,
    notify_problem: bool,
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            path: PathBuf::from("/home/rdkang/Pictures/Wallpapers"),
            width: 1920,
            height: 1080,
            notify_problem: false,
        }
    }
}

fn get_config() -> ConfyConfig {
    confy::load("woopaper", "config").unwrap_or_else(|e| match e {
        confy::ConfyError::BadTomlData(_) => {
            println!("Bad toml data");
            let config = ConfyConfig::default();
            confy::store("woopaper", "config", &config).unwrap();
            config
        }
        _ => panic!("Error getting config file: {}", e),
    })
}

fn main() {
    // print()
    let _logger = Logger::try_with_str("info")
        .unwrap()
        // FIX: a fixed logs folder
        .log_to_file(FileSpec::default().directory("logs")) // write logs to file
        .duplicate_to_stderr(Duplicate::Warn) // print warnings and errors also to the console
        .create_symlink("current_log.log")
        .start();

    let arguments = Cli::parse();
    match arguments.command {
        Choice::Wallpaper { option } => match option {
            WallpaperChoices::Random => set_random(),
            WallpaperChoices::Status => {
                print(get_wallpaper().yellow());
                notify_current()
            }
            WallpaperChoices::Trash => trash_file(get_wallpaper()),
            WallpaperChoices::Fzf => fuzzy(),
            WallpaperChoices::Favorite => favorite(get_wallpaper()),
            WallpaperChoices::FzfFavorite => fuzzy_favorites(),
        },
        Choice::Open { option } => match option {
            OpenChoices::Manager => open_in_file_manger(get_wallpaper()),
            OpenChoices::Sxiv => open_file(get_wallpaper()),
            OpenChoices::Viewer => open_file(get_wallpaper()),
        },
    }
}

fn get_path() -> PathBuf {
    get_config().path
}

fn set_random() {
    // is a vector of random files
    let files_random = get_random(get_files(), 1);

    // if file meets minimum requirements then will set it as wallpaper otherwise will recursion
    // and call it self and retry
    if image_size_check(files_random[0].path().display().to_string()) {
        wallpaper::set_from_path(&PathBuf::from(files_random[0].path()).display().to_string()).unwrap();
        wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    } else {
        set_random();
    }
}

fn image_size_check(path: String) -> bool {
    let path_temp = path.clone();
    let (width, height) = match size(path) {
        Ok(dim) => (dim.width, dim.height),
        Err(why) => {
            log::warn!("Error getting image size: {why} for {path_temp}");
            panic!("Error getting image size: {why}")
        }
    };

    let message = if width <= get_config().width {
        format!(
            "<b>{}</b> in <b>{}</b> Width is too small",
            get_filename(path_temp.clone()),
            get_parent_folder(path_temp.clone())
        )
    } else if height <= get_config().height {
        format!(
            "<b>{}</b> in <b>{}</b> Height is too small",
            get_filename(path_temp.clone()),
            get_parent_folder(path_temp.clone())
        )
    } else {
        "good".to_string()
    };

    // if user wants to notify that image doesn't meet minimum size then will show a notification of
    // the problem otherwise will be silent
    if message != "good" && get_config().notify_problem {
        notify(&message, &path_temp);
        print(message.yellow());
        return false;
    }
    true
}

fn get_files() -> Vec<walkdir::DirEntry> {
    // lists all files excluding directories
    let mut files: Vec<walkdir::DirEntry> = Vec::new();
    for file in WalkDir::new(get_config().path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            if file.path().has_extension(&["png", "jpg", "jpeg", "gif", "bmp"]) {
                files.push(file);
            }
        }
    }
    files
}

fn get_files_string() -> String {
    let mut files = String::new();
    for file in WalkDir::new(get_config().path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            if file.path().has_extension(&["png", "jpg", "jpeg", "gif", "bmp"]) {
                // pushes the file name with a new line appended to the files String
                files.push_str(&format!("{}\n", &file.path().display().to_string()));
            }
        }
    }
    files
}

fn get_random(files: Vec<walkdir::DirEntry>, num: usize) -> Vec<walkdir::DirEntry> {
    let choice = files.choose_multiple(&mut rand::thread_rng(), num).cloned().collect();
    choice
}

fn open_in_file_manger(file: String) {
    // FIX: make it work with other file managers
    Command::new("nautilus").args([file]).output().unwrap();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn notify_current() {
    let message = format!(
        "<b>{}</b> in <b>{}</b>",
        get_filename(get_wallpaper()),
        get_parent_folder(get_wallpaper())
    );
    notify(&message, &get_wallpaper());
}

fn print(text: ColoredString) {
    println!("{text}");
}

fn get_wallpaper() -> String {
    return wallpaper::get().unwrap();
}

fn get_filename(path: String) -> String {
    let current_wallpaper = path;
    return Path::new(&current_wallpaper)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
}

fn get_parent_folder(path: String) -> String {
    let current_wallpaper = path;
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

fn open_file(file: String) {
    open(file).unwrap();
}

fn trash_file(file: String) {
    let file_temp = file.clone();
    match trash::delete(file) {
        Ok(_fc) => {
            print(format!("Sucess putting {} in the trash", file_temp.magenta()).green());

            let message = format!(
                "trashed {} in {}",
                get_filename(get_wallpaper()),
                get_parent_folder(get_wallpaper())
            );
            notify(&message, &file_temp)
        }
        Err(error) => panic!("{error} trouble trashing file"),
    }
    set_random()
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
        // .action("open", "Open in image viewer")
        .action("favorite", "Add to favorites")
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "trash" => trash_file(image.to_string()),
            "manager" => open_in_file_manger(image.to_string()),
            "open" => open_file(image.to_string()),
            "favorite" => favorite(image.to_string()),
            _ => (),
        });
}

fn fuzzy() {
    // TODO: preview wallpaper
    let options = SkimOptionsBuilder::default()
        .prompt(Some("Woopaper > "))
        .header(Some("choose wallpaper"))
        .height(Some("30%"))
        .multi(false)
        .reverse(true)
        .nosort(true)
        .build()
        .unwrap();

    let items = SkimItemReader::default().of_bufread(Cursor::new(get_files_string()));

    let selected_files = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    let file = selected_files.iter().last().unwrap().output().to_string();
    // set_wallpaper(Path::new(&file).to_path_buf());
    wallpaper::set_from_path(&Path::new(&file).to_path_buf().display().to_string()).unwrap();
}

// allows to check if file is one of several extensions
pub trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P: AsRef<Path>> FileExtension for P {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(ref extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions.iter().any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }

        false
    }
}

fn favorite(wallpaper: String) {
    let file_path = get_favorite_path();
    // creates a file for appending wallpaper path too
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path.clone())
        .unwrap();

    // writes file and prints if successful or not
    match writeln!(file, "{}", wallpaper) {
        Ok(_okcode) => print(format!("Successfully favourited \"{}\"", wallpaper).green()),
        Err(error) => eprintln!("Problem in writing favorite: {}", error),
    }
    file.sync_all().unwrap();
    deduplicate_file(file_path.to_path_buf());
}

fn get_favorite_path() -> PathBuf {
    confy::get_configuration_file_path("woopaper", "config")
        .unwrap()
        .parent()
        .unwrap()
        .join("favorites.txt")
}

fn deduplicate_file(file_path: PathBuf) {
    let file = File::open(file_path.clone()).expect("file error");
    let reader = BufReader::new(file);

    let lines: BTreeSet<_> = reader.lines().map(|l| l.expect("Couldn't read a line")).collect();

    let mut file = File::create(file_path).expect("file error");

    for line in lines {
        file.write_all(line.as_bytes()).expect("Couldn't write to file");

        file.write_all(b"\n").expect("Couldn't write to file");
    }
}

fn fuzzy_favorites() {
    let favorite_file = read_to_string(get_favorite_path().to_str().unwrap()).unwrap();

    let options = SkimOptionsBuilder::default()
        .prompt(Some("Woopaper > "))
        .height(Some("30%"))
        .multi(false)
        .reverse(true)
        .nosort(true)
        .build()
        .unwrap();

    let items = SkimItemReader::default().of_bufread(Cursor::new(favorite_file));
    let selected_files = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    let file = selected_files.iter().last().unwrap().output().to_string();
    wallpaper::set_from_path(&Path::new(&file).to_path_buf().display().to_string()).unwrap();
}
