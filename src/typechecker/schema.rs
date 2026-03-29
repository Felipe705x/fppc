use crate::ast::DescriptorType;

use super::variable_type::{NodeType, VariableType};

/// Represents a user-defined schema for a GQL graph.
///
/// Contains:
/// - a list of allowed node types
/// - a list of allowed edge types
///
/// Used for type checking and type refinement of query patterns.
#[derive(Clone, Debug)]
pub struct Schema {
    pub nodes: Vec<VariableType>,
    pub edges: Vec<VariableType>,
}

impl Schema {
    /// Constructs a schema from explicitly provided node and edge types.
    pub fn new(nodes: Vec<VariableType>, edges: Vec<VariableType>) -> Self {
        Schema { nodes, edges }
    }

    /// Returns a permissive (default) schema that allows any label or property.
    ///
    /// Includes:
    /// - one generic node type (star descriptor)
    /// - one directional edge and one undirected edge, both with star descriptors
    pub fn star() -> Self {
        Schema {
            nodes: vec![VariableType::node()],
            edges: vec![
                VariableType::edge_directional(
                    DescriptorType::star(),
                    NodeType::default(),
                    NodeType::default(),
                ),
                VariableType::edge_non_directional(
                    DescriptorType::star(),
                    NodeType::default(),
                    NodeType::default(),
                ),
            ],
        }
    }
}
