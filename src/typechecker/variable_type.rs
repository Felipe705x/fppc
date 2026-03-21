use crate::ast::{DescriptorType, SimpleType};

/// Represents the type of a node variable in a GQL pattern.
/// A node is typed by a DescriptorType (label + properties).
#[derive(PartialEq, Clone, Debug)]
pub struct NodeType(pub DescriptorType);

impl Default for NodeType {
    fn default() -> Self {
        NodeType(DescriptorType::star())
    }
}

/// Edge directionality in the type system.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EdgeKind {
    /// Directed edge: left -[descriptor]-> right
    Directed,
    /// Undirected edge: left -[descriptor]- right
    Undirected,
}

/// Represents the type of an edge variable in a GQL pattern.
/// An edge is typed by a descriptor (label + properties),
/// a left endpoint (node), a right endpoint (node), and a directionality.
#[derive(PartialEq, Clone, Debug)]
pub struct EdgeType {
    pub descriptor: DescriptorType,
    pub left: NodeType,
    pub right: NodeType,
    pub kind: EdgeKind,
}

impl EdgeType {
    pub fn directed(descriptor: DescriptorType, left: NodeType, right: NodeType) -> Self {
        EdgeType {
            descriptor,
            left,
            right,
            kind: EdgeKind::Directed,
        }
    }

    pub fn undirected(descriptor: DescriptorType, left: NodeType, right: NodeType) -> Self {
        EdgeType {
            descriptor,
            left,
            right,
            kind: EdgeKind::Undirected,
        }
    }
}

/// Represents the inferred or declared types of variables in a GQL query.
#[derive(PartialEq, Clone, Debug)]
pub enum VariableType {
    /// Node with descriptor type (label + properties)
    Node(NodeType),
    /// Edge with descriptor, endpoints, and directionality
    Edge(EdgeType),
    /// Union of two variable types (disjunction)
    Union(Box<VariableType>, Box<VariableType>),
    /// List of variable values (from repetition or grouping)
    List(Box<VariableType>),
    /// Bottom type (inconsistent/unsatisfiable)
    Zero,
}

impl Default for VariableType {
    fn default() -> Self {
        VariableType::Node(NodeType::default())
    }
}

impl From<NodeType> for VariableType {
    fn from(n: NodeType) -> Self {
        VariableType::Node(n)
    }
}

impl VariableType {
    /// Creates a node variable type with a star descriptor.
    pub fn node() -> Self {
        VariableType::Node(NodeType::default())
    }

    /// Creates a node variable type with the given descriptor.
    pub fn node_with(descriptor: DescriptorType) -> Self {
        VariableType::Node(NodeType(descriptor))
    }

    /// Creates a directed edge variable type.
    pub fn edge_directional(descriptor: DescriptorType, left: NodeType, right: NodeType) -> Self {
        VariableType::Edge(EdgeType::directed(descriptor, left, right))
    }

    /// Creates an undirected edge variable type.
    pub fn edge_non_directional(
        descriptor: DescriptorType,
        left: NodeType,
        right: NodeType,
    ) -> Self {
        VariableType::Edge(EdgeType::undirected(descriptor, left, right))
    }

    /// Creates a union variable type.
    pub fn union(t1: VariableType, t2: VariableType) -> Self {
        VariableType::Union(Box::new(t1), Box::new(t2))
    }

    /// Creates a list variable type.
    pub fn list(t: VariableType) -> Self {
        VariableType::List(Box::new(t))
    }

    /// Returns the type of property `a`, if defined.
    pub fn get_attribute(&self, a: &str) -> SimpleType {
        match self {
            VariableType::Node(node) => node.0.properties.get(a),
            VariableType::Edge(edge) => edge.descriptor.properties.get(a),
            VariableType::Union(t1, t2) => {
                SimpleType::union(&t1.get_attribute(a), &t2.get_attribute(a))
            }
            VariableType::List(t) => SimpleType::List(Box::new(t.get_attribute(a))),
            VariableType::Zero => SimpleType::Zero,
        }
    }

    /// Greatest lower bound of two node types.
    fn meet_node(a: &NodeType, b: &NodeType) -> NodeType {
        NodeType(DescriptorType::meet(&a.0, &b.0))
    }

