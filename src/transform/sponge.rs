use std::str::FromStr;

use rand::RngExt;

use crate::error::Error;

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
            "lower-first" => Ok(Self::LowerFirst),
            "upper-first" => Ok(Self::UpperFirst),
            "random" => Ok(Self::Random),
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
            Self::Random => {
                let mut rng = rand::rng();
                rng.random_bool(0.5)
            }
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
        let mut out = String::with_capacity(self.as_ref().len());
        for (i, c) in self.as_ref().chars().enumerate() {
            if variant.should_uppercase(i) {
                out.push(c.to_ascii_uppercase());
            } else {
                out.push(c.to_ascii_lowercase());
            }
        }
        out
    }
}
