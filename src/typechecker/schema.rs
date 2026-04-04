use crate::ast::DescriptorType;

use super::variable_type::{EdgeType, NodeType};

/// Represents a user-defined schema for a GQL graph.
///
/// Contains:
/// - a list of allowed node types
/// - a list of allowed edge types
///
/// Used for type checking and type refinement of query patterns.
#[derive(Clone, Debug)]
pub struct Schema {
    pub nodes: Vec<NodeType>,
    pub edges: Vec<EdgeType>,
}

impl Schema {
    /// Constructs a schema from explicitly provided node and edge types.
    pub fn new(nodes: Vec<NodeType>, edges: Vec<EdgeType>) -> Self {
        Schema { nodes, edges }
    }

    /// Returns a permissive (default) schema that allows any label or property.
    ///
    /// Includes:
    /// - one generic node type (star descriptor)
    /// - one directional edge and one undirected edge, both with star descriptors
    pub fn star() -> Self {
        Schema {
            nodes: vec![NodeType::default()],
            edges: vec![
                EdgeType::directed(
                    DescriptorType::star(),
                    NodeType::default(),
                    NodeType::default(),
                ),
                EdgeType::undirected(
                    DescriptorType::star(),
                    NodeType::default(),
                    NodeType::default(),
                ),
            ],
        }
    }
}
