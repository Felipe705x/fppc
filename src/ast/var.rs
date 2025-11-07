use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Var(pub String);

impl From<&str> for Var {
    fn from(s: &str) -> Self {
        Var(s.to_string())
    }
}

impl From<String> for Var {
    fn from(s: String) -> Self {
        Var(s)
    }
}

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Var({})", self.0)
    }
}
