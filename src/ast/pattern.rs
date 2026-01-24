use super::descriptor::Descriptor;
use super::expr::Expr;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum PathPattern {
    Node(Descriptor),
    Filter(Box<PathPattern>, Expr),
    Edge(EdgeDirection, Descriptor),
    Concat(Box<PathPattern>, Box<PathPattern>),
    Union(Box<PathPattern>, Box<PathPattern>),
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

    pub fn concat(left: PathPattern, right: PathPattern) -> Self {
        PathPattern::Concat(Box::new(left), Box::new(right))
    }

    pub fn union(left: PathPattern, right: PathPattern) -> Self {
        PathPattern::Union(Box::new(left), Box::new(right))
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
            PathPattern::Concat(l, r) => write!(f, "Concat({:?}, {:?})", l, r),
            PathPattern::Union(l, r) => write!(f, "Union({:?}, {:?})", l, r),
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
