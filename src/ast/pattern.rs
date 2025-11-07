use super::descriptor::Descriptor;
use super::expr::Expr;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum PathPattern {
    Node(Descriptor),
    Filter(Box<PathPattern>, Expr),
}

impl PathPattern {
    pub fn new_filter(pattern: PathPattern, expr: Expr) -> Self {
        PathPattern::Filter(Box::new(pattern), expr)
    }
}

impl From<Descriptor> for PathPattern {
    fn from(descriptor: Descriptor) -> Self {
        PathPattern::Node(descriptor)
    }
}

impl fmt::Debug for PathPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PathPattern::Node(desc) => write!(f, "Node({:?})", desc),
            PathPattern::Filter(p, e) => write!(f, "Filter({:?}, {:?})", p, e),
        }
    }
}
