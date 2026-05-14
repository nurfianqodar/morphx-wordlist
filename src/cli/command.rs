use clap::Parser;

use crate::transform::Transform;

#[derive(Debug, Parser)]
pub struct Command {
    #[arg(long, short, value_delimiter = ',')]
    transform: Vec<Transform>,
}
