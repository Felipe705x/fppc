use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Var(pub String);

pub enum LabelType {
    Label(String),                       // e.g. Person
    Star,                                // *
    And(Box<LabelType>, Box<LabelType>), // e.g. Teacher & Student
    Or(Box<LabelType>, Box<LabelType>),  // e.g. Teacher | Student
}

impl fmt::Debug for LabelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LabelType::Label(s) => write!(f, "{}", s),
            LabelType::Star => write!(f, "*"),
            LabelType::And(l1, l2) => write!(f, "({:?} & {:?})", l1, l2),
            LabelType::Or(l1, l2) => write!(f, "({:?} | {:?})", l1, l2),
        }
    }
}

pub enum BaseType {
    Int,
    Bool,
    String,
}

impl fmt::Debug for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Int => write!(f, "int"),
            BaseType::Bool => write!(f, "bool"),
            BaseType::String => write!(f, "str"),
        }
    }
}

pub enum SimpleType {
    Base(BaseType),
    Star,
}

impl fmt::Debug for SimpleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleType::Base(b) => write!(f, "{:?}", b),
            SimpleType::Star => write!(f, "*"),
        }
    }
}

pub enum PropertyType {
    Open(HashMap<String, SimpleType>),
    Closed(HashMap<String, SimpleType>),
}

impl fmt::Debug for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PropertyType::Open(map) => {
                if map.is_empty() {
                    write!(f, "{{*}}")
                } else {
                    let mut keys: Vec<_> = map.keys().collect();
                    keys.sort();
                    write!(f, "{{")?;
                    for (i, key) in keys.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {:?}", key, map[*key])?;
                    }
                    write!(f, ", *}}")
                }
            }
            PropertyType::Closed(map) => {
                if map.is_empty() {
                    write!(f, "{{*}}")
                } else {
                    let mut keys: Vec<_> = map.keys().collect();
                    keys.sort();
                    write!(f, "{{")?;
                    for (i, key) in keys.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}: {:?}", key, map[*key])?;
                    }
                    write!(f, "}}")
                }
            }
        }
    }
}

pub struct DescriptorType {
    pub label: LabelType,
    pub properties: PropertyType,
}

// Debug (__repr__ equivalent) - developer representation
impl fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.label, self.properties)
    }
}

pub struct Descriptor {
    pub variable: Option<Var>,
    pub descriptor_type: DescriptorType, // Always present, defaults to Star {}
}

// Debug (__repr__ equivalent) - "Descriptor(x, Person{...})"
impl fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.variable {
            Some(var) => write!(f, "Descriptor({}, {:?})", var.0, self.descriptor_type),
            None => write!(f, "Descriptor(None, {:?})", self.descriptor_type),
        }
    }
}

pub struct ElementPatternFiller {
    pub descriptor: Descriptor, // Always present
                                // pub where_clause: Option<WhereClause>, // Add later
}

impl fmt::Debug for ElementPatternFiller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.descriptor)
    }
}


pub struct NodePattern {
    pub filler: ElementPatternFiller,
}

impl fmt::Debug for NodePattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?})", self.filler)
    }
}
