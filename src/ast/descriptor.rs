use super::label::LabelType;
use super::types::SimpleType;
use super::var::Var;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Clone)]
pub enum PropertyType {
    Open(HashMap<String, SimpleType>),
    Closed(HashMap<String, SimpleType>),
}

impl PropertyType {
    pub fn open() -> Self {
        PropertyType::Open(HashMap::new())
    }

    pub fn closed() -> Self {
        PropertyType::Closed(HashMap::new())
    }
}

impl Default for PropertyType {
    fn default() -> Self {
        Self::open()
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

#[derive(PartialEq, Clone, Default)]
pub struct DescriptorType {
    pub label: LabelType,
    pub properties: PropertyType,
}

impl DescriptorType {
    pub fn new(label: LabelType, properties: PropertyType) -> Self {
        DescriptorType { label, properties }
    }

    pub fn with_label(label: LabelType) -> Self {
        DescriptorType {
            label,
            properties: PropertyType::default(),
        }
    }

    pub fn with_properties(properties: PropertyType) -> Self {
        DescriptorType {
            label: LabelType::default(),
            properties,
        }
    }
}

impl fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DescriptorType({:?}, {:?})", self.label, self.properties)
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct Descriptor {
    pub variable: Option<Var>,
    pub descriptor_type: DescriptorType,
}

impl Descriptor {
    pub fn new(variable: Var, descriptor_type: DescriptorType) -> Self {
        Descriptor {
            variable: Some(variable),
            descriptor_type,
        }
    }

    pub fn with_variable(variable: Var) -> Self {
        Descriptor {
            variable: Some(variable),
            descriptor_type: DescriptorType::default(),
        }
    }

    pub fn with_type(descriptor_type: DescriptorType) -> Self {
        Descriptor {
            variable: None,
            descriptor_type,
        }
    }
}

impl fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Descriptor({:?}, {:?})",
            self.variable, self.descriptor_type
        )
    }
}
