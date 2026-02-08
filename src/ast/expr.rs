use super::types::SimpleType;
use super::var::Var;
use std::fmt;

/// Base enum for all expressions in the query language.
/// Expressions are used in filters (e.g., `WHERE` clauses).
#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Constant(Constant),
    TypeLiteral(SimpleType),
    AttributeLookup(Var, Var),
    Binop(BinOpKind, Box<Expr>, Box<Expr>),
    Unop(UnOpKind, Box<Expr>),
}

impl Expr {
    pub fn binop(kind: BinOpKind, left: Expr, right: Expr) -> Self {
        Expr::Binop(kind, Box::new(left), Box::new(right))
    }

    pub fn unop(kind: UnOpKind, expr: Expr) -> Self {
        Expr::Unop(kind, Box::new(expr))
    }

    /// Creates `entity.attribute` lookup expression
    pub fn attr_lookup(entity: Var, attribute: Var) -> Self {
        Expr::AttributeLookup(entity, attribute)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Constant(c) => write!(f, "{}", c),
            Expr::TypeLiteral(t) => write!(f, "{}", t),
            Expr::AttributeLookup(e, a) => write!(f, "{}.{}", e.0, a.0),
            Expr::Binop(op, e1, e2) => write!(f, "({} {} {})", e1, op, e2),
            Expr::Unop(op, e) => write!(f, "{} {}", op, e),
        }
    }
}

/// Represents a constant expression (string, int, or boolean).
#[derive(PartialEq, Clone)]
pub enum Constant {
    String(String),
    Int(i64),
    Bool(bool),
}

impl From<String> for Constant {
    fn from(s: String) -> Self {
        Constant::String(s)
    }
}

impl From<&str> for Constant {
    fn from(s: &str) -> Self {
        Constant::String(s.to_string())
    }
}

impl From<i64> for Constant {
    fn from(i: i64) -> Self {
        Constant::Int(i)
    }
}

impl From<bool> for Constant {
    fn from(b: bool) -> Self {
        Constant::Bool(b)
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::String(s) => write!(f, "{}", s),
            Constant::Int(i) => write!(f, "{}", i),
            Constant::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl fmt::Debug for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constant::String(s) => write!(f, "'{}'", s),
            Constant::Int(i) => write!(f, "{}", i),
            Constant::Bool(b) => write!(f, "{}", b),
        }
    }
}

/// Binary operator kinds
#[derive(PartialEq, Clone, Debug)]
pub enum BinOpKind {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // Comparison
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
    // Logical
    And,
    Or,
    // Type operations
    Is,
    As,
}

impl fmt::Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinOpKind::Add => write!(f, "+"),
            BinOpKind::Sub => write!(f, "-"),
            BinOpKind::Mul => write!(f, "*"),
            BinOpKind::Div => write!(f, "/"),
            BinOpKind::Lt => write!(f, "<"),
            BinOpKind::Gt => write!(f, ">"),
            BinOpKind::Le => write!(f, "<="),
            BinOpKind::Ge => write!(f, ">="),
            BinOpKind::Eq => write!(f, "="),
            BinOpKind::Ne => write!(f, "!="),
            BinOpKind::And => write!(f, "AND"),
            BinOpKind::Or => write!(f, "OR"),
            BinOpKind::Is => write!(f, "IS"),
            BinOpKind::As => write!(f, "AS"),
        }
    }
}

/// Unary operator kinds
#[derive(PartialEq, Clone, Debug)]
pub enum UnOpKind {
    Neg, // -
    Not, // not
}

impl fmt::Display for UnOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnOpKind::Neg => write!(f, "-"),
            UnOpKind::Not => write!(f, "NOT"),
        }
    }
}
