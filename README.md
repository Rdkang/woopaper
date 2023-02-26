<h3 align="center">Woopaper</h3>

<div align="center">

  [![Status](https://img.shields.io/badge/status-active-success.svg)]() 
  [![GitHub Issues](https://img.shields.io/github/issues/rdkang/The-Documentation-Compendium.svg)](https://github.com/rdkang/woopaper/issues)
  [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/kylelobo/The-Documentation-Compendium.svg)](https://github.com/kylelobo/The-Documentation-Compendium/pulls)

</div>

---

## 🧐 About
Woopaper is a program written in Rust that allows you to change your wallpaper quickly and easily using keyboard shortcuts or the command line. It also includes features such as showing the current wallpaper as a notification, and easily deleting the image or opening it in the default image viewer.

This program's predecessor is [chinguRandomWallpaper](https://github.com/Rdkang/chinguRandomWallpaper) and have decided to improve it in rust

## Installation
To build woopaper from scratch , you'll need to have Rust installed on your system. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can download and compile the woopaper program by cloning this repository and running the following command:

```bash
git clone https://github.com/Rdkang/woopaper
```
- This will compile the wooclock program and create an executable file in the `target/release` directory. and could can put in your chosen location and run like any other program.
```bash
cargo build --release
```

- Or while in cloned directory do `cargo install --path .` which will install to your cargo home directory

- Or just download the executable from the github's release page


## Usage
Woopaper has two subcommands: `open` and `wallpaper`.
`open` subcommand

The open subcommand allows you to open the current wallpaper in a specified image viewer. The following flags are available:
- `manager`: Opens the wallpaper directory in your file manager.
- `sxiv`: Opens the wallpaper in the sxiv image viewer.
- `viewer`: Opens the wallpaper in the default image viewer.

Example usage:

will open the current wallpaper in your default file manager so that you can do what you want with it
```bash
woopaper open manager
```

`wallpaper` subcommand

The wallpaper subcommand allows you to change the wallpaper and perform other actions related to wallpapers. The following flags are available:

- `random`: Sets a random wallpaper from the wallpaper directory.
- `status`: Shows the current wallpaper and its file path.
- `trash`: Moves the current wallpaper to the trash directory.

Example usage:

will set a random wallpaper
```bash
woopaper wallpaper random
```
## Configuration
By default, the program will use the `~/Pictures/Wallpapers/` directory to get wallpapers. You can change this in config file at `~/.config/woopaper/config.toml`
The default config file is stored in `~/.config/woopaper/config.toml`
the following options:
- `path` (path) : path to where want the program get it's wallpapers
- `height` (integer) : the minimum height of the wallpaper image, otherwise will be skipped
- `width` (integer) : the minimum width of the wallpaper image, otherwise will be skipped
- `notify_problem` (boolean) : if the program will show a notification when the image doesn't meet the minimum size requirements. **false** will just skip that image and try the next one


## Features

- Notification with current wallpaper and can click on it for common actions such as [trash,open in file manager, open in image viewer]

### Roadmap
- [ ] TODO: favorite wallpapers
- [ ] TODO: fuzzy find through wallpapers
- [ ] TODO: fuzzy through wallpapers
- [ ] TODO: set wallpaper by given path
- [ ] TODO: open 20 wallpapers in sxiv to be set
- [ ] TODO: set wallpaper from given path
- [x] DONE: file manager
- [x] DONE: random
- [x] DONE: trash
- [x] DONE: sxiv
- [x] DONE: notifications
- [x] DONE: config file

## 🎉 Acknowledgements

- https://github.com/vineetred/flowy/tree/master/wallpaper_rs
- https://github.com/reujab/wallpaper.rs
