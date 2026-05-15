use crate::error::Error;
use rand::RngExt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum SpongeVariant {
    LowerFirst,
    UpperFirst,
    Random,
}

impl FromStr for SpongeVariant {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lower-first" | "l" => Ok(Self::LowerFirst),
            "upper-first" | "u" => Ok(Self::UpperFirst),
            "random" | "r" => Ok(Self::Random),
            _ => Err(Error::Arg(
                "invalid sponge variant (use: lower-first|upper-first|random)",
            )),
        }
    }
}

impl SpongeVariant {
    fn should_uppercase(&self, idx: usize) -> bool {
        let is_even = idx % 2 == 0;
        match self {
            Self::UpperFirst => is_even,
            Self::LowerFirst => !is_even,
            Self::Random => rand::random(),
        }
    }
}

pub trait Sponge {
    fn to_sponge(&self, variant: SpongeVariant) -> String;
}

impl<S> Sponge for S
where
    S: AsRef<str>,
{
    fn to_sponge(&self, variant: SpongeVariant) -> String {
        let s = self.as_ref();
        let mut out = String::with_capacity(s.len());
        for (i, c) in s.chars().enumerate() {
            let c = if variant.should_uppercase(i) {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            };
            out.push(c);
        }
        out
    }
}