    /// Greatest lower bound of two edge variable types.
    /// Expects both arguments to be `Edge` variants.
    fn meet_edge(e1: &VariableType, e2: &VariableType) -> Result<VariableType, String> {
        match (e1, e2) {
            (VariableType::Edge(edge1), VariableType::Edge(edge2)) => {
                match (&edge1.kind, &edge2.kind) {
                    (EdgeKind::Directed, EdgeKind::Directed) => {
                        let desc = DescriptorType::meet(&edge1.descriptor, &edge2.descriptor);
                        let left = VariableType::meet_node(&edge1.left, &edge2.left);
                        let right = VariableType::meet_node(&edge1.right, &edge2.right);
                        Ok(VariableType::Edge(EdgeType::directed(desc, left, right)))
                    }
                    (EdgeKind::Undirected, EdgeKind::Undirected) => {
                        let as_dir1 = VariableType::Edge(EdgeType::directed(
                            edge1.descriptor.clone(),
                            edge1.left.clone(),
                            edge1.right.clone(),
                        ));
                        let as_dir2 = VariableType::Edge(EdgeType::directed(
                            edge2.descriptor.clone(),
                            edge2.left.clone(),
                            edge2.right.clone(),
                        ));
                        let as_dir2_flipped = VariableType::Edge(EdgeType::directed(
                            edge2.descriptor.clone(),
                            edge2.right.clone(),
                            edge2.left.clone(),
                        ));

                        let n1 = VariableType::meet_edge(&as_dir1, &as_dir2);
                        let n2 = VariableType::meet_edge(&as_dir1, &as_dir2_flipped);

                        match (n1, n2) {
                            (Ok(VariableType::Edge(ed1)), Ok(VariableType::Edge(ed2))) => {
                                Ok(VariableType::join(
                                    VariableType::Edge(EdgeType::undirected(
                                        ed1.descriptor,
                                        ed1.left,
                                        ed1.right,
                                    )),
                                    VariableType::Edge(EdgeType::undirected(
                                        ed2.descriptor,
                                        ed2.left,
                                        ed2.right,
                                    )),
                                ))
                            }
                            _ => Err("Cannot meet non-directional edges".to_string()),
                        }
                    }
                    _ => Err("Cannot meet edges of different directionality".to_string()),
                }
            }
            _ => Err(format!(
                "meet_edge called on non-edge types {:?} and {:?}",
                e1, e2
            )),
        }
    }

    /// General meet operator (greatest lower bound).
    pub fn meet(a: &VariableType, b: &VariableType) -> Result<VariableType, String> {
        match (a, b) {
            (VariableType::List(inner_a), VariableType::List(inner_b)) => Ok(VariableType::List(
                Box::new(VariableType::meet(inner_a, inner_b)?),
            )),
            (VariableType::Node(n1), VariableType::Node(n2)) => {
                Ok(VariableType::Node(VariableType::meet_node(n1, n2)))
            }
            (VariableType::Edge(_), VariableType::Edge(_)) => VariableType::meet_edge(a, b),
            (VariableType::Union(t1, t2), _) => {
                let r1 = VariableType::meet(t1, b);
                let r2 = VariableType::meet(t2, b);
                match (r1, r2) {
                    (Ok(v1), Ok(v2)) => Ok(VariableType::Union(Box::new(v1), Box::new(v2))),
                    (Ok(v1), Err(_)) => Ok(v1),
                    (Err(_), Ok(v2)) => Ok(v2),
                    (Err(e1), Err(_)) => Err(e1),
                }
            }
            (_, VariableType::Union(_, _)) => VariableType::meet(b, a),
            (VariableType::Zero, _) | (_, VariableType::Zero) => Ok(VariableType::Zero),
            _ => Err(format!(
                "Meet undefined between variable types {:?} and {:?}",
                a, b
            )),
        }
    }

    /// Least upper bound (union) of two types.
    pub fn join(a: VariableType, b: VariableType) -> VariableType {
        match (&a, &b) {
            (VariableType::Zero, _) => b,
            (_, VariableType::Zero) => a,
            _ if a == b => a,
            _ => VariableType::Union(Box::new(a), Box::new(b)),
        }
    }

    /// Combines a list of variable types using join.
    /// Returns Zero if list is empty.
    pub fn join_from_list(types: Vec<VariableType>) -> VariableType {
        types
            .into_iter()
            .fold(VariableType::Zero, VariableType::join)
    }

