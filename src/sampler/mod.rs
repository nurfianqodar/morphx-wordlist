use std::{rc::Rc, str::FromStr};

use itertools::Itertools;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Sampler {
    Permutation(usize),
    Combination(usize),
    CartesianProduct(usize),
}

impl Sampler {
    pub fn sample_iter<'a>(
        &self,
        s: &'a [&'a str],
    ) -> Box<dyn Iterator<Item = Rc<[&'a str]>> + 'a> {
        match self {
            Self::Permutation(size) => Box::new(
                itertools::Itertools::permutations(s.iter().copied(), *size)
                    .map(|v| Rc::<[&str]>::from(v)),
            ),

            Self::Combination(size) => Box::new(
                itertools::Itertools::combinations(s.iter().copied(), *size)
                    .map(|v| Rc::<[&str]>::from(v)),
            ),

            Self::CartesianProduct(size) => Box::new(
                std::iter::repeat_n(s.iter().copied(), *size)
                    .multi_cartesian_product()
                    .map(|v| Rc::<[&str]>::from(v)),
            ),
        }
    }
}

impl FromStr for Sampler {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, arg) = s.split_once(':').unwrap_or((s, ""));
        match name {
            "permutation" => {
                if arg.is_empty() {
                    Err(Error::Arg("permutation sampler require an argument"))
                } else {
                    let size: usize = arg
                        .parse()
                        .map_err(|_| Error::Arg("permutation size must be an integer"))?;
                    Ok(Self::Permutation(size))
                }
            }

            "combination" => {
                if arg.is_empty() {
                    Err(Error::Arg("combination sampler require an argument"))
                } else {
                    let size: usize = arg
                        .parse()
                        .map_err(|_| Error::Arg("combination size must be an integer"))?;
                    Ok(Self::Combination(size))
                }
            }

            "cartesian-product" => {
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
