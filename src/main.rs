use std::{env, error::Error, fs, process};

use minigrep::{search, search_case_insensitive};

fn main() {
    let config = Config::builds(env::args()).unwrap_or_else(|err| {
        eprintln!("Masalah ketika memparsing argumen: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Aplikasi error: {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn builds(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Tidak ada argumen untuk query"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Tidak ada argumen untuk file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
