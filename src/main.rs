use std::io::Write;
use std::{fs::OpenOptions, path::PathBuf, str::FromStr, usize};

use clap::Parser;

const DIRECTORY: &'static str = "/sys/class/backlight/apple-panel-bl";

fn get_brightness_file() -> PathBuf {
    let mut path = PathBuf::from_str(DIRECTORY).unwrap();
    path.push("brightness");
    path
}

fn get_max_brightness_file() -> PathBuf {
    let mut path = PathBuf::from_str(DIRECTORY).unwrap();
    path.push("max_brightness");
    path
}

fn read_max_brightness() -> eyre::Result<usize> {
    let contents = std::fs::read(get_max_brightness_file())?;
    let string = String::from_utf8(contents)?;
    let string = string.trim();
    let val = usize::from_str(&string)?;
    Ok(val)
}

fn read_brightness() -> eyre::Result<usize> {
    let contents = std::fs::read(get_brightness_file())?;
    let string = String::from_utf8(contents)?;
    let string = string.trim();
    let val = usize::from_str(&string)?;
    Ok(val)
}

fn cap_brightness(brightness: usize) -> eyre::Result<usize> {
    let max = read_max_brightness()?;
    if brightness > max {
        return Ok(max);
    }
    return Ok(brightness);
}

fn write_brightness(brightness: usize) -> eyre::Result<()> {
    let brightness = cap_brightness(brightness)?;
    let mut file = OpenOptions::new().write(true).open(get_brightness_file())?;
    write!(file, "{}", brightness)?;
    Ok(())
}

#[derive(Parser, Debug)]
enum Opts {
    Up { value: usize },
    Down { value: usize },
    Set { value: usize },
    Get,
}

fn main() -> eyre::Result<()> {
    let args = Opts::parse();

    match args {
        Opts::Up { value } => {
            let brightness = read_brightness()?.saturating_add(value);
            write_brightness(brightness)?;
        }
        Opts::Down { value } => {
            let brightness = read_brightness()?.saturating_sub(value);
            write_brightness(brightness)?;
        }
        Opts::Set { value: brightness } => {
            write_brightness(brightness)?;
        }
        Opts::Get => {
            println!("{}", read_brightness()?);
        }
    }
    Ok(())
}
