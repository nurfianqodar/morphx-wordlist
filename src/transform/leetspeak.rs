pub trait Leetspeak {
    fn to_leetspeak(&self) -> String;
}

impl<S> Leetspeak for S
where
    S: AsRef<str>,
{
    fn to_leetspeak(&self) -> String {
        let s = self.as_ref();
        let mut out = String::with_capacity(s.len());
        for c in s.chars() {
            let c = match c {
                'a' | 'A' => '4',
                'e' | 'E' => '3',
                'i' | 'I' => '1',
                'l' | 'L' => '1',
                'o' | 'O' => '0',
                's' | 'S' => '5',
                't' | 'T' => '7',
                'z' | 'Z' => '2',
                _ => c,
            };
            out.push(c);
        }
        out
    }
}
