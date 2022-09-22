use super::Config;
use super::IoDir;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

pub fn get_config(path: &str) -> Option<Config> {
    let scope = "rust";
    let description = String::new();
    let mut body: Vec<String> = Vec::new();

    let file_name = path.split('/').last().unwrap().split('.').next();
    let prefix = file_name.expect("").to_string();

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let line = line.unwrap();
        if !line.trim().starts_with("//") {
            body.push(line);
        }
    }
    Some(Config::new(scope, prefix, body, description))
}

pub fn write_snippets(config: Config, io_dir: &IoDir) -> Result<(), String> {
    let out_dir = &io_dir.out_dir;
    let output_path = String::new() + out_dir + &config.get_name() + ".code-snippets";
    let output = File::create(output_path);
    match output {
        Ok(file) => {
            let mut output = BufWriter::new(file);
            let result = output.write(config.into_snippets().as_bytes());
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to write .code-snippets".to_string()),
            }
        }
        Err(_) => Err("Can't find your output path".to_string()),
    }
}
