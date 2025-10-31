use std::fmt;
use super::label::LabelType;
use super::types::PropertyType;
use super::var::Var;

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

