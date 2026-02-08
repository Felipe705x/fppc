use super::types::SimpleType;
use super::var::Var;

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

/// Represents a constant expression (string, int, or boolean).
#[derive(PartialEq, Clone, Debug)]
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

/// Unary operator kinds
#[derive(PartialEq, Clone, Debug)]
pub enum UnOpKind {
    Neg, // -
    Not, // not
}
