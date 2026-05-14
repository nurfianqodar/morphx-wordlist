use itertools::Itertools;
use rand::seq::IteratorRandom;

pub trait JoinRandomSymbols {
    fn join_random_symbols(&self, symbols: &str) -> String;
}

impl<S> JoinRandomSymbols for &[S]
where
    S: AsRef<str>,
{
    fn join_random_symbols(&self, symbols: &str) -> String {
        let mut rng = rand::rng();
        let sep = symbols.chars().choose(&mut rng).unwrap_or(' ');
        self.iter().map(|s| s.as_ref()).join(&sep.to_string())
    }
}
