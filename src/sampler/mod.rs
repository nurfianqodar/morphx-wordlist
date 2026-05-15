use crate::error::Error;
use itertools::Itertools;
use std::str::FromStr;

pub const SAMPLER_LONG_HELP: &str = "\
List of sampling strategies used to generate
word combinations. Values are separated by
commas.

Example:
    -s permutation:2,combination:3
    equal to
    -s p:2,c:3

Valid samplers:
    p:<SIZE>    permutation:<SIZE>
    c:<SIZE>    combination:<SIZE>
    C:<SIZE>    cartesian-product:<SIZE>
";

#[derive(Debug, Clone)]
pub enum Sampler {
    Permutation(usize),
    Combination(usize),
    CartesianProduct(usize),
}

impl Sampler {
    pub fn sample_iter<'a>(&self, s: &'a [&'a str]) -> Box<dyn Iterator<Item = Vec<&'a str>> + 'a> {
        match self {
            Self::Permutation(size) => Box::new(
                itertools::Itertools::permutations(s.iter().copied(), *size).map(|v| v.to_vec()),
            ),

            Self::Combination(size) => Box::new(
                itertools::Itertools::combinations(s.iter().copied(), *size).map(|v| v.to_vec()),
            ),

            Self::CartesianProduct(size) => Box::new(
                std::iter::repeat_n(s.iter().copied(), *size)
                    .multi_cartesian_product()
                    .map(|v| v.to_vec()),
            ),
        }
    }
}

impl FromStr for Sampler {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, arg) = s.split_once(':').unwrap_or((s, ""));
        match name {
            "permutation" | "p" => {
                if arg.is_empty() {
                    Err(Error::Arg("permutation sampler require an argument"))
                } else {
                    let size: usize = arg
                        .parse()
                        .map_err(|_| Error::Arg("permutation size must be an integer"))?;
                    Ok(Self::Permutation(size))
                }
            }

            "combination" | "c" => {
                if arg.is_empty() {
                    Err(Error::Arg("combination sampler require an argument"))
                } else {
                    let size: usize = arg
                        .parse()
                        .map_err(|_| Error::Arg("combination size must be an integer"))?;
                    Ok(Self::Combination(size))
                }
            }

            "cartesian-product" | "C" => {
                if arg.is_empty() {
                    Err(Error::Arg("cartesian-product sampler require an argument"))
                } else {
                    let size: usize = arg
                        .parse()
                        .map_err(|_| Error::Arg("cartesian-product size must be an integer"))?;
                    Ok(Self::CartesianProduct(size))
                }
            }

            _ => Err(Error::Arg("invalid sampler")),
        }
    }
}
