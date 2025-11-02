use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Var(pub String);

impl fmt::Debug for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Var({})", self.0)
    }
}

