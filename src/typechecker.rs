pub mod checker;
pub mod path_type;
pub mod schema;
pub mod type_environment;
pub mod variable_type;

pub use checker::{TypecheckResult, Typechecker};
pub use path_type::PathType;
pub use schema::Schema;
pub use type_environment::TypeEnvironment;
pub use variable_type::{EdgeKind, EdgeType, NodeType, VariableType};
