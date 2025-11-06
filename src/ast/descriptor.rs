use std::fmt;
use super::label::LabelType;
use super::types::PropertyType;
use super::var::Var;

#[derive(PartialEq, Clone)]
pub struct DescriptorType {
    pub label: LabelType,
    pub properties: PropertyType,
}

impl Default for DescriptorType {
    fn default() -> Self {
        DescriptorType {
            label: LabelType::Star,
            properties: PropertyType::default(),
        }
    }
}

impl fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DescriptorType({:?}, {:?})", self.label, self.properties)
    }
}

#[derive(PartialEq, Clone)]
pub struct Descriptor {
    pub variable: Option<Var>,
    pub descriptor_type: DescriptorType, // Always present, defaults to Star {}
}

impl fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Descriptor({:?}, {:?})", self.variable, self.descriptor_type)
    }
}

