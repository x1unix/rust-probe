use probe::ugrep::{self, Args};
use std::error::Error;
use std::process;

fn die(msg: &str) {
    eprintln!("Error: {msg}");
    process::exit(1);
}

fn main() {
    if let Err(err) = run() {
        die(format!("{err}").as_str());
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse()?;
    ugrep::run(&args)?;
    Ok(())
}
