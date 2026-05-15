use crate::result::Result;
use crate::{combine::Combine, sampler::Sampler, transform::Transform};
use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufWriter};
use std::rc::Rc;

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
    - [s:<SPONGE_VARIANT>] | sponge:<SPONGE_VARIANT>
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
    - [s:<SEPARATOR>] | separator:<SEPARATOR>
    - [r:<SYMBOLS>] | random-symbols:<SYMBOLS>
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

const MIN_HELP: &str = "minimum length of generated keywords";

const WRITE_HELP: &str = "write file path (default: stdout)";

#[derive(Debug, Parser)]
#[command(version, about, author, long_about = None)]
pub struct Command {
    #[arg(long, short, required = true, value_delimiter = ',',
        help = KEYWORDS_HELP, long_help = KEYWORDS_LONG_HELP)]
    keywords: Vec<String>,

    #[arg(long, short, required = true, value_delimiter = ',',
        help = TRANSFORM_HELP, long_help = TRANSFORM_LONG_HELP)]
    transform: Vec<Transform>,

    #[arg(long, short, required = true, value_delimiter = ',',
        help = COMBINE_HELP, long_help = COMBINE_LONG_HELP)]
    combine: Vec<Combine>,

    #[arg(long, short = 'S', required = true, value_delimiter = ',',
        help = SAMPLER_HELP, long_help= SAMPLER_LONG_HELP)]
    sampler: Vec<Sampler>,

    #[arg(long, short, required = false, value_delimiter = ',',
        help = PREFIX_HELP, long_help = PREFIX_LONG_HELP)]
    prefix: Option<Vec<String>>,

    #[arg(long, short, required = false, value_delimiter = ',',
        help = SUFFIX_HELP, long_help = SUFFIX_LONG_HELP)]
    suffix: Option<Vec<String>>,

    #[arg(long, short, required = false, help = MIN_HELP)]
    min: Option<usize>,

    #[arg(long, short, required = false, help = WRITE_HELP)]
    write: Option<String>,
}

impl Command {
    pub fn run(&mut self) -> Result<()> {
        self.wirte()?;
        Ok(())
    }

    pub fn m_parse() -> Self {
        let mut me = Self::parse();
        me.transform_keywords();
        me
    }

    fn wirte(&mut self) -> Result<()> {
        self.transform_keywords();
        let keywords = self.keywords();
        let mut writer = self.writer()?;
        for sampler in &self.sampler {
            for combine in &self.combine {
                for sub in sampler.sample_iter(&keywords) {
                    write_combine(
                        &mut writer,
                        sub.as_ref(),
                        combine,
                        self.min,
                        self.prefix.as_deref(),
                        self.suffix.as_deref(),
                    )?;
                }
            }
        }
        Ok(())
    }

    fn writer(&self) -> Result<Box<dyn io::Write>> {
        if let Some(path) = &self.write {
            let f = File::create_new(path)?;
            Ok(Box::new(BufWriter::new(f)))
        } else {
            Ok(Box::new(io::stdout()))
        }
    }

    fn transform_keywords(&mut self) {
        let mut set = HashSet::<String>::with_capacity(self.keywords.len());
        self.keywords.iter().for_each(|s| {
            set.insert(s.to_string());
            for t in &self.transform {
                if let Some(word) = t.transform(s.as_ref()) {
                    set.insert(word);
                }
            }
        });
        self.keywords.clear();
        self.keywords.extend(set.into_iter());
    }

    fn keywords(&self) -> Rc<[&str]> {
        self.keywords
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .into()
    }
}

// wirite combined sub with combine
// if any prefixes write with prefix
// if any suffixes write wirh suffix
// if min specified this function not write while
// word length + prefix length < min
// or word length + suffix length < min
fn write_combine<W>(
    writer: &mut W,
    sub: &[&str],
    combine: &Combine,
    min: Option<usize>,
    prefixes: Option<&[String]>,
    suffixes: Option<&[String]>,
) -> Result<()>
where
    W: io::Write,
{
    let word = combine.combine(sub);
    if let Some(m) = min {
        if word.len() >= m {
            writeln!(writer, "{}", word)?;
        }
    } else {
        writeln!(writer, "{}", word)?;
    }

    if let Some(pref) = prefixes {
        for p in pref {
            write_with_prefix(writer, &word, p, min)?;
        }
    }

    if let Some(suff) = suffixes {
        for s in suff {
            write_with_suffix(writer, &word, s, min)?;
        }
    }
    Ok(())
}

// write word with prefix
// if min specified this function not write while
// word leng + prefix len < min
fn write_with_prefix<W>(writer: &mut W, word: &str, prefix: &str, min: Option<usize>) -> Result<()>
where
    W: io::Write,
{
    if let Some(m) = min {
        if (word.len() + prefix.len()) >= m {
            writeln!(writer, "{}{}", prefix, word)?;
        }
    } else {
        writeln!(writer, "{}{}", prefix, word)?;
    }
    Ok(())
}

// write word with suffix
// if min specified this function not write while
// word leng + suffix len < min
fn write_with_suffix<W>(writer: &mut W, word: &str, suffix: &str, min: Option<usize>) -> Result<()>
where
    W: io::Write,
{
    if let Some(m) = min {
        if (word.len() + suffix.len()) >= m {
            writeln!(writer, "{}{}", word, suffix)?;
        }
    } else {
        writeln!(writer, "{}{}", word, suffix)?;
    }
    Ok(())
}
