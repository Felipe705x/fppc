use std::fmt;
use super::descriptor::Descriptor;
use super::expr::Expr;

pub enum PathPattern {
    Node(NodePattern),
    Filter(Box<PathPattern>, Expr),
}

impl fmt::Debug for PathPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PathPattern::Node(n) => write!(f, "{:?}", n),
            PathPattern::Filter(p, e) => write!(f, "({:?} WHERE {:?})", p, e),
        }
    }
}

pub struct NodePattern {
    pub descriptor: Descriptor,
}

impl fmt::Debug for NodePattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", self.descriptor)
    }
}
