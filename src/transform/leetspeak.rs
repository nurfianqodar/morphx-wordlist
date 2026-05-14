pub trait Leetspeak {
    fn to_leetspeak(&self) -> String;
}

impl<S> Leetspeak for S
where
    S: AsRef<str>,
{
    fn to_leetspeak(&self) -> String {
        let mut out = String::with_capacity(self.as_ref().len());
        self.as_ref().chars().for_each(|c| {
            let c = match c.to_ascii_lowercase() {
                'a' => '4',
                'e' => '3',
                'i' => '1',
                'l' => '1',
                'o' => '0',
                's' => '5',
                't' => '7',
                'z' => '2',
                _ => c,
            };
            out.push(c);
        });
        out
    }
}
