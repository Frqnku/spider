mod parse_args;
use parse_args::Cli;

fn main() {
    let cli = Cli::new();
    println!("{:?}", cli);
    if let Err(err) = cli.check_path() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}