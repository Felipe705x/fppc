use std::fmt;


#[derive(PartialEq, Clone)]
pub enum LabelType {
    Label(String),                       // e.g. Person
    Star,                                // *
    And(Box<LabelType>, Box<LabelType>), // e.g. Teacher & Student
    Or(Box<LabelType>, Box<LabelType>),  // e.g. Teacher | Student
}

impl LabelType {
    pub fn new_and(l1: LabelType, l2: LabelType) -> Self {
        LabelType::And(Box::new(l1), Box::new(l2))
    }

    pub fn new_or(l1: LabelType, l2: LabelType) -> Self {
        LabelType::Or(Box::new(l1), Box::new(l2))
    }
}

impl fmt::Debug for LabelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LabelType::Label(s) => write!(f, "Label({})", s),
            LabelType::Star => write!(f, "Star"),
            LabelType::And(l1, l2) => write!(f, "And({:?}, {:?})", l1, l2),
            LabelType::Or(l1, l2) => write!(f, "Or({:?}, {:?})", l1, l2),
        }
    }
}

