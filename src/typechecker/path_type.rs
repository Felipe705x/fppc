use crate::ast::{DescriptorType, EdgeDirection};

use super::schema::Schema;
use super::variable_type::{EdgeKind, EdgeType, NodeType, VariableType};

#[derive(PartialEq, Clone, Debug)]
pub struct NodePathType {
    pub n: NodeType,
}

impl NodePathType {
    pub fn new(n: NodeType) -> Self {
        NodePathType { n }
    }
}

impl From<NodeType> for NodePathType {
    fn from(n: NodeType) -> Self {
        NodePathType::new(n)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct EdgePathType {
    pub p1: Box<PathType>,
    pub n2: NodePathType,
}

/// Path types representing sequences of nodes and edges.
#[derive(PartialEq, Clone, Debug)]
pub enum PathType {
    /// A single node in the path.
    Node(NodePathType),
    /// An edge connecting a path to a node: path - node.
    Edge(EdgePathType),
    /// Union of two path types.
    Union(Box<PathType>, Box<PathType>),
    /// Bottom type (empty/inconsistent path).
    Zero,
}

impl Default for PathType {
    fn default() -> Self {
        PathType::Node(NodeType::default().into())
    }
}

impl From<(&VariableType, EdgeDirection)> for PathType {
    fn from((t, direction): (&VariableType, EdgeDirection)) -> Self {
        match t {
            VariableType::Node(n) => PathType::Node(n.clone().into()),
            VariableType::Edge(edge) => match edge.kind {
                EdgeKind::Directed => PathType::directed_edge_to_path(edge, direction),
                EdgeKind::Undirected => {
                    let directed = edge.to_directed(false);
                    PathType::directed_edge_to_path(&directed, EdgeDirection::Any)
                }
            },
            VariableType::Union(t1, t2) => PathType::union(
                PathType::from((&**t1, direction)),
                PathType::from((&**t2, direction)),
            ),
            VariableType::Zero => PathType::Zero,
            VariableType::List(_) => PathType::Zero,
        }
    }
}

impl PathType {
    fn directed_edge_to_path(edge: &EdgeType, direction: EdgeDirection) -> PathType {
        let make_step = |e: &EdgeType| {
            PathType::Edge(EdgePathType {
                p1: Box::new(PathType::Node(e.left.clone().into())),
                n2: e.right.clone().into(),
            })
        };

        let forward = make_step(edge);
        let reversed_edge = edge.to_directed(true);
        let reversed = make_step(&reversed_edge);

        match direction {
            EdgeDirection::Right => forward,
            EdgeDirection::Left => reversed,
            EdgeDirection::Any | EdgeDirection::None => PathType::union(forward, reversed),
        }
    }

    /// Creates a node path type.
    pub fn node(n: NodeType) -> Self {
        PathType::Node(n.into())
    }

    /// Creates an edge path type.
    pub fn edge(path: PathType, node: NodeType) -> Self {
        PathType::Edge(EdgePathType {
            p1: Box::new(path),
            n2: node.into(),
        })
    }

    /// Returns the length of the path (number of edges).
    pub fn len(&self) -> usize {
        match self {
            PathType::Node(_) => 0,
            PathType::Edge(edge) => edge.p1.len() + 1,
            PathType::Union(p1, p2) => p1.len().min(p2.len()),
            PathType::Zero => 0,
        }
    }

    /// Returns true if the path has no edges (length 0).
    pub fn is_empty(&self) -> bool {
        match self {
            PathType::Node(_) | PathType::Zero => true,
            PathType::Edge(_) => false,
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

    /// Computes the meet (greatest lower bound) of two path types.
    pub fn meet(schema: &Schema, p1: &PathType, p2: &PathType) -> PathType {
        match (p1, p2) {
            (PathType::Zero, _) | (_, PathType::Zero) => PathType::Zero,

            (PathType::Node(n1), PathType::Node(n2)) => {
                let met = VariableType::Node(NodeType(DescriptorType::meet(&n1.n.0, &n2.n.0)));
                let nodes = VariableType::refine_to_nodes(schema, &met);
                PathType::union_from_list(
                    nodes
                        .into_iter()
                        .map(|n| PathType::Node(n.into()))
                        .collect(),
                )
            }

            (PathType::Edge(p1_edge), PathType::Node(n2)) => {
                let met =
                    VariableType::Node(NodeType(DescriptorType::meet(&p1_edge.n2.n.0, &n2.n.0)));
                let nodes = VariableType::refine_to_nodes(schema, &met);
                PathType::union_from_list(
                    nodes
                        .into_iter()
                        .map(|n| {
                            PathType::Edge(EdgePathType {
                                p1: Box::new((*p1_edge.p1).clone()),
                                n2: n.into(),
                            })
                        })
                        .collect(),
                )
            }

            (_, PathType::Edge(p2_edge)) => PathType::Edge(EdgePathType {
                p1: Box::new(PathType::meet(schema, p1, &p2_edge.p1)),
                n2: p2_edge.n2.clone(),
            }),

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
            PathType::Node(n) => DescriptorType::is_empty(&n.n.0),
            PathType::Edge(edge) => {
                edge.p1.is_unsatisfiable() || DescriptorType::is_empty(&edge.n2.n.0)
            }
            PathType::Union(p1, p2) => p1.is_unsatisfiable() && p2.is_unsatisfiable(),
        }
    }
}
