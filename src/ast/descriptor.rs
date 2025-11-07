use super::label::LabelType;
use super::types::PropertyType;
use super::var::Var;
use std::fmt;

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
