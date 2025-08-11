use clap::Parser;
use std::path::PathBuf;

fn positive_usize(s: &str) -> Result<usize, String> {
    let value = s
        .parse::<usize>()
        .map_err(|_| format!("`{s}` is not a valid positive integer"))?;
    if value == 0 {
        Err(String::from("Value must be greater than 0"))
    } else {
        Ok(value)
    }
}

#[derive(Parser, Debug)]
#[command(name = "spider")]
#[command(about = "A Rust image scraper", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub recursive: bool,

    #[arg(short, long, requires = "recursive", default_value_t = 5, value_parser = positive_usize)]
    pub limit: usize,

    #[arg(short, long, default_value = "./data")]
    pub path: PathBuf,

    pub url: String,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn check_path(&self) -> Result<(), String> {
        check_path(&self.path)
    }
}

fn check_path(path: &PathBuf) -> Result<(), String> {
    match path {
        p if !p.exists() => Err(format!("The path {} does not exist.", p.display())),
        p if !p.is_dir() => Err(format!(
            "The path {} is not a valid directory.",
            p.display()
        )),
        p if p.metadata().is_err() || p.metadata().unwrap().permissions().readonly() => Err(
            format!("The path {} does not have write permissions.", p.display()),
        ),
        _ => Ok(()),
    }
}
