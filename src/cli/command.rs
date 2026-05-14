use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufWriter};

use crate::result::Result;
use crate::{combine::Combine, sampler::Sampler, transform::Transform};
use clap::Parser;

const KEYWORDS_HELP: &str = "base keywords";
const KEYWORDS_LONG_HELP: &str = r#"base keyowrds
format:
    [,<KEYWORD>]
KEYWORD:
    any string"#;

const TRANSFORM_HELP: &str = "keyowrds transform options";
const TRANSFORM_LONG_HELP: &str = r#"keyowrds transform options
format:
    <TRANSFORM>[,<TRANSFORM>]
TRANSFORM:
    - [o] | original
    - [u] | upper
    - [l] | lower
    - [L] | leet
    - [s] | sponge:<SPONGE_VARIANT>
    - [r] | reverse
    - [t] | title
SPONGE_VARIANT:
    lower-first|upper-first|random"#;

const COMBINE_HELP: &str = "keywords combine options";
const COMBINE_LONG_HELP: &str = r#"keyowrds combine options
format:
    <COMBINE>[,<COMBINE>]
COMBINE:
    - [c] | concat
    - [s] | separator:<SEPARATOR>
    - [r] | random-symbols:<SYMBOLS>
SEPARATOR:
    any string
SYMBOLS:
    any string"#;

const SAMPLER_HELP: &str = "keyowrds sampling options";
const SAMPLER_LONG_HELP: &str = "keyowrds sampling options
format:
    <SAMPLER>[,<SAMPLER>]
SAMPLER:
    - [p] | permutation<SIZE>
    - [c] | combination:<SIZE>
    - [C] | cartesian-product:<SIZE>
SIZE:
    non-zero and non-negative integer";

const PREFIX_HELP: &str = "prefixes applied after combine";
const PREFIX_LONG_HELP: &str = r#"prefixes applied after combine
format:
    <PREFIX>[,<PREFIX>]
PREFIX:
    any string"#;

const SUFFIX_HELP: &str = "suffixes applied after combine";
const SUFFIX_LONG_HELP: &str = r#"suffixes applied after combine
format:
    <SUFFIX>[,<SUFFIX>]
SUFFIX:
    any string"#;

const WRITE_HELP: &str = "write file path (default: stdout)";

#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Command {
    #[arg(
        long,
        short,
        required = true,
        value_delimiter = ',',
        help = KEYWORDS_HELP,
        long_help = KEYWORDS_LONG_HELP,
    )]
    keywords: Vec<String>,

    #[arg(
        long,
        short,
        required = true,
        value_delimiter = ',',
        help = TRANSFORM_HELP,
        long_help = TRANSFORM_LONG_HELP,
    )]
    transform: Vec<Transform>,

    #[arg(
        long,
        short,
        required = true,
        value_delimiter = ',',
        help = COMBINE_HELP,
        long_help = COMBINE_LONG_HELP,
    )]
    combine: Vec<Combine>,

    #[arg(
        long,
        short = 'S',
        required = true,
        value_delimiter = ',',
        help = SAMPLER_HELP,
        long_help= SAMPLER_LONG_HELP,
    )]
    sampler: Vec<Sampler>,

    #[arg(
        long,
        short,
        required = false,
        value_delimiter = ',',
        help = PREFIX_HELP,
        long_help = PREFIX_LONG_HELP,
    )]
    prefix: Option<Vec<String>>,

    #[arg(
        long,
        short,
        required = false,
        value_delimiter = ',',
        help = SUFFIX_HELP,
        long_help = SUFFIX_LONG_HELP,
    )]
    suffix: Option<Vec<String>>,

    #[arg(
        long,
        short,
        required = false,
        help = WRITE_HELP
    )]
    write: Option<String>,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        let mut writer: Box<dyn io::Write>;
        if let Some(path) = &self.write {
            let f = File::create_new(path)?;
            writer = Box::new(BufWriter::new(f));
        } else {
            writer = Box::new(io::stdout());
        }

        let mut keyword_set = HashSet::<String>::new();
        // Transform keywords
        self.keywords.iter().for_each(|s| {
            keyword_set.insert(s.to_string());
            for t in &self.transform {
                if let Some(word) = t.transform(s.as_ref()) {
                    keyword_set.insert(word);
                }
            }
        });

        let keywords = keyword_set
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();

        for s in &self.sampler {
            for c in &self.combine {
                for sub in s.sample_iter(&keywords.clone()) {
                    let word = c.combine(&sub);
                    writeln!(&mut writer, "{}", word)?;

                    if let Some(prefixes) = &self.prefix {
                        for prefix in prefixes {
                            writeln!(&mut writer, "{}{}", prefix, word)?;
                        }
                    }

                    if let Some(suffixes) = &self.suffix {
                        for suffix in suffixes {
                            writeln!(&mut writer, "{}{}", word, suffix)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
