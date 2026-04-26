#[derive(PartialEq, Clone, Default, Debug)]
pub enum LabelType {
    Label(String),
    #[default]
    Star,
    And(Box<LabelType>, Box<LabelType>),
    Or(Box<LabelType>, Box<LabelType>),
}

impl LabelType {
    pub fn and(l1: LabelType, l2: LabelType) -> Self {
        LabelType::And(Box::new(l1), Box::new(l2))
    }

    pub fn or(l1: LabelType, l2: LabelType) -> Self {
        LabelType::Or(Box::new(l1), Box::new(l2))
    }

    /// Constructs a conjunction (AND) of labels from a list of strings.
    /// E.g., ["A", "B", "C"] → A & B & C
    pub fn from_list(labels: &[String]) -> Self {
        match labels {
            [] => LabelType::Star,
            [single] => LabelType::Label(single.clone()),
            [first, rest @ ..] => {
                LabelType::and(LabelType::Label(first.clone()), LabelType::from_list(rest))
            }
        }
    }

    /// Computes the meet (greatest lower bound) of two label types under logical entailment.
    pub fn meet(a: LabelType, b: LabelType) -> LabelType {
        match (&a, &b) {
            (LabelType::Star, _) => b,
            (_, LabelType::Star) => a,
            _ if LabelType::is_subtype(&a, &b) => a,
            _ if LabelType::is_subtype(&b, &a) => b,
            _ => LabelType::and(a, b),
        }
    }

    /// Determines whether label type `l1` is a subtype of `l2`.
    pub fn is_subtype(l1: &LabelType, l2: &LabelType) -> bool {
        match (l1, l2) {
            (LabelType::Star, _) | (_, LabelType::Star) => true,
            (LabelType::Label(a), LabelType::Label(b)) => a == b,
            (_, LabelType::And(left, right)) => {
                LabelType::is_subtype(l1, left) && LabelType::is_subtype(l1, right)
            }
            (_, LabelType::Or(left, right)) => {
                LabelType::is_subtype(l1, left) || LabelType::is_subtype(l1, right)
            }
            (LabelType::And(left, right), _) => {
                LabelType::is_subtype(left, l2) || LabelType::is_subtype(right, l2)
            }
            (LabelType::Or(left, right), _) => {
                LabelType::is_subtype(left, l2) || LabelType::is_subtype(right, l2)
            }
        }
    }
}
