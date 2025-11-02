use std::collections::HashMap;
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

#[derive(Clone)]
pub enum PropertyType {
    Open(HashMap<String, SimpleType>),
    Closed(HashMap<String, SimpleType>),
}

impl Default for PropertyType {
    fn default() -> Self {
        PropertyType::Open(HashMap::new())
    }
}

impl fmt::Debug for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyType::Open(map) => {
                if map.is_empty() {
                    write!(f, "Open({{}})")
                } else {
                    let mut keys: Vec<_> = map.keys().collect();
                    keys.sort();
                    write!(f, "Open({{")?;
                    for (i, key) in keys.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {:?}", key, map[*key])?;
                    }
                    write!(f, "}})")
                }
            }
            PropertyType::Closed(map) => {
                if map.is_empty() {
                    write!(f, "Closed({{}})")
                } else {
                    let mut keys: Vec<_> = map.keys().collect();
                    keys.sort();
                    write!(f, "Closed({{")?;
                    for (i, key) in keys.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {:?}", key, map[*key])?;
                    }
                    write!(f, "}})")
                }
            }
        }
    }
}

