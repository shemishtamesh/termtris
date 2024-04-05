use crate::config::{find_config_file, CONFIG_FILE_NAME};
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn log(msg: &str) {
    let config_file_path = find_config_file();
    let config_dir_path =
        config_file_path[..config_file_path.len() - CONFIG_FILE_NAME.len()].to_string();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{config_dir_path}log.txt"))
        .unwrap();

    if let Err(e) = writeln!(file, "{}", msg) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
