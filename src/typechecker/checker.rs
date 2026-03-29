use crate::ast::{
    BaseType, BinOpKind, Constant, Descriptor, EdgeDirection, Expr, PathPattern, Quantifier,
    SimpleType, UnOpKind,
};
use crate::parser;

use super::path_type::{Direction, PathType};
use super::schema::Schema;
use super::type_environment::TypeEnvironment;
use super::variable_type::{EdgeKind, EdgeType, NodeType, VariableType};

// -----------------------------------------------
// TypecheckResult
// -----------------------------------------------

/// Result of type checking a path pattern.
#[derive(Clone, Debug)]
pub struct TypecheckResult {
    /// The inferred path type.
    pub path: PathType,
    /// The type environment after checking.
    pub env: TypeEnvironment,
    /// Whether the check completed without errors.
    pub ok: bool,
    /// Whether the result is unsatisfiable (path or environment is empty).
    pub empty: bool,
}

impl TypecheckResult {
    fn new(path: PathType, env: TypeEnvironment) -> Self {
        TypecheckResult {
            path,
            env,
            ok: true,
            empty: false,
        }
    }
}

// -----------------------------------------------
// Typechecker
// -----------------------------------------------

/// The main typechecker for GQL path patterns.
pub struct Typechecker {
    pub schema: Schema,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Typechecker {
    /// Creates a new typechecker with the given schema.
    pub fn new(schema: Schema) -> Self {
        Typechecker {
            schema,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Creates a new typechecker with the permissive star schema (no constraints).
    pub fn untyped() -> Self {
        Typechecker::new(Schema::star())
    }

    /// Parses and type-checks a query string.
    pub fn check(&mut self, query: &str) -> TypecheckResult {
        match parser::parse(query) {
            Ok(parsed) => self.check_parsed(&parsed),
            Err(e) => {
                self.errors.clear();
                self.warnings.clear();
                self.errors.push(format!("Parse error: {}", e));
                TypecheckResult {
                    path: PathType::Zero,
                    env: TypeEnvironment::default(),
                    ok: false,
                    empty: true,
                }
            }
        }
    }

    /// Type-checks a parsed path pattern.
    pub fn check_parsed(&mut self, parsed: &PathPattern) -> TypecheckResult {
        self.errors.clear();
        self.warnings.clear();

        let mut r = self.check_path_pattern(parsed);

        if !self.errors.is_empty() {
            r.ok = false;
        }
        r.empty = r.path.is_unsatisfiable() || r.env.is_empty();
        r
    }

    // -----------------------------------------------
    // Path pattern checking
    // -----------------------------------------------

    fn check_path_pattern(&mut self, node: &PathPattern) -> TypecheckResult {
        match node {
            PathPattern::Node(desc) => {
                let t = self.refine_pattern_node(desc);
                let p = PathType::to_path_type(&t, Direction::Any);
                let c = TypeEnvironment::create_context(desc, t);
                TypecheckResult::new(p, c)
            }

            PathPattern::Edge(dir, desc) => {
                let t = self.refine_pattern_edge(dir, desc);
                let path_dir = Self::to_path_direction(dir);
                let p = PathType::to_path_type(&t, path_dir);
                let c = TypeEnvironment::create_context(desc, t);
                TypecheckResult::new(p, c)
            }

            PathPattern::Concat(p1, p2) => {
                let r1 = self.check_path_pattern(p1);
                let r2 = self.check_path_pattern(p2);

                let cm = match TypeEnvironment::meet(&self.schema, &r1.env, &r2.env) {
                    Ok(env) => env,
                    Err(e) => {
                        self.errors
                            .push(format!("Concatenation of contexts failed: {}", e));
                        r1.env.clone()
                    }
                };

                let p = PathType::meet(&self.schema, &r1.path, &r2.path);
                TypecheckResult::new(p, cm)
            }

            PathPattern::Filter(pattern, expr) => {
                let r = self.check_path_pattern(pattern);
                let t = self.check_expr(expr, &r.env);

                let bool_type = SimpleType::Base(BaseType::Bool);
                if SimpleType::is_empty(&SimpleType::meet(&t, &bool_type)) {
                    self.warnings.push(format!(
                        "Filter expression has type {:?}, which is definitively not a boolean",
                        t
                    ));
                    TypecheckResult::new(PathType::Zero, r.env)
                } else {
                    r
                }
            }

            PathPattern::Union(p1, p2) => {
                let r1 = self.check_path_pattern(p1);
                let r2 = self.check_path_pattern(p2);
                TypecheckResult::new(
                    PathType::union(r1.path, r2.path),
                    TypeEnvironment::union(&r1.env, &r2.env),
                )
            }

            PathPattern::Quantified(p, quantifier) => {
                let r = self.check_path_pattern(p);
                let lb = Self::lower_bound(quantifier);

                let effective_lb = if !r.path.is_empty() {
                    lb.min(3)
                } else {
                    self.warnings
                        .push("Repeat expression must have length > 0".to_string());
                    lb
                };

                TypecheckResult::new(self.pow_path_type(&r.path, effective_lb), r.env.to_list())
            }

            PathPattern::Questioned(p) => {
                // ? is equivalent to {0,1}, lb = 0
                let r = self.check_path_pattern(p);

                if r.path.is_empty() {
                    self.warnings
                        .push("Repeat expression must have length > 0".to_string());
                }

                TypecheckResult::new(self.pow_path_type(&r.path, 0), r.env.to_list())
            }
        }
    }

    // -----------------------------------------------
    // Expression checking
    // -----------------------------------------------

    fn check_expr(&mut self, e: &Expr, env: &TypeEnvironment) -> SimpleType {
        match e {
            Expr::Constant(c) => match c {
                Constant::Int(_) => SimpleType::Base(BaseType::Int),
                Constant::String(_) => SimpleType::Base(BaseType::String),
                Constant::Bool(_) => SimpleType::Base(BaseType::Bool),
            },

            Expr::TypeLiteral(t) => t.clone(),

            Expr::AttributeLookup(entity, attribute) => match env.get(&entity.0) {
                Some(t) => {
                    if matches!(t, VariableType::Zero) {
                        self.warnings
                            .push(format!("Variable {} is bound to empty type", entity.0));
                        return SimpleType::Zero;
                    }
                    let at = t.get_attribute(&attribute.0);
                    if SimpleType::is_empty(&at) {
                        self.warnings
                            .push(format!("Attribute {} not found in {:?}", attribute.0, t));
                    }
                    at
                }
                None => {
                    self.errors
                        .push(format!("Variable {} not found in context", entity.0));
                    SimpleType::Zero
                }
            },

            Expr::Binop(op, e1, e2) => {
                let t1 = self.check_expr(e1, env);

                // Special handling for type operations
                match op {
                    BinOpKind::Is => {
                        if let Expr::TypeLiteral(_) = e2.as_ref() {
                            return SimpleType::Base(BaseType::Bool);
                        }
                    }
                    BinOpKind::As => {
                        if let Expr::TypeLiteral(t) = e2.as_ref() {
                            return t.clone();
                        }
                    }
                    _ => {}
                }

                let t2 = self.check_expr(e2, env);

                match Self::binop_delta(op) {
                    Some((expected_t1, expected_t2, result_t)) => {
                        if SimpleType::gradual_eq(&t1, &expected_t1)
                            && SimpleType::gradual_eq(&t2, &expected_t2)
                        {
                            result_t
                        } else {
                            self.warnings.push(format!(
                                "Binop {:?} between types {:?} and {:?} is not defined",
                                op, t1, t2
                            ));
                            SimpleType::Zero
                        }
                    }
                    None => {
                        self.errors.push(format!(
                            "Binop {:?} undefined between types {:?} and {:?}",
                            op, t1, t2
                        ));
                        SimpleType::Zero
                    }
                }
            }

            Expr::Unop(op, inner) => {
                let t = self.check_expr(inner, env);

                match Self::unop_delta(op) {
                    Some((expected_t, result_t)) => {
                        if SimpleType::gradual_eq(&t, &expected_t) {
                            result_t
                        } else {
                            self.warnings
                                .push(format!("Unop {:?} on type {:?} is not defined", op, t));
                            SimpleType::Zero
                        }
                    }
                    None => {
                        self.errors
                            .push(format!("Unop {:?} undefined for type {:?}", op, t));
                        SimpleType::Zero
                    }
                }
            }
        }
    }

    // -----------------------------------------------
    // Delta rules for operators
    // -----------------------------------------------

    /// Returns the type signature `(input1, input2, output)` for a binary operator.
    fn binop_delta(op: &BinOpKind) -> Option<(SimpleType, SimpleType, SimpleType)> {
        let int = SimpleType::Base(BaseType::Int);
        let bool_t = SimpleType::Base(BaseType::Bool);
        let star = SimpleType::Star;

        match op {
            // Arithmetic: Int × Int → Int
            BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div => {
                Some((int.clone(), int.clone(), int))
            }
            // Comparison: Int × Int → Bool
            BinOpKind::Lt | BinOpKind::Gt | BinOpKind::Le | BinOpKind::Ge => {
                Some((int.clone(), int, bool_t))
            }
            // Equality: ⋆ × ⋆ → Bool (polymorphic)
            BinOpKind::Eq | BinOpKind::Ne => Some((star.clone(), star, bool_t)),
            // Logical: Bool × Bool → Bool
            BinOpKind::And | BinOpKind::Or => Some((bool_t.clone(), bool_t.clone(), bool_t)),
            // Type operations handled separately
            BinOpKind::Is | BinOpKind::As => None,
        }
    }

    /// Returns the type signature `(input, output)` for a unary operator.
    fn unop_delta(op: &UnOpKind) -> Option<(SimpleType, SimpleType)> {
        let int = SimpleType::Base(BaseType::Int);
        let bool_t = SimpleType::Base(BaseType::Bool);

        match op {
            UnOpKind::Neg => Some((int.clone(), int)),
            UnOpKind::Not => Some((bool_t.clone(), bool_t)),
        }
    }

    // -----------------------------------------------
    // Helpers
    // -----------------------------------------------

    /// Converts an AST `EdgeDirection` to a path `Direction`.
    fn to_path_direction(dir: &EdgeDirection) -> Direction {
        match dir {
            EdgeDirection::Right => Direction::Right,
            EdgeDirection::Left => Direction::Left,
            EdgeDirection::Any => Direction::Any,
            EdgeDirection::None => Direction::Undirected,
        }
    }

    /// Extracts the lower bound from a quantifier.
    fn lower_bound(q: &Quantifier) -> u64 {
        match q {
            Quantifier::Star => 0,
            Quantifier::Plus => 1,
            Quantifier::Fixed(n) => *n,
            Quantifier::Range(lo, _) => lo.unwrap_or(0),
        }
    }

    /// Refines a node pattern's descriptor against the schema.
    fn refine_pattern_node(&self, desc: &Descriptor) -> VariableType {
        let vt = VariableType::node_with(desc.descriptor_type.clone());
        VariableType::refine(&self.schema, &vt)
    }

    /// Refines an edge pattern's descriptor against the schema.
    fn refine_pattern_edge(&self, dir: &EdgeDirection, desc: &Descriptor) -> VariableType {
        let kind = match dir {
            EdgeDirection::Right | EdgeDirection::Left | EdgeDirection::Any => EdgeKind::Directed,
            EdgeDirection::None => EdgeKind::Undirected,
        };
        let vt = VariableType::Edge(EdgeType {
            descriptor: desc.descriptor_type.clone(),
            left: NodeType::default(),
            right: NodeType::default(),
            kind,
        });
        VariableType::refine(&self.schema, &vt)
    }

    /// Computes p^n — the n-fold meet of a path type with itself.
    ///
    /// - p^0 = default node path (empty path)
    /// - p^1 = p
    /// - p^n = meet(p, p^(n-1))
    fn pow_path_type(&self, p: &PathType, n: u64) -> PathType {
        match n {
            0 => PathType::default(),
            1 => p.clone(),
            _ => PathType::meet(&self.schema, p, &self.pow_path_type(p, n - 1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BaseType, DescriptorType, LabelType, PropertyType, SimpleType};
    use std::collections::HashMap;

    // ====================================================
    // Schema builders
    // ====================================================

    /// Fraud detection schema (derived from fraud.json):
    ///
    /// Nodes:
    ///   - Account {owner: str, isBlocked: bool}
    ///   - Dummy & Person {owner: str, isBlocked: bool, isDummy: bool}
    ///
    /// Edges (all directed):
    ///   - Transfer {amount: int}  Account → Account
    ///   - Foo {amount: int}       Account → Dummy&Person
    fn fraud_schema() -> Schema {
        let account_desc = DescriptorType::new(
            LabelType::Label("Account".into()),
            PropertyType::Closed(HashMap::from([
                ("owner".into(), SimpleType::Base(BaseType::String)),
                ("isBlocked".into(), SimpleType::Base(BaseType::Bool)),
            ])),
        );
        let dummy_person_desc = DescriptorType::new(
            LabelType::and(
                LabelType::Label("Dummy".into()),
                LabelType::Label("Person".into()),
            ),
            PropertyType::Closed(HashMap::from([
                ("owner".into(), SimpleType::Base(BaseType::String)),
                ("isBlocked".into(), SimpleType::Base(BaseType::Bool)),
                ("isDummy".into(), SimpleType::Base(BaseType::Bool)),
            ])),
        );

        let account_node = NodeType(account_desc);
        let dummy_person_node = NodeType(dummy_person_desc);

        let transfer_desc = DescriptorType::new(
            LabelType::Label("Transfer".into()),
            PropertyType::Closed(HashMap::from([(
                "amount".into(),
                SimpleType::Base(BaseType::Int),
            )])),
        );
        let foo_desc = DescriptorType::new(
            LabelType::Label("Foo".into()),
            PropertyType::Closed(HashMap::from([(
                "amount".into(),
                SimpleType::Base(BaseType::Int),
            )])),
        );

        Schema::new(
            vec![
                VariableType::Node(account_node.clone()),
                VariableType::Node(dummy_person_node.clone()),
            ],
            vec![
                VariableType::Edge(EdgeType::directed(
                    transfer_desc,
                    account_node.clone(),
                    account_node.clone(),
                )),
                VariableType::Edge(EdgeType::directed(
                    foo_desc,
                    account_node,
                    dummy_person_node,
                )),
            ],
        )
    }

    /// Social network schema (derived from social-network.json):
    ///
    /// Nodes:
    ///   - Person & Teacher {name: str, status: str}
    ///   - Person & Student {name: str, status: int}
    ///   - Comment {content: str, status: bool}
    ///
    /// Edges:
    ///   - Knows {since: int}  Teacher ↔ Student  (undirected)
    ///   - Likes {}             Teacher → Comment  (directed)
    ///   - Author {}            Comment → Student  (directed)
    fn social_schema() -> Schema {
        let teacher_desc = DescriptorType::new(
            LabelType::and(
                LabelType::Label("Person".into()),
                LabelType::Label("Teacher".into()),
            ),
            PropertyType::Closed(HashMap::from([
                ("name".into(), SimpleType::Base(BaseType::String)),
                ("status".into(), SimpleType::Base(BaseType::String)),
            ])),
        );
        let student_desc = DescriptorType::new(
            LabelType::and(
                LabelType::Label("Person".into()),
                LabelType::Label("Student".into()),
            ),
            PropertyType::Closed(HashMap::from([
                ("name".into(), SimpleType::Base(BaseType::String)),
                ("status".into(), SimpleType::Base(BaseType::Int)),
            ])),
        );
        let comment_desc = DescriptorType::new(
            LabelType::Label("Comment".into()),
            PropertyType::Closed(HashMap::from([
                ("content".into(), SimpleType::Base(BaseType::String)),
                ("status".into(), SimpleType::Base(BaseType::Bool)),
            ])),
        );

        let teacher_node = NodeType(teacher_desc);
        let student_node = NodeType(student_desc);
        let comment_node = NodeType(comment_desc);

        let knows_desc = DescriptorType::new(
            LabelType::Label("Knows".into()),
            PropertyType::Closed(HashMap::from([(
                "since".into(),
                SimpleType::Base(BaseType::Int),
            )])),
        );
        let likes_desc =
            DescriptorType::new(LabelType::Label("Likes".into()), PropertyType::closed());
        let author_desc =
            DescriptorType::new(LabelType::Label("Author".into()), PropertyType::closed());

        Schema::new(
            vec![
                VariableType::Node(teacher_node.clone()),
                VariableType::Node(student_node.clone()),
                VariableType::Node(comment_node.clone()),
            ],
            vec![
                // Knows {since: int} — directed for refinement purposes
                VariableType::Edge(EdgeType::directed(
                    knows_desc,
                    teacher_node.clone(),
                    student_node.clone(),
                )),
                // Likes is directed (->)
                VariableType::Edge(EdgeType::directed(
                    likes_desc,
                    teacher_node,
                    comment_node.clone(),
                )),
                // Author is directed (->)
                VariableType::Edge(EdgeType::directed(author_desc, comment_node, student_node)),
            ],
        )
    }

    // ====================================================
    // Basic node pattern tests (untyped / star schema)
    // ====================================================

    #[test]
    fn test_node_empty() {
        assert!(Typechecker::untyped().check("()").ok);
    }

    #[test]
    fn test_node_var() {
        assert!(Typechecker::untyped().check("(x)").ok);
    }

    #[test]
    fn test_node_non_empty() {
        assert!(Typechecker::untyped().check("(x: Person {owner: str})").ok);
    }

    // test_not skipped — label negation (!) not implemented

    // ====================================================
    // Basic edge pattern tests
    // ====================================================

    #[test]
    fn test_node_variable() {
        assert!(Typechecker::untyped().check("->").ok);
    }

    #[test]
    fn test_node_variable_2() {
        assert!(
            Typechecker::untyped()
                .check("-[x: Transfer {amount: int}]->")
                .ok
        );
    }

    // ====================================================
    // Concatenation tests
    // ====================================================

    #[test]
    fn test_concat() {
        assert!(Typechecker::untyped().check("(x:Account)(y:Account)").ok);
    }

    #[test]
    fn test_concat_1() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int})(x:Person {b: bool, a: bool})")
                .ok
        );
    }

    #[test]
    fn test_concat_node_edge() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int})-[y:Person {b: bool, a: bool}]->")
                .ok
        );
    }

    #[test]
    fn test_concat_node_node() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int}) (y: {b: bool})")
                .ok
        );
    }

    #[test]
    fn test_concat_node_edge_node() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int}) -[y:Person {b: bool, a: bool}]-> (z: {b: bool})")
                .ok
        );
    }

    #[test]
    fn test_concat_edge_edge() {
        assert!(
            Typechecker::untyped()
                .check("-[x:Person]->-[y:University]->")
                .ok
        );
    }

    #[test]
    fn test_concat_node_node_node_edge() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int}) (y: {b: bool}) (z: {c: str}) -[w:Person]->")
                .ok
        );
    }

    #[test]
    fn test_concat_node_node_empty() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int}) (y: {a: bool})")
                .ok
        );
    }

    // ====================================================
    // Schema-based emptiness tests (fraud)
    // ====================================================

    #[test]
    fn test_empty_1() {
        let mut tc = Typechecker::new(fraud_schema());
        let r = tc.check("(x: {a: int})");
        assert!(r.ok);
        assert!(r.empty);
    }

    #[test]
    fn test_empty_2() {
        let mut tc = Typechecker::new(fraud_schema());
        let r = tc.check("-[x: {nonExistant: int}]->");
        assert!(r.ok);
        assert!(r.empty);
    }

    // ====================================================
    // Filter / WHERE tests
    // ====================================================

    #[test]
    fn test_where() {
        assert!(
            Typechecker::untyped()
                .check("(: {b: str}) (x: {a: str, b: str} where x.a = x.b)")
                .ok
        );
    }

    #[test]
    fn test_filter_4() {
        // Just check it doesn't panic
        let _ = Typechecker::untyped().check("-[y WHERE y.a]->");
    }

    #[test]
    fn test_no_warnings() {
        let mut tc = Typechecker::untyped();
        tc.check("-[y WHERE y.amount>=3500000]->");
        assert!(tc.warnings.is_empty());
    }

    #[test]
    fn test_bad_attribute() {
        assert!(
            Typechecker::untyped()
                .check("(x: {amount: int} WHERE x.amout > 1000)")
                .ok
        );
    }

    // ====================================================
    // Union tests
    // ====================================================

    #[test]
    fn test_union() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int}) | (y: {b: bool})")
                .ok
        );
    }

    #[test]
    fn test_union_2() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int}) | (x: {b: bool})")
                .ok
        );
    }

    #[test]
    fn test_union_heterogeneous() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int}) | -[x: {a: bool}]->")
                .ok
        );
    }

    #[test]
    fn test_union_concat_fail() {
        assert!(
            Typechecker::untyped()
                .check("((:{a: int}) | (:{a: bool})) (:{a: str})")
                .ok
        );
    }

    #[test]
    fn test_union_concat_ok() {
        assert!(
            Typechecker::untyped()
                .check("((:{a: int}) | (:{a: bool})) (:{a: int})")
                .ok
        );
    }

    #[test]
    fn test_zero_path() {
        assert!(
            Typechecker::untyped()
                .check("((:{a: int})(:{a:bool})) | ()")
                .ok
        );
    }

    // ====================================================
    // Repetition / quantifier tests
    // ====================================================

    #[test]
    fn test_repetition_1() {
        assert!(Typechecker::untyped().check("(x: {a: int}){1,2}").ok);
    }

    #[test]
    fn test_repetition_2() {
        assert!(Typechecker::untyped().check("-[x: {a:int}]-> {1,2}").ok);
    }

    #[test]
    fn test_repetition_3() {
        assert!(
            Typechecker::new(fraud_schema())
                .check("(y)(-[x: {a:int}]-> {1,2}){2,3}")
                .ok
        );
    }

    #[test]
    fn test_repetition_4() {
        // Just check it doesn't panic
        let _ = Typechecker::untyped().check("-{1,2}");
    }

    #[test]
    fn test_foo() {
        assert!(Typechecker::untyped().check("-[x]->{1,3}").ok);
    }

    // ====================================================
    // Misc pattern tests
    // ====================================================

    #[test]
    fn test_bad_pop() {
        assert!(Typechecker::untyped().check("()()").ok);
    }

    #[test]
    fn test_readers_digest_ex1() {
        assert!(
            Typechecker::untyped()
                .check("(x)-[z:Transfer WHERE z.amount>1000000]->(y WHERE y.isBlocked=true)")
                .ok
        );
    }

    // ====================================================
    // Type operations: is / as
    // ====================================================

    #[test]
    fn test_is() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int} WHERE x.a is int)")
                .ok
        );
    }

    #[test]
    fn test_as() {
        assert!(
            Typechecker::untyped()
                .check("(x: {a: int} WHERE x.a as bool)")
                .ok
        );
    }

    // ====================================================
    // Error detection tests
    // ====================================================

    #[test]
    fn test_example21() {
        // Same variable for node and edge → context meet fails → error
        assert!(!Typechecker::untyped().check("(x)-[x]->").ok);
    }

    #[test]
    fn test_example22() {
        // Same variable with incompatible property types → empty
        assert!(
            Typechecker::untyped()
                .check("(x: {status: bool} WHERE x.status = true)-[:Knows]->(x: {status: str})")
                .empty
        );
    }

    #[test]
    fn test_example23() {
        let query = "(x: {stauts: int} WHERE x.stauts > 0)";

        // Part 1: untyped — ok, no warnings
        let mut tc = Typechecker::untyped();
        assert!(tc.check(query).ok);
        assert!(tc.warnings.is_empty());

        // Part 2: schema with closed {status: bool} — "stauts" doesn't exist → empty
        let schema = Schema::new(
            vec![VariableType::node_with(DescriptorType::new(
                LabelType::Star,
                PropertyType::Closed(HashMap::from([(
                    "status".into(),
                    SimpleType::Base(BaseType::Bool),
                )])),
            ))],
            vec![],
        );
        let mut tc2 = Typechecker::new(schema);
        assert!(tc2.check(query).empty);
    }

    #[test]
    fn test_example24() {
        let schema = Schema::new(
            vec![VariableType::node_with(DescriptorType::new(
                LabelType::Star,
                PropertyType::Closed(HashMap::from([(
                    "status".into(),
                    SimpleType::Base(BaseType::Bool),
                )])),
            ))],
            vec![],
        );
        let mut tc = Typechecker::new(schema);

        let query = "(x: {status: bool} WHERE x.status > 0)";
        assert!(tc.check(query).empty);
    }

    #[test]
    fn test_unbound_variable() {
        assert!(!Typechecker::untyped().check("(y WHERE x.status = true)").ok);
    }

    // ====================================================
    // Label subtype test (pure type system)
    // ====================================================

    #[test]
    fn test_is_subtype() {
        let l = LabelType::and(
            LabelType::Label("Person".into()),
            LabelType::Label("Teacher".into()),
        );
        assert!(LabelType::is_subtype(&l, &l));
    }

    // ====================================================
    // Social network schema tests
    // ====================================================

    #[test]
    fn test_social_1() {
        let mut tc = Typechecker::new(social_schema());
        let query = "(x WHERE x.status=true)";
        assert!(!tc.check(query).empty);
    }

    #[test]
    fn test_paper_example_1_part_1() {
        let mut tc = Typechecker::new(social_schema());
        let query = "(x : Teacher) -[: Likes]->";
        assert!(!tc.check(query).empty);
    }

    #[test]
    fn test_paper_example_1_part_2() {
        let mut tc = Typechecker::new(social_schema());
        let query = "(: Student ) -[y : Knows WHERE y . since < 2019]- (x)";
        assert!(!tc.check(query).empty);
    }

    #[test]
    fn test_incompatible_records() {
        let mut tc = Typechecker::new(social_schema());
        let query = "(x: {{a: bool}})(x: {b: int})";
        assert!(tc.check(query).empty);
    }

    // ====================================================
    // Unary operator tests (fraud schema)
    // ====================================================

    #[test]
    fn test_unop_1() {
        let mut tc = Typechecker::new(fraud_schema());
        assert!(!tc.check("(x WHERE not x.isBlocked)").empty);
    }

    #[test]
    fn test_unop_2() {
        let mut tc = Typechecker::new(fraud_schema());
        assert!(!tc.check("-[x WHERE -x.amount < 0]->").empty);
    }
}
