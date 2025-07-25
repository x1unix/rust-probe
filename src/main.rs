use std::env;

pub mod kata;
pub mod ugrep;

struct Args(String, String);

impl Args {
    fn parse() -> Option<Self> {
        let args: Vec<String> = env::args().skip(1).take(2).collect();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    for (i, arg) in args.iter().enumerate() {
        println!("arg{{{i}}}: {arg}");
    }
}
