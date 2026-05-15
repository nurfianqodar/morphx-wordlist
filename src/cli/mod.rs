use crate::combine::Combine;
use crate::result::Result;
use crate::sampler::Sampler;
use crate::transform::Transform;
use clap::Parser;
use std::fmt::Arguments;
use std::fs::File;
use std::io;
use std::io::Write;

use crate::combine::COMBINE_LONG_HELP;
use crate::sampler::SAMPLER_LONG_HELP;
use crate::transform::TRANSFORM_LONG_HELP;
const KEYWORDS_LONG_HELP: &str = "\
List of base keywords. Values are separated
by commas.

Example:
    --keywords foo,bar,baz
    equal to
    -k foo,bar,baz";
const PREFIXES_LONG_HELP: &str = "\
List of prefixes applied before output.
Values are separated by commas.

Example:
    --prefixes admin,sys,prod

Note:
    Applying prefixes does not remove the
    original generated value.
";
const SUFFIXES_LONG_HELP: &str = "\
List of suffixes applied before output.
Values are separated by commas.

Example:
    --suffixes 123,2026,**

Note:
    Applying suffixes does not remove the
    original generated value.
";
pub const MIN_LONG_HELP: &str = "\
Minimum length required for generated words
to be written to output. This filter is
applied after all transformations,
combinations, prefixes, and suffixes are
processed.

Example:
    --min 8
    equal to
    -m 8

Notes:
    Values shorter than the minimum length
    are skipped. When not specified, no
    minimum length filter is applied.
";
pub const WRITE_HELP: &str = "\
    Write to a file instead of stdout
";

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, required = true, value_delimiter = ',', long_help = KEYWORDS_LONG_HELP)]
    keywords: Vec<String>,

    #[arg(short, long, required = true, value_delimiter = ',', long_help = TRANSFORM_LONG_HELP)]
    transforms: Vec<Transform>,

    #[arg(short, long, required = true, value_delimiter = ',', long_help = COMBINE_LONG_HELP)]
    combines: Vec<Combine>,

    #[arg(short, long, required = true, value_delimiter = ',', long_help = SAMPLER_LONG_HELP)]
    samplers: Vec<Sampler>,

    #[arg(long, required = false, value_delimiter = ',', long_help = SUFFIXES_LONG_HELP)]
    suffixes: Vec<String>,

    #[arg(long, required = false, value_delimiter = ',', long_help = PREFIXES_LONG_HELP)]
    prefixes: Vec<String>,

    #[arg(short, long, required = false, long_help = MIN_LONG_HELP)]
    min: Option<usize>,

    #[arg(short, long, required = false, help = WRITE_HELP)]
    write: Option<String>,
}

impl Cli {
    pub fn sanitize(mut self) -> Self {
        self.keywords.sort_unstable();
        self.keywords.dedup();

        // sanitize and transform
        let mut sanitized_keywords = Vec::new();
        for word in self.keywords.drain(..) {
            for t in &self.transforms {
                if let Some(transformed) = t.transform(&word) {
                    sanitized_keywords.push(transformed);
                }
            }
            sanitized_keywords.push(word);
        }
        sanitized_keywords.sort_unstable();
        sanitized_keywords.dedup();
        self.keywords = sanitized_keywords;

        self.prefixes.sort_unstable();
        self.prefixes.dedup();

        self.suffixes.sort_unstable();
        self.suffixes.dedup();

        self
    }

    pub fn run(&self) -> Result<()> {
        let keywords = self.keywords.iter().map(|w| w.as_str()).collect::<Vec<_>>();
        for sampler in &self.samplers {
            for combine in &self.combines {
                self.generate_streaming(sampler, combine, &keywords)?;
            }
        }
        Ok(())
    }

    fn generate_streaming(
        &self,
        sampler: &Sampler,
        combine: &Combine,
        keywords: &[&str],
    ) -> Result<()> {
        let mut writer: Box<dyn Write> = match &self.write {
            Some(path) => Box::new(File::create_new(path)?),
            None => Box::new(io::stdout()),
        };

        for sample in sampler.sample_iter(keywords) {
            let word = combine.combine(&sample);
            self.write(&mut writer, &word)?;
        }

        Ok(())
    }

    fn write<W>(&self, writer: &mut W, word: &str) -> Result<()>
    where
        W: io::Write,
    {
        emit(writer, word.len(), format_args!("{word}"), self.min)?;

        for p in &self.prefixes {
            emit(
                writer,
                word.len() + p.len(),
                format_args!("{p}{word}"),
                self.min,
            )?;
        }

        for s in &self.suffixes {
            emit(
                writer,
                word.len() + s.len(),
                format_args!("{word}{s}"),
                self.min,
            )?;
        }

        for p in &self.prefixes {
            for s in &self.suffixes {
                emit(
                    writer,
                    word.len() + p.len() + s.len(),
                    format_args!("{p}{word}{s}"),
                    self.min,
                )?;
            }
        }
        Ok(())
    }
}

fn emit<W>(writer: &mut W, len: usize, args: Arguments<'_>, min: Option<usize>) -> Result<()>
where
    W: io::Write,
{
    if min.is_none_or(|m| m > len) {
        writer.write_fmt(args)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}
