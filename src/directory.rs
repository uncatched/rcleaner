use std::{
    fmt,
    fs::{self, metadata, ReadDir},
    path::PathBuf,
};

use crate::utils;

#[derive(Debug)]
pub struct Directory<'a> {
    path: &'a str,
    size: f64,
}

impl<'a> Directory<'a> {
    pub fn new(path: &'a str) -> Self {
        let directory_size = check_directory(&path).unwrap();

        return Directory {
            path: &path,
            size: directory_size,
        };
    }
}

fn check_directory(path: &str) -> std::io::Result<f64> {
    let absolute_path = utils::home_path_to(path);
    let dir = &mut fs::read_dir(absolute_path)?;
    let size = dir_size(dir)? as f64;
    let result = size / 1_000_000_000 as f64;

    Ok(result)
}

fn format_size(size: f64) -> String {
    format!("{:.2}", size)
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

impl<'a> fmt::Display for Directory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {} GB", self.path, format_size(self.size))
    }
}
