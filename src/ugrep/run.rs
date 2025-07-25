use std::{error::Error, fs};

use super::args::Args;

pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut results = Vec::with_capacity(32);
    for line in haystack.lines() {
        if line.contains(needle) {
            results.push(line);
        }
    }

    results
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let Args { file_name, expr } = args;
    let content = fs::read_to_string(file_name).expect("can't read file");

    let results = search(expr, &content);
    for line in results {
        println!("{line}");
    }

    Ok(())
}
