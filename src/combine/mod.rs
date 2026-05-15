mod random_symbols;

use crate::error::Error;
use random_symbols::JoinRandomSymbols;
use std::str::FromStr;

pub const COMBINE_LONG_HELP: &str = "\
List of functions used to combine generated
keywords. Values are separated by commas.

Example:
    -c concat,separator:-,random-symbols:_!?
    equal to
    -c c,s:-,r:_!?

Valid combiners:
    c           concat
    s           separator:<TEXT>
    r:<SYMBOLS> random-symbols:<SYMBOLS>

Notes:
    separator:<TEXT>
        Uses the provided text literally as a
        separator.

    random-symbols:<SYMBOLS>
        Randomly selects one character from
        SYMBOLS between combined words.
";

#[derive(Debug, Clone)]
pub enum Combine {
    Concat,
    Separator(String),
    RandomSymbols(Vec<char>),
}

impl Combine {
    pub fn combine(&self, words: &[&str]) -> String {
        match self {
            Self::Concat => words.join(""),
            Self::Separator(sep) => words.join(sep),
            Self::RandomSymbols(syms) => words.join_random_symbols(syms),
        }
    }
}

impl FromStr for Combine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, arg) = s.split_once(':').unwrap_or((s, ""));
        match name {
            "concat" | "c" => {
                if !arg.is_empty() {
                    Err(Error::Arg("concat does not accept argument"))
                } else {
                    Ok(Self::Concat)
                }
            }

            "separator" | "s" => {
                if arg.is_empty() {
                    Err(Error::Arg("separator require an agument"))
                } else {
                    Ok(Self::Separator(arg.to_string()))
                }
            }

            "random-symbols" | "r" => {
                if arg.is_empty() {
                    Err(Error::Arg("random symbol require an agument"))
                } else {
                    Ok(Self::RandomSymbols(arg.chars().collect()))
                }
            }

            _ => Err(Error::Arg("invalid combine")),
        }
    }
}
