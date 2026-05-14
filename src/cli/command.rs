use clap::Parser;

use crate::{combine::Combine, transform::Transform};

#[derive(Debug, Parser)]
pub struct Command {
    #[arg(long, short, value_delimiter = ',')]
    transform: Vec<Transform>,

    #[arg(long, short, value_delimiter = ',')]
    combine: Vec<Combine>,
}
