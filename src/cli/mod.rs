mod command;
use std::collections::HashSet;

use clap::Parser;
pub use command::Command;

use crate::{combine::Combine, transform::Transform};

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'k', long = "keywords")]
    keywords: Vec<String>,

    #[arg(short = 't', long = "transforms")]
    transforms: Vec<Transform>,

    #[arg(short = 'c', long = "combine")]
    combines: Vec<Combine>,

    #[arg(short = 's', long = "samplers")]
    samplers: Vec<Combine>,

    #[arg(long, required = false)]
    suffixes: Vec<String>,

    #[arg(long)]
    prefixes: Vec<String>,
}

impl Cli {
    pub fn parse_args() -> Self {
        let mut cli = Self::parse();
        cli.keywords = cli
            .keywords
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        cli.prefixes = cli
            .prefixes
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        cli.suffixes = cli
            .suffixes
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        cli
    }
}
