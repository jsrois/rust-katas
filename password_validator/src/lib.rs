pub trait Password {
    fn validate(&self) -> bool;
}

impl Password for String {
    fn validate(&self) -> bool {
        self.len() >= 12 &&
            self.chars().any(|c| c.is_lowercase()) &&
            self.chars().any(|c: char| c.is_uppercase()) &&
            self.chars().any(|c: char| c.is_numeric()) &&
            self.chars().any(|c: char| "!@#$%^&*".contains(c))
    }
}

