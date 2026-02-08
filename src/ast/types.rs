#[derive(PartialEq, Clone, Debug)]
pub enum SimpleType {
    Base(BaseType),
    Star,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BaseType {
    Int,
    Bool,
    String,
}