    /// Checks if t1 is a subtype of t2.
    pub fn is_subtype(t1: &VariableType, t2: &VariableType) -> bool {
        match (t1, t2) {
            (VariableType::Node(n1), VariableType::Node(n2)) => {
                DescriptorType::is_subtype(&n1.0, &n2.0)
            }
            (VariableType::Edge(e1), VariableType::Edge(e2)) => match (&e1.kind, &e2.kind) {
                (EdgeKind::Directed, EdgeKind::Directed) => {
                    DescriptorType::is_subtype(&e1.descriptor, &e2.descriptor)
                        && DescriptorType::is_subtype(&e1.left.0, &e2.left.0)
                        && DescriptorType::is_subtype(&e1.right.0, &e2.right.0)
                }
                (EdgeKind::Undirected, EdgeKind::Undirected) => {
                    let dir1 = VariableType::Edge(EdgeType::directed(
                        e1.descriptor.clone(),
                        e1.left.clone(),
                        e1.right.clone(),
                    ));
                    let dir2_normal = VariableType::Edge(EdgeType::directed(
                        e2.descriptor.clone(),
                        e2.left.clone(),
                        e2.right.clone(),
                    ));
                    let dir2_flipped = VariableType::Edge(EdgeType::directed(
                        e2.descriptor.clone(),
                        e2.right.clone(),
                        e2.left.clone(),
                    ));
                    VariableType::is_subtype(&dir1, &dir2_normal)
                        || VariableType::is_subtype(&dir1, &dir2_flipped)
                }
                _ => false,
            },
            (VariableType::List(inner1), VariableType::List(inner2)) => {
                VariableType::is_subtype(inner1, inner2)
            }
            (VariableType::Union(t1a, t1b), _) => {
                VariableType::is_subtype(t1a, t2) && VariableType::is_subtype(t1b, t2)
            }
            (_, VariableType::Union(t2a, t2b)) => {
                VariableType::is_subtype(t1, t2a) || VariableType::is_subtype(t1, t2b)
            }
            _ => false,
        }
    }

    /// Narrows the variable type by intersecting it with schema-defined types.
    pub fn refine(schema: &Schema, node: &VariableType) -> VariableType {
        match node {
            VariableType::Node(_) => {
                let refined: Vec<VariableType> = schema
                    .nodes
                    .iter()
                    .filter(|n| VariableType::is_subtype(n, node))
                    .filter_map(|n| VariableType::meet(n, node).ok())
                    .collect();
                VariableType::join_from_list(refined)
            }
            VariableType::Edge(_) => {
                let refined: Vec<VariableType> = schema
                    .edges
                    .iter()
                    .filter(|e| VariableType::is_subtype(e, node))
                    .filter_map(|e| VariableType::meet(e, node).ok())
                    .collect();
                VariableType::join_from_list(refined)
            }
            VariableType::Union(t1, t2) => VariableType::join(
                VariableType::refine(schema, t1),
                VariableType::refine(schema, t2),
            ),
            VariableType::List(inner) => {
                VariableType::List(Box::new(VariableType::refine(schema, inner)))
            }
            VariableType::Zero => VariableType::Zero,
        }
    }

    /// Determines if a type is inconsistent or unsatisfiable.
    pub fn is_empty(t: &VariableType) -> bool {
        match t {
            VariableType::Zero => true,
            VariableType::Node(node) => DescriptorType::is_empty(&node.0),
            VariableType::Edge(edge) => {
                DescriptorType::is_empty(&edge.descriptor)
                    || DescriptorType::is_empty(&edge.left.0)
                    || DescriptorType::is_empty(&edge.right.0)
            }
            VariableType::Union(t1, t2) => VariableType::is_empty(t1) && VariableType::is_empty(t2),
            VariableType::List(inner) => VariableType::is_empty(inner),
        }
    }
}

// TODO: This is a placeholder. Replace with the real Schema type.
/// Schema containing node and edge type definitions.
#[derive(Default, Clone, Debug)]
pub struct Schema {
    pub nodes: Vec<VariableType>,
    pub edges: Vec<VariableType>,
}

impl Schema {
    pub fn new() -> Self {
        Schema {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: VariableType) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: VariableType) {
        self.edges.push(edge);
    }
}
