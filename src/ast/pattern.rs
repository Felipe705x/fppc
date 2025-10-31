use std::fmt;
use super::descriptor::Descriptor;
use super::expr::Expr;

pub enum PathPattern {
    Node(NodePattern),
    Filter(Box<PathPattern>, Expr),
}

impl PathPattern {
    pub fn new_node(descriptor: Descriptor) -> Self {
        PathPattern::Node(NodePattern::new(descriptor))
    }

    pub fn new_filter(pattern: PathPattern, expr: Expr) -> Self {
        PathPattern::Filter(Box::new(pattern), expr)
    }
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

impl NodePattern {
    pub fn new(descriptor: Descriptor) -> Self {
        NodePattern { descriptor }
    }
}

impl fmt::Debug for NodePattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", self.descriptor)
    }
}
