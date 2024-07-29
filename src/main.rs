#![allow(warnings)]

extern crate plist;
mod command;
mod directory;
mod utils;

use std::{
    env,
    ffi::OsStr,
    fs::{self, metadata, ReadDir},
    path::PathBuf,
};

use directory::Directory;

fn main() {
    let args: Vec<String> = env::args().collect();
    handle_command(&args[1], &args);
}

fn handle_command(cmd: &String, args: &Vec<String>) {
    match &cmd[..] {
        "info" => handle_info(&args[2]),
        "clean" => handle_clean(&args[2], &args),
        _ => {}
    }
}

fn handle_info(cmd: &String) {
    match &cmd[..] {
        "all" => {
            let derived_data = Directory::new("/Library/Developer/Xcode/DerivedData");
            let caches = Directory::new("/Library/Developer/CoreSimulator/Caches");
            let devices = Directory::new("/Library/Developer/CoreSimulator/Devices");
            println!("{derived_data}");
            println!("{caches}");
            println!("{devices}");
        }
        "derived-data" => {
            let derived_data = Directory::new("/Library/Developer/Xcode/DerivedData");
            println!("{derived_data}");
        }
        "caches" => {
            let caches = Directory::new("/Library/Developer/CoreSimulator/Caches");
            println!("{caches}");
        }
        "devices" => {
            grab_simulators();
        }
        _ => {}
    }
}

fn handle_clean(cmd: &String, args: &Vec<String>) {
    match &cmd[..] {
        "all" => {
            println!("Cleaning all...");
        }
        "derived-data" => {
            let derived_data = Directory::new("/Library/Developer/Xcode/DerivedData");
            let path = &derived_data.home_path;
            match fs::remove_dir_all(path) {
                Ok(()) => println!("Removed: {derived_data}"),
                Err(e) => println!("Failed: {e}"),
            }
        }
        "caches" => {
            let caches = Directory::new("/Library/Developer/CoreSimulator/Caches");
            let path = &caches.home_path;
            match fs::remove_dir_all(path) {
                Ok(()) => println!("Removed: {caches}"),
                Err(e) => println!("Failed: {e}"),
            }
        }
        "devices" => {
            let udid = &args[3][..];
            let path = utils::home_path_to("/Library/Developer/CoreSimulator/Devices/") + udid;
            match fs::remove_dir_all(path) {
                Ok(()) => println!("Removed: {udid}"),
                Err(e) => println!("Failed: {e}"),
            }
        }
        _ => {}
    }
}

struct Simulator {
    name: String,
    size: f64,
}

fn dir_size(dir: &mut ReadDir) -> std::io::Result<u64> {
    dir.try_fold(0, |result, file| {
        let file = file?;
        let size = match file.metadata()? {
            data if data.is_dir() => {
                let sub_dir = &mut fs::read_dir(file.path())?;
                dir_size(sub_dir)?
            }
            data => data.len(),
        };

        Ok(result + size)
    })
}

fn grab_simulators() -> std::io::Result<()> {
    let path = utils::home_path_to("/Library/Developer/CoreSimulator/Devices");
    let mut simulators: Vec<Simulator> = Vec::new();
    let dir = fs::read_dir(path)?;

    let sim_directories = dir
        .filter_map(Result::ok)
        .filter_map(|f| match f.metadata() {
            Result::Ok(data) if data.is_dir() => Some(f),
            _ => None,
        })
        .map(|dir| dir.path())
        .for_each(|dir| {
            simulator_info(&dir);
            simulator_size(&dir);
        });
    Ok(())
}

fn simulator_info(path: &PathBuf) -> std::io::Result<()> {
    let dir = &mut fs::read_dir(path)?;

    dir.filter_map(Result::ok)
        .filter_map(|file| match file.path().extension() {
            Option::Some(data) if data.to_str() == Some("plist") => Some(file),
            _ => None,
        })
        .for_each(|file| {
            let name = file.file_name();
            let path = file.path();
            let content = plist::Value::from_file(path).expect("Failed to read device.plist");
            let name = simulator_name(&content).expect("Name is not available");
            let udid = simulator_udid(&content).expect("UUID is not available");
            println!("  -{name}");
            println!("  -{udid}");
        });

    Ok(())
}

fn simulator_name(plist: &plist::Value) -> std::io::Result<&str> {
    let name = plist
        .as_dictionary()
        .and_then(|dict| dict.get("name"))
        .unwrap()
        .as_string()
        .unwrap();

    return Ok(name);
}

fn simulator_udid(plist: &plist::Value) -> std::io::Result<&str> {
    let name = plist
        .as_dictionary()
        .and_then(|dict| dict.get("UDID"))
        .unwrap()
        .as_string()
        .unwrap();

    return Ok(name);
}

fn simulator_size(path: &PathBuf) -> std::io::Result<()> {
    let dir = &mut fs::read_dir(path)?;
    let size = dir_size(dir)? as f64;
    let result = size / 1_000_000_000 as f64;
    println!("      {result} GB");

    Ok(())
}
