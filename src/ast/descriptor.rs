use super::label::LabelType;
use super::types::SimpleType;
use super::var::Var;
use std::collections::HashMap;
use std::fmt;

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
        if f.alternate() {
            // Pretty: use full struct format with newlines
            f.debug_struct("Descriptor")
                .field("variable", &self.variable)
                .field("label", &self.descriptor_type.label)
                .field("properties", &self.descriptor_type.properties)
                .finish()
        } else {
            // Compact: flattened tuple format
            write!(
                f,
                "Descriptor({:?}, {:?}, {:?})",
                self.variable, self.descriptor_type.label, self.descriptor_type.properties
            )
        }
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub enum PropertyType {
    Open(HashMap<String, SimpleType>),
    Closed(HashMap<String, SimpleType>),
    #[doc(hidden)]
    Zero,
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
