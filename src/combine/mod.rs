use std::str::FromStr;

use rand::seq::IteratorRandom;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Combine {
    Concat,
    Separator(String),
    RandomSymbols(String),
}

impl Combine {
    pub fn combine(&self, words: &[&str]) -> String {
        match self {
            Self::Concat => words.join(""),
            Self::Separator(sep) => words.join(sep),
            Self::RandomSymbols(syms) => {
                let mut rng = rand::rng();
                let sep = syms
                    .chars()
                    .choose(&mut rng)
                    .expect("syms already specified");
                words.join(&sep.to_string())
            }
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
                    Ok(Self::RandomSymbols(arg.to_string()))
                }
            }

            _ => Err(Error::Arg("invalid combine")),
        }
    }
}
