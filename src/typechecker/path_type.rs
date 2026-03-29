use crate::ast::EdgeDirection;

use super::schema::Schema;
use super::variable_type::{EdgeKind, EdgeType, VariableType};

/// Path types representing sequences of nodes and edges.
#[derive(PartialEq, Clone, Debug)]
pub enum PathType {
    /// A single node in the path.
    Node(VariableType),
    /// An edge connecting a path to a node: path - node.
    Edge {
        path: Box<PathType>,
        node: VariableType,
    },
    /// Union of two path types.
    Union(Box<PathType>, Box<PathType>),
    /// Bottom type (empty/inconsistent path).
    Zero,
}

impl Default for PathType {
    fn default() -> Self {
        PathType::Node(VariableType::node())
    }
}

impl From<&EdgeType> for PathType {
    fn from(edge: &EdgeType) -> Self {
        match edge.kind {
            EdgeKind::Directed => PathType::Edge {
                path: Box::new(PathType::Node(edge.left.clone().into())),
                node: edge.right.clone().into(),
            },
            EdgeKind::Undirected => {
                let forward = edge.to_directed(false);
                let reversed = edge.to_directed(true);
                PathType::union(PathType::from(&forward), PathType::from(&reversed))
            }
        }
    }
}

impl From<EdgeType> for PathType {
    fn from(edge: EdgeType) -> Self {
        PathType::from(&edge)
    }
}

impl PathType {
    /// Creates a node path type.
    pub fn node(n: VariableType) -> Self {
        PathType::Node(n)
    }

    /// Creates an edge path type.
    pub fn edge(path: PathType, node: VariableType) -> Self {
        PathType::Edge {
            path: Box::new(path),
            node,
        }
    }

    /// Returns the length of the path (number of edges).
    pub fn len(&self) -> usize {
        match self {
            PathType::Node(_) => 0,
            PathType::Edge { path, .. } => path.len() + 1,
            PathType::Union(p1, p2) => p1.len().min(p2.len()),
            PathType::Zero => 0,
        }
    }

    /// Returns true if the path has no edges (length 0).
    pub fn is_empty(&self) -> bool {
        match self {
            PathType::Node(_) | PathType::Zero => true,
            PathType::Edge { .. } => false,
            PathType::Union(p1, p2) => p1.is_empty() || p2.is_empty(),
        }
    }

    /// Computes the union of two path types.
    pub fn union(p1: PathType, p2: PathType) -> PathType {
        match (&p1, &p2) {
            (PathType::Zero, _) => p2,
            (_, PathType::Zero) => p1,
            _ if p1 == p2 => p1,
            _ => PathType::Union(Box::new(p1), Box::new(p2)),
        }
    }

    /// Constructs a union from a list of path types.
    pub fn union_from_list(paths: Vec<PathType>) -> PathType {
        if paths.is_empty() {
            return PathType::Zero;
        }
        paths
            .into_iter()
            .reduce(PathType::union)
            .unwrap_or(PathType::Zero)
    }

    /// Converts a VariableType to a PathType given a direction.
    pub fn to_path_type(t: &VariableType, direction: EdgeDirection) -> PathType {
        match t {
            VariableType::Node(_) => PathType::Node(t.clone()),
            VariableType::Edge(edge) => match edge.kind {
                EdgeKind::Directed => {
                    let forward = edge.to_directed(false);
                    let reversed = edge.to_directed(true);
                    match direction {
                        EdgeDirection::Right => PathType::from(&forward),
                        EdgeDirection::Left => PathType::from(&reversed),
                        EdgeDirection::Any | EdgeDirection::None => {
                            PathType::union(PathType::from(&forward), PathType::from(&reversed))
                        }
                    }
                }
                EdgeKind::Undirected => PathType::from(edge),
            },
            VariableType::Union(t1, t2) => PathType::union(
                PathType::to_path_type(t1, direction),
                PathType::to_path_type(t2, direction),
            ),
            VariableType::Zero => PathType::Zero,
            VariableType::List(_) => PathType::Zero,
        }
    }

    /// Computes the meet (greatest lower bound) of two path types.
    pub fn meet(schema: &Schema, p1: &PathType, p2: &PathType) -> PathType {
        match (p1, p2) {
            (PathType::Zero, _) | (_, PathType::Zero) => PathType::Zero,

            (PathType::Node(n1), PathType::Node(n2)) => match VariableType::meet(n1, n2) {
                Ok(met) => PathType::Node(VariableType::refine(schema, &met)),
                Err(_) => PathType::Zero,
            },

            (
                PathType::Edge {
                    path: p1_path,
                    node: p1_node,
                },
                PathType::Node(n2),
            ) => match VariableType::meet(p1_node, n2) {
                Ok(met) => PathType::Edge {
                    path: p1_path.clone(),
                    node: VariableType::refine(schema, &met),
                },
                Err(_) => PathType::Zero,
            },

            (
                _,
                PathType::Edge {
                    path: p2_path,
                    node: p2_node,
                },
            ) => PathType::Edge {
                path: Box::new(PathType::meet(schema, p1, p2_path)),
                node: p2_node.clone(),
            },

            (_, PathType::Union(u1, u2)) => {
                let m1 = PathType::meet(schema, p1, u1);
                let m2 = PathType::meet(schema, p1, u2);
                if m1.is_unsatisfiable() {
                    return m2;
                }
                if m2.is_unsatisfiable() {
                    return m1;
                }
                PathType::union(m1, m2)
            }

            (PathType::Union(u1, u2), _) => {
                let m1 = PathType::meet(schema, u1, p2);
                let m2 = PathType::meet(schema, u2, p2);
                if m1.is_unsatisfiable() {
                    return m2;
                }
                if m2.is_unsatisfiable() {
                    return m1;
                }
                PathType::union(m1, m2)
            }
        }
    }

    /// Determines if a path type is unsatisfiable (inconsistent/bottom).
    pub fn is_unsatisfiable(&self) -> bool {
        match self {
            PathType::Zero => true,
            PathType::Node(n) => VariableType::is_empty(n),
            PathType::Edge { path, node } => {
                path.is_unsatisfiable() || VariableType::is_empty(node)
            }
            PathType::Union(p1, p2) => p1.is_unsatisfiable() && p2.is_unsatisfiable(),
        }
    }
}
