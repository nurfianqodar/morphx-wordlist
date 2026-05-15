use clap::Parser;
use morphx_wordlist::cli::Cli;

fn main() {
    let cli = Cli::parse().sanitize();
    match cli.run() {
        Err(e) => eprintln!("{e}"),
        _ => (),
    }
}
