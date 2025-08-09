use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "spider")]
#[command(about = "A Rust image scraper", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long, requires = "recursive", default_value_t = 5)]
    limit: u32,

    #[arg(short, long, default_value = "./data")]
    path: Box<Path>,

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

fn check_path(path: &Path) -> Result<(), String> {
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