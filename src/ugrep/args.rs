use std::env;
#[derive(Debug)]
pub struct Args {
    pub file_name: String,
    pub expr: String,
    pub case_insensitive: bool,
}

impl Args {
    pub fn parse() -> Result<Self, &'static str> {
        // TODO: refactor into TryFrom

        let case_insensitive = env::var("IGNORE_CASE")
            .map(|s| s.parse::<bool>().unwrap_or(false))
            .unwrap_or(false);

        let args: Vec<String> = env::args().skip(1).take(2).collect();
        if args.len() != 2 {
            return Err("not enough arguments. Usage: ugrep [expr] file_name");
        }

        let [expr, file_name]: [String; 2] = args.try_into().map_err(|_| "can't read arguments")?;
        Ok(Args {
            file_name,
            expr,
            case_insensitive,
        })
    }
}
