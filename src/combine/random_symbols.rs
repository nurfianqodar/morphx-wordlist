use itertools::Itertools;
use rand::seq::IteratorRandom;

pub trait JoinRandomSymbols {
    fn join_random_symbols(&self, symbols: &[char]) -> String;
}

impl<S> JoinRandomSymbols for &[S]
where
    S: AsRef<str>,
{
    fn join_random_symbols(&self, symbols: &[char]) -> String {
        let mut rng = rand::rng();
        let sep = symbols.iter().copied().choose(&mut rng).unwrap_or(' ');
        let mut buf = [0_u8; 4]; // i avoid heap allocation with this
        let sep = sep.encode_utf8(&mut buf);
        self.iter().map(|s| s.as_ref()).join(sep)
    }
}
