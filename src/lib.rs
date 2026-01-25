use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub grammar);

pub use crate::grammar::{
    DescriptorParser, DescriptorTypeParser, ExprParser, LabelTypeParser, PathPatternParser,
    PropertyTypeParser, SimpleTypeParser,
};

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{
        AttributeLookup, Binop, Descriptor, DescriptorType, EdgeDirection, PathPattern, Unop,
    };
    use ast::{
        BaseType, BinOpKind, Constant, Expr, LabelType, PropertyType, SimpleType, UnOpKind, Var,
    };
    use std::collections::HashMap;

    // ==========================================
    // NODE PATTERN TESTS
    // ==========================================

    #[test]
    fn test_node_empty() {
        let result = PathPatternParser::new().parse("()").unwrap();
        let expected = PathPattern::Node(Descriptor {
            variable: None,
            descriptor_type: DescriptorType {
                label: LabelType::Star,
                properties: PropertyType::open(),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_node_variable() {
        let result = PathPatternParser::new().parse("(x)").unwrap();
        let expected = PathPattern::Node(Descriptor {
            variable: Some(Var("x".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Star,
                properties: PropertyType::open(),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_descriptor() {
        let result = PathPatternParser::new().parse("(x:Person)").unwrap();
        let expected = PathPattern::Node(Descriptor {
            variable: Some(Var("x".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Label("Person".to_string()),
                properties: PropertyType::open(),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_descriptor_empty_record() {
        let result = PathPatternParser::new().parse("(x:Person {})").unwrap();
        let expected = PathPattern::Node(Descriptor {
            variable: Some(Var("x".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Label("Person".to_string()),
                properties: PropertyType::open(),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_descriptor_record() {
        let result = PathPatternParser::new()
            .parse("(x :Person {a: int})")
            .unwrap();
        let mut props = HashMap::new();
        props.insert("a".to_string(), SimpleType::Base(BaseType::Int));
        let expected = PathPattern::Node(Descriptor {
            variable: Some(Var("x".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Label("Person".to_string()),
                properties: PropertyType::Open(props),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_descriptor_record_multiple() {
        let result = PathPatternParser::new()
            .parse("(:Person {a: int, b: bool})")
            .unwrap();
        let mut props = HashMap::new();
        props.insert("a".to_string(), SimpleType::Base(BaseType::Int));
        props.insert("b".to_string(), SimpleType::Base(BaseType::Bool));
        let expected = PathPattern::Node(Descriptor {
            variable: None,
            descriptor_type: DescriptorType {
                label: LabelType::Label("Person".to_string()),
                properties: PropertyType::Open(props),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_descriptor_no_label() {
        let result = PathPatternParser::new()
            .parse("(:{a: int, b: bool})")
            .unwrap();
        let mut props = HashMap::new();
        props.insert("a".to_string(), SimpleType::Base(BaseType::Int));
        props.insert("b".to_string(), SimpleType::Base(BaseType::Bool));
        let expected = PathPattern::Node(Descriptor {
            variable: None,
            descriptor_type: DescriptorType {
                label: LabelType::Star,
                properties: PropertyType::Open(props),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_descriptor_record_closed() {
        let result = PathPatternParser::new()
            .parse("(x :Person {{a: int}})")
            .unwrap();
        let mut props = HashMap::new();
        props.insert("a".to_string(), SimpleType::Base(BaseType::Int));
        let expected = PathPattern::Node(Descriptor {
            variable: Some(Var("x".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Label("Person".to_string()),
                properties: PropertyType::Closed(props),
            },
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_label_and() {
        let result = PathPatternParser::new()
            .parse("(:Person & Company)")
            .unwrap();
        let expected = PathPattern::Node(Descriptor {
            variable: None,
            descriptor_type: DescriptorType {
                label: LabelType::And(
                    Box::new(LabelType::Label("Person".to_string())),
                    Box::new(LabelType::Label("Company".to_string())),
                ),
                properties: PropertyType::open(),
            },
        });
        assert_eq!(result, expected);
    }

    // ==========================================
    // EDGE PATTERN TESTS
    // ==========================================

    #[test]
    fn test_edge_right_empty() {
        let result = PathPatternParser::new().parse("->").unwrap();
        let expected = PathPattern::edge(EdgeDirection::Right, Descriptor::default(), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_right_empty_alt() {
        let result = PathPatternParser::new().parse("-[]->").unwrap();
        let expected = PathPattern::edge(EdgeDirection::Right, Descriptor::default(), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_right_with_descriptor() {
        let result = PathPatternParser::new()
            .parse("-[x:Person {a: int}]->")
            .unwrap();
        let mut props = HashMap::new();
        props.insert("a".to_string(), SimpleType::Base(BaseType::Int));
        let expected = PathPattern::edge(
            EdgeDirection::Right,
            Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Label("Person".to_string()),
                    properties: PropertyType::Open(props),
                },
            },
            None,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_left_empty() {
        let result = PathPatternParser::new().parse("<-").unwrap();
        let expected = PathPattern::edge(EdgeDirection::Left, Descriptor::default(), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_left_empty_alt() {
        let result = PathPatternParser::new().parse("<-[]-").unwrap();
        let expected = PathPattern::edge(EdgeDirection::Left, Descriptor::default(), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_left_with_descriptor() {
        let result = PathPatternParser::new()
            .parse("<-[x:Person {a: int}]-")
            .unwrap();
        let mut props = HashMap::new();
        props.insert("a".to_string(), SimpleType::Base(BaseType::Int));
        let expected = PathPattern::edge(
            EdgeDirection::Left,
            Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Label("Person".to_string()),
                    properties: PropertyType::Open(props),
                },
            },
            None,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_non_directional_empty() {
        let result = PathPatternParser::new().parse("~").unwrap();
        let expected = PathPattern::edge(EdgeDirection::None, Descriptor::default(), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_non_directional_empty_alt() {
        let result = PathPatternParser::new().parse("~[]~").unwrap();
        let expected = PathPattern::edge(EdgeDirection::None, Descriptor::default(), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_non_directional_with_descriptor() {
        let result = PathPatternParser::new()
            .parse("~[x:Person {a: int}]~")
            .unwrap();
        let mut props = HashMap::new();
        props.insert("a".to_string(), SimpleType::Base(BaseType::Int));
        let expected = PathPattern::edge(
            EdgeDirection::None,
            Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Label("Person".to_string()),
                    properties: PropertyType::Open(props),
                },
            },
            None,
        );
        assert_eq!(result, expected);
    }

    // ==========================================
    // CONCATENATION PATTERN TESTS
    // ==========================================

    #[test]
    fn test_concatenation() {
        let result = PathPatternParser::new().parse("(x)~[y]~(z)").unwrap();
        let x = PathPattern::Node(Descriptor {
            variable: Some(Var("x".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Star,
                properties: PropertyType::open(),
            },
        });
        let y = PathPattern::Edge(
            EdgeDirection::None,
            Descriptor {
                variable: Some(Var("y".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            },
        );
        let z = PathPattern::Node(Descriptor {
            variable: Some(Var("z".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Star,
                properties: PropertyType::open(),
            },
        });
        let expected = PathPattern::concat(PathPattern::concat(x, y), z);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_union() {
        let result = PathPatternParser::new().parse("() | ()").unwrap();
        let a = PathPattern::Node(Descriptor::default());
        let b = PathPattern::Node(Descriptor::default());
        let expected = PathPattern::union(a, b);
        assert_eq!(result, expected);
    }

    // ==========================================
    // FILTER PATTERN TESTS
    // ==========================================

    #[test]
    fn test_filter_attribute_gt() {
        let result = PathPatternParser::new().parse("(x where x.a>10)").unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Binop(Binop::new(
                BinOpKind::Gt,
                Expr::AttributeLookup(AttributeLookup::new(
                    Var("x".to_string()),
                    Var("a".to_string()),
                )),
                Expr::Constant(Constant::Int(10)),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_and() {
        let result = PathPatternParser::new()
            .parse("(x where 11>10 and (1 = 2 or 3>='1'))")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Binop(Binop::new(
                BinOpKind::And,
                Expr::Binop(Binop::new(
                    BinOpKind::Gt,
                    Expr::Constant(Constant::Int(11)),
                    Expr::Constant(Constant::Int(10)),
                )),
                Expr::Binop(Binop::new(
                    BinOpKind::Or,
                    Expr::Binop(Binop::new(
                        BinOpKind::Eq,
                        Expr::Constant(Constant::Int(1)),
                        Expr::Constant(Constant::Int(2)),
                    )),
                    Expr::Binop(Binop::new(
                        BinOpKind::Ge,
                        Expr::Constant(Constant::Int(3)),
                        Expr::Constant(Constant::String("1".to_string())),
                    )),
                )),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_on_edge() {
        let result = PathPatternParser::new()
            .parse("(x)-[y where y.a>10]->(z)")
            .unwrap();
        let x = PathPattern::Node(Descriptor {
            variable: Some(Var("x".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Star,
                properties: PropertyType::open(),
            },
        });
        let y = PathPattern::Filter(
            Box::new(PathPattern::Edge(
                EdgeDirection::Right,
                Descriptor {
                    variable: Some(Var("y".to_string())),
                    descriptor_type: DescriptorType {
                        label: LabelType::Star,
                        properties: PropertyType::open(),
                    },
                },
            )),
            Expr::Binop(Binop::new(
                BinOpKind::Gt,
                Expr::AttributeLookup(AttributeLookup::new(
                    Var("y".to_string()),
                    Var("a".to_string()),
                )),
                Expr::Constant(Constant::Int(10)),
            )),
        );
        let z = PathPattern::Node(Descriptor {
            variable: Some(Var("z".to_string())),
            descriptor_type: DescriptorType {
                label: LabelType::Star,
                properties: PropertyType::open(),
            },
        });
        let expected = PathPattern::concat(PathPattern::concat(x, y), z);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_prioritization() {
        let result = PathPatternParser::new()
            .parse("(x where 11 = 10 and 1 = 2 or 1=2)")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Binop(Binop::new(
                BinOpKind::Or,
                Expr::Binop(Binop::new(
                    BinOpKind::And,
                    Expr::Binop(Binop::new(
                        BinOpKind::Eq,
                        Expr::Constant(Constant::Int(11)),
                        Expr::Constant(Constant::Int(10)),
                    )),
                    Expr::Binop(Binop::new(
                        BinOpKind::Eq,
                        Expr::Constant(Constant::Int(1)),
                        Expr::Constant(Constant::Int(2)),
                    )),
                )),
                Expr::Binop(Binop::new(
                    BinOpKind::Eq,
                    Expr::Constant(Constant::Int(1)),
                    Expr::Constant(Constant::Int(2)),
                )),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_logical() {
        let result = PathPatternParser::new()
            .parse("(x where true and 1>2)")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Binop(Binop::new(
                BinOpKind::And,
                Expr::Constant(Constant::Bool(true)),
                Expr::Binop(Binop::new(
                    BinOpKind::Gt,
                    Expr::Constant(Constant::Int(1)),
                    Expr::Constant(Constant::Int(2)),
                )),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple_arithmetic() {
        let result = PathPatternParser::new()
            .parse("(x where x.a>x.b>1)")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Binop(Binop::new(
                BinOpKind::Gt,
                Expr::Binop(Binop::new(
                    BinOpKind::Gt,
                    Expr::AttributeLookup(AttributeLookup::new(
                        Var("x".to_string()),
                        Var("a".to_string()),
                    )),
                    Expr::AttributeLookup(AttributeLookup::new(
                        Var("x".to_string()),
                        Var("b".to_string()),
                    )),
                )),
                Expr::Constant(Constant::Int(1)),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unop_1() {
        let result = PathPatternParser::new()
            .parse("(x WHERE not x.status)")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Unop(Unop::new(
                UnOpKind::Not,
                Expr::AttributeLookup(AttributeLookup::new(
                    Var("x".to_string()),
                    Var("status".to_string()),
                )),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unop_2() {
        let result = PathPatternParser::new()
            .parse("(x WHERE -x.status>0)")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Binop(Binop::new(
                BinOpKind::Gt,
                Expr::Unop(Unop::new(
                    UnOpKind::Neg,
                    Expr::AttributeLookup(AttributeLookup::new(
                        Var("x".to_string()),
                        Var("status".to_string()),
                    )),
                )),
                Expr::Constant(Constant::Int(0)),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_unop_3() {
        let result = PathPatternParser::new()
            .parse("((x) WHERE -x.status>0)")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::open(),
                },
            })),
            Expr::Binop(Binop::new(
                BinOpKind::Gt,
                Expr::Unop(Unop::new(
                    UnOpKind::Neg,
                    Expr::AttributeLookup(AttributeLookup::new(
                        Var("x".to_string()),
                        Var("status".to_string()),
                    )),
                )),
                Expr::Constant(Constant::Int(0)),
            )),
        );
        assert_eq!(result, expected);
    }

    // ==========================================
    // UNIMPLEMENTED FEATURES (skipped)
    // ==========================================
    // - Repetition patterns (*, +, {n,m})
    // - Questioned patterns (?)
}
