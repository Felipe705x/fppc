use super::types::SimpleType;
use super::var::Var;
use std::fmt;

/// Base enum for all expressions in the query language.
/// Expressions are used in filters (e.g., WHERE clauses).
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Constant(Constant),
    Variable(Var),
    TypeLiteral(SimpleType),
    AttributeLookup(AttributeLookup),
    Binop(Binop),
    Unop(Unop),
}

/// Represents a constant expression (string, int, or boolean).
#[derive(PartialEq, Clone)]
pub enum Constant {
    /// A constant string value (SConstant in Python)
    String(String),
    /// A constant integer (Z) value (ZConstant in Python)
    Int(i64),
    /// A constant boolean value
    Bool(bool),
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


/// Expression of the form `e.a` that accesses the attribute `a` of the entity `e`.
#[derive(Debug, PartialEq, Clone)]
pub struct AttributeLookup {
    pub e: Var,
    pub a: Var,
}

impl AttributeLookup {
    pub fn new(e: Var, a: Var) -> Self {
        AttributeLookup { e, a }
    }
}

impl fmt::Display for AttributeLookup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.e.0, self.a.0)
    }
}

/// Binary operator kinds
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
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

/// Binary operation expression, e.g., x + y, x = y, x < y.
#[derive(Debug, PartialEq, Clone)]
pub struct Binop {
    pub op: BinOpKind,
    pub e1: Box<Expr>,
    pub e2: Box<Expr>,
}

impl Binop {
    pub fn new(op: BinOpKind, e1: Expr, e2: Expr) -> Self {
        Binop {
            op,
            e1: Box::new(e1),
            e2: Box::new(e2),
        }
    }
}

impl fmt::Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {} {})", self.e1, self.op, self.e2)
    }
}

/// Unary operation expression, e.g., -x, not x.
#[derive(PartialEq, Clone, Debug)]
pub struct Unop {
    pub op: UnOpKind,
    pub e: Box<Expr>,
}

impl Unop {
    pub fn new(op: UnOpKind, e: Expr) -> Self {
        Unop { op, e: Box::new(e) }
    }
}

impl fmt::Display for Unop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.op, self.e)
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Constant(c) => write!(f, "{}", c),
            Expr::Variable(v) => write!(f, "{}", v.0),
            Expr::TypeLiteral(t) => write!(f, "{}", t),
            Expr::AttributeLookup(a) => write!(f, "{}", a),
            Expr::Binop(b) => write!(f, "{}", b),
            Expr::Unop(u) => write!(f, "{}", u),
        }
    }
}
