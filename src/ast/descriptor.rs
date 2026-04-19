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

    /// Returns the top-level wildcard descriptor: (*,*).
    /// Matches any label and allows any properties.
    pub fn star() -> Self {
        DescriptorType {
            label: LabelType::Star,
            properties: PropertyType::open(),
        }
    }

    /// Computes the greatest lower bound (meet) of two descriptors.
    /// This merges both label and property constraints.
    pub fn meet(a: &DescriptorType, b: &DescriptorType) -> DescriptorType {
        DescriptorType {
            label: LabelType::meet(a.label.clone(), b.label.clone()),
            properties: PropertyType::meet(&a.properties, &b.properties),
        }
    }

    /// Returns true if descriptor t1 is a subtype of descriptor t2.
    /// This requires both label and property subtyping to hold.
    pub fn is_subtype(t1: &DescriptorType, t2: &DescriptorType) -> bool {
        LabelType::is_subtype(&t1.label, &t2.label)
            && PropertyType::is_subtype(&t1.properties, &t2.properties)
    }

    /// Returns true if the descriptor is unsatisfiable (i.e., some inconsistency).
    /// A descriptor is empty if its property type is empty.
    pub fn is_empty(t: &DescriptorType) -> bool {
        PropertyType::is_empty(&t.properties)
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

    /// Returns the set of keys in this property type.
    pub fn keys(&self) -> std::collections::HashSet<&String> {
        match self {
            PropertyType::Open(m) | PropertyType::Closed(m) => m.keys().collect(),
            PropertyType::Zero => std::collections::HashSet::new(),
        }
    }

    /// Gets the type associated with a key.
    /// Returns Star for missing keys in Open types, Zero for missing keys in Closed types.
    pub fn get(&self, key: &str) -> SimpleType {
        match self {
            PropertyType::Open(m) => m.get(key).cloned().unwrap_or(SimpleType::Star),
            PropertyType::Closed(m) => m.get(key).cloned().unwrap_or(SimpleType::Zero),
            PropertyType::Zero => SimpleType::Zero,
        }
    }

    /// Adds or overrides an attribute with the given type.
    pub fn extend(&mut self, key: String, t: SimpleType) {
        match self {
            PropertyType::Open(m) | PropertyType::Closed(m) => {
                m.insert(key, t);
            }
            PropertyType::Zero => {}
        }
    }

    /// Computes the meet (greatest lower bound) of two property types.
    ///
    /// The result is:
    /// - Closed only if both are Closed and have the same keys.
    /// - Open if both are Open.
    /// - Closed if one is Open and the other Closed, but their keys are compatible.
    /// - Zero otherwise.
    pub fn meet(a: &PropertyType, b: &PropertyType) -> PropertyType {
        match (a, b) {
            // Case 1: Both closed with identical keys
            (PropertyType::Closed(_), PropertyType::Closed(_)) if a.keys() == b.keys() => {
                let mut c = PropertyType::closed();
                for k in a.keys() {
                    c.extend(k.clone(), SimpleType::meet(&a.get(k), &b.get(k)));
                }
                c
            }

            // Case 2: Both open — union of keys
            (PropertyType::Open(_), PropertyType::Open(_)) => {
                let mut c = PropertyType::open();
                let all_keys: std::collections::HashSet<_> =
                    a.keys().union(&b.keys()).cloned().collect();
                for k in all_keys {
                    let t = if a.keys().contains(k) && b.keys().contains(k) {
                        SimpleType::meet(&a.get(k), &b.get(k))
                    } else if a.keys().contains(k) {
                        a.get(k)
                    } else {
                        b.get(k)
                    };
                    c.extend(k.clone(), t);
                }
                c
            }

            // Case 3: Open ⊓ Closed, where open keys ⊆ closed keys
            (PropertyType::Open(_), PropertyType::Closed(_)) if a.keys().is_subset(&b.keys()) => {
                let mut c = PropertyType::closed();
                for k in b.keys() {
                    let t = if a.keys().contains(&k) {
                        SimpleType::meet(&a.get(k), &b.get(k))
                    } else {
                        b.get(k)
                    };
                    c.extend(k.clone(), t);
                }
                c
            }

            // Case 4: Closed ⊓ Open, symmetric
            (PropertyType::Closed(_), PropertyType::Open(_)) if b.keys().is_subset(&a.keys()) => {
                let mut c = PropertyType::closed();
                for k in a.keys() {
                    let t = if b.keys().contains(&k) {
                        SimpleType::meet(&a.get(k), &b.get(k))
                    } else {
                        a.get(k)
                    };
                    c.extend(k.clone(), t);
                }
                c
            }

            // All other cases (incompatible or involving Zero)
            _ => PropertyType::Zero,
        }
    }

    /// Returns true if t1 is a subtype of t2.
    /// Open types allow extra keys; closed types must match exactly.
    pub fn is_subtype(t1: &PropertyType, t2: &PropertyType) -> bool {
        match (t1, t2) {
            (PropertyType::Zero, _) => true,
            (PropertyType::Open(_), PropertyType::Open(_)) => {
                // Check common keys
                t1.keys()
                    .intersection(&t2.keys())
                    .all(|k| SimpleType::is_subtype(&t1.get(k), &t2.get(k)))
            }

            (PropertyType::Closed(_), PropertyType::Closed(_)) => {
                // Must have same keys
                t1.keys() == t2.keys()
                    && t2
                        .keys()
                        .iter()
                        .all(|k| SimpleType::is_subtype(&t1.get(k), &t2.get(k)))
            }

            (PropertyType::Closed(_), PropertyType::Open(_)) => {
                // Open keys must be subset of closed keys
                t2.keys().is_subset(&t1.keys())
                    && t2
                        .keys()
                        .iter()
                        .all(|k| SimpleType::is_subtype(&t1.get(k), &t2.get(k)))
            }

            (PropertyType::Open(_), PropertyType::Closed(_)) => {
                // Open keys must be subset of closed keys
                t1.keys().is_subset(&t2.keys())
                    && t1
                        .keys()
                        .iter()
                        .all(|k| SimpleType::is_subtype(&t1.get(k), &t2.get(k)))
            }

            _ => false,
        }
    }

    /// Returns true if any attribute in the property type is statically empty (⊥).
    pub fn is_empty(t: &PropertyType) -> bool {
        match t {
            PropertyType::Open(_) | PropertyType::Closed(_) => {
                !t.keys().is_empty() && t.keys().iter().any(|k| SimpleType::is_empty(&t.get(k)))
            }
            PropertyType::Zero => true,
        }
    }
}

impl Default for PropertyType {
    fn default() -> Self {
        Self::open()
    }
}
