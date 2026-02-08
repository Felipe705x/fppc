#[derive(PartialEq, Clone, Debug)]
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
