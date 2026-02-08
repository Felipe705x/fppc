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
}
