pub trait Titlecase {
    fn to_titlecase(&self) -> String;
}

impl<S> Titlecase for S
where
    S: AsRef<str>,
{
    fn to_titlecase(&self) -> String {
        self.as_ref()
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }
}
