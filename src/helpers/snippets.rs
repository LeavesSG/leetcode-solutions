mod config;
mod generate;

use std::io::{BufRead, BufReader};

pub use config::Config;
use generate::{get_config, write_snippets};

const CONFIG_PATH: &str = "config/snippets-io.config";

pub struct IoDir {
    pub out_dir: String,
    pub in_dir: String,
}

impl IoDir {
    pub fn get() -> Option<Self> {
        let mut out_dir = String::new();
        let mut in_dir = String::new();
        if let Ok(file) = std::fs::File::open(CONFIG_PATH) {
            let rd = BufReader::new(file);
            for line in rd.lines().flatten() {
                if line.contains("in_dir") {
                    in_dir = line.split(' ').last().unwrap_or("src/algs/").to_string();
                }
                if line.contains("out_dir") {
                    out_dir = line.split(' ').last().unwrap_or(".vscode/").to_string();
                }
            }
            return Some(IoDir { in_dir, out_dir });
        }
        None
    }
}

pub fn batch() {
    if let Some(io_config) = IoDir::get() {
        let res = std::fs::read_dir(&io_config.in_dir).unwrap();
        res.for_each(|dir| {
            let config = get_config(
                (String::new()
                    + io_config.in_dir.as_str()
                    + dir.unwrap().file_name().to_str().unwrap())
                .as_str(),
            );
            write_snippets(config.unwrap(), &io_config).unwrap();
        })
    }
}

#[test]
pub fn convert_to_snippets() {
    let res = std::fs::read_dir("src/algs").unwrap();
    res.for_each(|x| {
        println!("{:?}", x.unwrap().file_name());
    })
}
