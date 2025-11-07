use std::fmt;

#[derive(PartialEq, Clone)]
pub enum BaseType {
    Int,
    Bool,
    String,
}

impl fmt::Debug for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Int => write!(f, "Int"),
            BaseType::Bool => write!(f, "Bool"),
            BaseType::String => write!(f, "String"),
        }
    }
}

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Int => write!(f, "int"),
            BaseType::Bool => write!(f, "bool"),
            BaseType::String => write!(f, "str"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum SimpleType {
    Base(BaseType),
    Star,
}

impl fmt::Debug for SimpleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleType::Base(b) => write!(f, "Base({:?})", b),
            SimpleType::Star => write!(f, "Star"),
        }
    }
}

impl fmt::Display for SimpleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleType::Base(b) => write!(f, "{}", b),
            SimpleType::Star => write!(f, "*"),
        }
    }
}
