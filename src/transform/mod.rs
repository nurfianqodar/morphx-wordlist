mod leetspeak;
mod sponge;
mod title;

use crate::{
    error::Error,
    transform::{leetspeak::Leetspeak, sponge::Sponge, title::Titlecase},
};
pub use sponge::SpongeVariant;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Transform {
    Original,
    Upper,
    Lower,
    Leet,
    Sponge(SpongeVariant),
    Reverse,
    Title,
}

impl Transform {
    pub fn transform(self, word: &str) -> Option<String> {
        let out = match self {
            Self::Original => word.to_string(),
            Self::Upper => word.to_ascii_uppercase(),
            Self::Lower => word.to_ascii_lowercase(),
            Self::Leet => word.to_leetspeak(),
            Self::Sponge(v) => word.to_sponge(v),
            Self::Reverse => word.chars().rev().collect(),
            Self::Title => word.to_titlecase(),
        };
        if out == word {
            None
        } else {
            Some(out)
        }
    }
}

impl FromStr for Transform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, arg) = s.split_once(':').unwrap_or((s, ""));
        match name {
            "original" => {
                if !arg.is_empty() {
                    Err(Error::Arg("original does not accept argument"))
                } else {
                    Ok(Self::Original)
                }
            }

            "upper" => {
                if !arg.is_empty() {
                    Err(Error::Arg("upper does not accept argument"))
                } else {
                    Ok(Self::Upper)
                }
            }

            "lower" => {
                if !arg.is_empty() {
                    Err(Error::Arg("lower does not accept argument"))
                } else {
                    Ok(Self::Lower)
                }
            }

            "leet" => {
                if !arg.is_empty() {
                    Err(Error::Arg("leet does not accept argument"))
                } else {
                    Ok(Self::Leet)
                }
            }

            "sponge" => {
                if arg.is_empty() {
                    Err(Error::Arg(
                        "sponge require an argunent (lower-first|upper-first|random)",
                    ))
                } else {
                    Ok(Self::Sponge(arg.parse()?))
                }
            }

            "reverse" => {
                if !arg.is_empty() {
                    Err(Error::Arg("reverse does not accept argument"))
                } else {
                    Ok(Self::Reverse)
                }
            }

            "title" => {
                if !arg.is_empty() {
                    Err(Error::Arg("title does not accept argument"))
                } else {
                    Ok(Self::Title)
                }
            }

            _ => Err(Error::Arg("invalid transform")),
        }
    }
}
