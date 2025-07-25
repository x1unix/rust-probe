use std::{error::Error, fs};

use super::args::Args;

pub fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let query = needle.to_lowercase();
    let mut results = Vec::with_capacity(32);
    for line in haystack.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

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
    let Args {
        file_name,
        expr,
        case_insensitive,
    } = args;
    let content = fs::read_to_string(file_name).expect("can't read file");

    let results = if *case_insensitive {
        search_case_insensitive(expr, &content)
    } else {
        search(expr, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
