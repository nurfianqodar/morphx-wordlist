pub trait Titlecase {
    fn to_titlecase(&self) -> String;
}

impl<S> Titlecase for S
where
    S: AsRef<str>,
{
    fn to_titlecase(&self) -> String {
        let s = self.as_ref();
        let mut out = String::with_capacity(s.len());
        let mut new_word = true;
        for c in s.chars() {
            if c.is_whitespace() {
                new_word = true;
                out.push(c);
                continue;
            }
            if new_word {
                new_word = false;
                out.push(c.to_ascii_uppercase());
            } else {
                out.push(c.to_ascii_lowercase());
            }
        }
        out
    }
}
