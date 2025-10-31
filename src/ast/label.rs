use std::fmt;


#[derive(PartialEq, Clone)]
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

