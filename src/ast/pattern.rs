use super::descriptor::Descriptor;
use super::expr::Expr;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum PathPattern {
    Node(Descriptor),
    Filter(Box<PathPattern>, Expr),
    Edge(EdgeDirection, Descriptor),
}

impl PathPattern {
    pub fn filter(pattern: PathPattern, expr: Expr) -> Self {
        PathPattern::Filter(Box::new(pattern), expr)
    }

    pub fn edge(dir: EdgeDirection, desc: Descriptor, filter: Option<Expr>) -> Self {
        let edge = PathPattern::Edge(dir, desc);
        match filter {
            None => edge,
            Some(expr) => PathPattern::filter(edge, expr),
        }
    }

    pub fn node(desc: Descriptor, filter: Option<Expr>) -> Self {
        let node = PathPattern::Node(desc);
        match filter {
            None => node,
            Some(expr) => PathPattern::filter(node, expr),
        }
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
            PathPattern::Edge(dir, desc) => write!(f, "Edge({:?}, {:?})", dir, desc),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EdgeDirection {
    Right,
    Left,
    None,
    Any,
}
