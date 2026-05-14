use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufWriter};

use crate::result::Result;
use crate::{combine::Combine, sampler::Sampler, transform::Transform};
use clap::Parser;

/// morphx: Keywords based wordlist generator
#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Command {
    /// Base keywords
    ///
    /// Format: [,<KEYWORD>]
    ///
    /// KEYWORD: any string
    #[arg(long, short, required = true, value_delimiter = ',')]
    keywords: Vec<String>,

    /// Keywords transforms options
    ///
    /// Format: <TRANSFORM>[,<TRANSFORM>]
    ///
    /// TRANSFORM:
    ///     original|upper|lower|leet|sponge:<SPONGE_VARIANT>|reverse|title
    ///
    /// SPONGE_VARIANT:
    ///     lower-first|upper-first|random
    #[arg(long, short, required = true, value_delimiter = ',')]
    transform: Vec<Transform>,

    /// Keywords combine options
    ///
    /// Format: <COMBINE>[,<COMBINE>]
    ///
    /// COMBINE: concat|separator:<SEPARATOR>|random-symbols:<SYMBOLS>
    ///
    /// SEPARATOR: any string
    ///
    /// SYMBOLS: any string
    #[arg(long, short, required = true, value_delimiter = ',')]
    combine: Vec<Combine>,

    /// Keywords sampling options
    ///
    /// Format: <SAMPLER>[,<SAMPLER>]
    ///
    /// SAMPLER: permutation:<SIZE>|combination:<SIZE>|cartesian-product:<SIZE>
    ///
    /// SIZE: non negative integer
    #[arg(long, short = 'S', required = true, value_delimiter = ',')]
    sampler: Vec<Sampler>,

    /// Prefixes to add after combined
    #[arg(long, short, required = false, value_delimiter = ',')]
    prefix: Option<Vec<String>>,

    /// Suffixes to add after combined
    ///
    /// Format: <SUFFIX>[,<SUFFIX>]
    #[arg(long, short, required = false, value_delimiter = ',')]
    suffix: Option<Vec<String>>,

    /// Write output to file
    ///
    /// Format:
    ///     -s <FILE_PATH>
    #[arg(long, short, required = false)]
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
