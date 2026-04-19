#[derive(PartialEq, Clone, Debug, Eq)]
pub enum SimpleType {
    Base(BaseType),
    Star,
    Union(Box<SimpleType>, Box<SimpleType>),
    List(Box<SimpleType>),
    #[doc(hidden)]
    Zero,
}

impl SimpleType {
    /// Computes the least upper bound (gradual union) of two types.
    /// Special case: union with ⊥ yields the other type.
    pub fn union(t1: &SimpleType, t2: &SimpleType) -> SimpleType {
        match (t1, t2) {
            (SimpleType::Zero, _) => t2.clone(),
            (_, SimpleType::Zero) => t1.clone(),
            _ if t1 == t2 => t1.clone(),
            _ => SimpleType::Union(Box::new(t1.clone()), Box::new(t2.clone())),
        }
    }

    /// Computes the greatest lower bound of two types.
    /// Used to detect inconsistencies across disjoint branches or filters.
    pub fn meet(a: &SimpleType, b: &SimpleType) -> SimpleType {
        match (a, b) {
            (SimpleType::Star, _) => b.clone(),
            (_, SimpleType::Star) => a.clone(),
            (SimpleType::Union(t1, t2), _) => {
                let meet1 = SimpleType::meet(t1, b);
                let meet2 = SimpleType::meet(t2, b);
                SimpleType::union(&meet1, &meet2)
            }
            (_, SimpleType::Union(t1, t2)) => {
                let meet1 = SimpleType::meet(a, t1);
                let meet2 = SimpleType::meet(a, t2);
                SimpleType::union(&meet1, &meet2)
            }
            _ if a == b => a.clone(),
            _ => SimpleType::Zero, // Incompatible types
        }
    }

    /// Returns true if t1 is a (gradual) subtype of t2.
    /// Used in type tests and type refinements.
    pub fn is_subtype(t1: &SimpleType, t2: &SimpleType) -> bool {
        match (t1, t2) {
            (SimpleType::Star, _) | (_, SimpleType::Star) => true,
            (SimpleType::Zero, _) => true,
            (SimpleType::Union(t1a, t1b), _) => {
                SimpleType::is_subtype(t1a, t2) || SimpleType::is_subtype(t1b, t2)
            }
            (_, SimpleType::Union(t2a, t2b)) => {
                SimpleType::is_subtype(t1, t2a) || SimpleType::is_subtype(t1, t2b)
            }
            _ => t1 == t2,
        }
    }

    /// Checks whether the type is empty (⊥) or composed only of empty subtypes.
    pub fn is_empty(t: &SimpleType) -> bool {
        match t {
            SimpleType::Zero => true,
            SimpleType::Union(t1, t2) => SimpleType::is_empty(t1) && SimpleType::is_empty(t2),
            SimpleType::List(inner) => SimpleType::is_empty(inner),
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Eq)]
pub enum BaseType {
    Int,
    Bool,
    String,
}
