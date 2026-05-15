use clap::Parser;
use morphx_wordlist::cli::Cli;

fn main() {
    let cli = Cli::parse().sanitize();
    if let Err(e) = cli.run() {
        eprintln!("{e}");
    }
}
