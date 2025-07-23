use std::env;

pub mod kata;
pub mod ugrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    for (i, arg) in args.iter().enumerate() {
        println!("arg{{{i}}}: {arg}");
    }
}
