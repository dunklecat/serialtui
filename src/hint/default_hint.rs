pub struct DefaultHint {
    pub hint: String,
    pub complete_up_to: usize,
}

impl rustyline::hint::Hint for DefaultHint {
    fn display(&self) -> &str {
        self.hint.as_str()
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.hint[..self.complete_up_to])
        } else {
            None
        }
    }
}
