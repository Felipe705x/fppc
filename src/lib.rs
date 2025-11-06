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
    use ast::{AttributeLookup, Binop, Descriptor, DescriptorType, PathPattern, Unop};
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
                properties: PropertyType::Open(HashMap::new()),
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
                properties: PropertyType::Open(HashMap::new()),
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
                properties: PropertyType::Open(HashMap::new()),
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
                properties: PropertyType::Open(HashMap::new()),
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
                properties: PropertyType::Open(HashMap::new()),
            },
        });
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
                    properties: PropertyType::Open(HashMap::new()),
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
                    properties: PropertyType::Open(HashMap::new()),
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
    fn test_prioritization() {
        let result = PathPatternParser::new()
            .parse("(x where 11 = 10 and 1 = 2 or 1=2)")
            .unwrap();
        let expected = PathPattern::Filter(
            Box::new(PathPattern::Node(Descriptor {
                variable: Some(Var("x".to_string())),
                descriptor_type: DescriptorType {
                    label: LabelType::Star,
                    properties: PropertyType::Open(HashMap::new()),
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
                    properties: PropertyType::Open(HashMap::new()),
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
                    properties: PropertyType::Open(HashMap::new()),
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
                    properties: PropertyType::Open(HashMap::new()),
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
                    properties: PropertyType::Open(HashMap::new()),
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
    // - Edge patterns (EdgePatternRight, EdgePatternLeft, EdgePatternUndirected)
    // - Concatenation patterns
    // - Repetition patterns (*, +, {n,m})
    // - Union patterns (|)
    // - Questioned patterns (?)
}
