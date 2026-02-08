use super::descriptor::Descriptor;
use super::expr::Expr;
use std::fmt;

/// Quantifiers for path pattern repetition
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Quantifier {
    /// `*` - zero or more (equivalent to {0,})
    Star,
    /// `+` - one or more (equivalent to {1,})
    Plus,
    /// `{n}` - exactly n times
    Fixed(u64),
    /// `{m,n}` - between m and n times (both bounds optional)
    Range(Option<u64>, Option<u64>),
}

#[derive(PartialEq, Clone)]
pub enum PathPattern {
    Node(Descriptor),
    Filter(Box<PathPattern>, Expr),
    Edge(EdgeDirection, Descriptor),
    Concat(Box<PathPattern>, Box<PathPattern>),
    Union(Box<PathPattern>, Box<PathPattern>),
    /// `p?` - optional pattern (zero or one)
    Questioned(Box<PathPattern>),
    /// `p*`, `p+`, `p{n}`, `p{m,n}` - quantified pattern
    Quantified(Box<PathPattern>, Quantifier),
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

    pub fn questioned(pattern: PathPattern) -> Self {
        PathPattern::Questioned(Box::new(pattern))
    }

    pub fn quantified(pattern: PathPattern, quantifier: Quantifier) -> Self {
        PathPattern::Quantified(Box::new(pattern), quantifier)
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
            PathPattern::Questioned(p) => write!(f, "Questioned({:?})", p),
            PathPattern::Quantified(p, q) => write!(f, "Quantified({:?}, {:?})", p, q),
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
