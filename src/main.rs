use clap::Parser;
use morphx_wordlist::cli::Command;

fn main() {
    let cmd = Command::parse();
    match cmd.run() {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
