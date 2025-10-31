use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub grammar);

pub use crate::grammar::{
    LabelTypeParser, SimpleTypeParser, PropertyTypeParser, 
    DescriptorTypeParser, DescriptorParser, PathPatternParser,
    ExprParser
};

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Var, LabelType, PropertyType, SimpleType, BaseType, Expr, Constant, BinOpKind, UnOpKind};

    // ==========================================
    // NODE PATTERN TESTS (from parser_test.py)
    // ==========================================

    #[test]
    fn test_node_empty() {
        // () -> NodePattern(Descriptor(None, DescriptorType.star()))
        let result = PathPatternParser::new().parse("()").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, None);
        assert_eq!(desc.descriptor_type.label, LabelType::Star);
        assert!(matches!(desc.descriptor_type.properties, PropertyType::Open(_)));
    }

    #[test]
    fn test_node_variable() {
        // (x) -> NodePattern(Descriptor(Var("x"), DescriptorType.star()))
        let result = PathPatternParser::new().parse("(x)").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert_eq!(desc.descriptor_type.label, LabelType::Star);
    }

    #[test]
    fn test_descriptor() {
        // (x:Person) -> NodePattern(Descriptor(Var("x"), DescriptorType(Label("Person"), OpenPropertyType())))
        let result = PathPatternParser::new().parse("(x:Person)").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => assert!(map.is_empty()),
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_descriptor_empty_record() {
        // (x:Person {}) -> NodePattern(Descriptor(Var("x"), DescriptorType(Label("Person"), OpenPropertyType())))
        let result = PathPatternParser::new().parse("(x:Person {})").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => assert!(map.is_empty()),
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_descriptor_record() {
        // (x :Person {a: int})
        let result = PathPatternParser::new().parse("(x :Person {a: int})").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => {
                assert_eq!(map.len(), 1);
                assert!(matches!(map.get("a"), Some(SimpleType::Base(BaseType::Int))));
            }
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_descriptor_record_multiple() {
        // (:Person {a: int, b: bool})
        let result = PathPatternParser::new().parse("(:Person {a: int, b: bool})").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, None);
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => {
                assert_eq!(map.len(), 2);
                assert!(matches!(map.get("a"), Some(SimpleType::Base(BaseType::Int))));
                assert!(matches!(map.get("b"), Some(SimpleType::Base(BaseType::Bool))));
            }
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_descriptor_no_label() {
        // (:{a: int, b: bool}) -> DescriptorType(StarLabel(), OpenPropertyType(...))
        let result = PathPatternParser::new().parse("(:{a: int, b: bool})").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, None);
        assert!(matches!(desc.descriptor_type.label, LabelType::Star));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => {
                assert_eq!(map.len(), 2);
                assert!(matches!(map.get("a"), Some(SimpleType::Base(BaseType::Int))));
                assert!(matches!(map.get("b"), Some(SimpleType::Base(BaseType::Bool))));
            }
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_descriptor_record_closed() {
        // (x :Person {{a: int}}) -> ClosedPropertyType
        let result = PathPatternParser::new().parse("(x :Person {{a: int}})").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Closed(map) => {
                assert_eq!(map.len(), 1);
                assert!(matches!(map.get("a"), Some(SimpleType::Base(BaseType::Int))));
            }
            _ => panic!("Expected Closed properties"),
        }
    }

    #[test]
    fn test_label_and() {
        // (:Person & Company) -> NodePattern(Descriptor(None, DescriptorType(AndLabel(Label("Person"), Label("Company")), OpenPropertyType())))
        let result = PathPatternParser::new().parse("(:Person & Company)").unwrap();
        let node = match result {
            ast::PathPattern::Node(n) => n,
            _ => panic!("Expected PathPattern::Node"),
        };
        let desc = &node.descriptor;
        match &desc.descriptor_type.label {
            LabelType::And(l1, l2) => {
                assert_eq!(**l1, LabelType::Label("Person".to_string()));
                assert_eq!(**l2, LabelType::Label("Company".to_string()));
            }
            _ => panic!("Expected AndLabel"),
        }
    }

    // ==========================================
    // EXPRESSION TESTS (from parser_test.py filter tests)
    // ==========================================

    #[test]
    fn test_expr_attribute_gt() {
        // x.a>10
        let result = ExprParser::new().parse("x.a>10").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Gt);
                assert!(matches!(*binop.e1, Expr::AttributeLookup(_)));
                assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(10))));
            }
            _ => panic!("Expected Binop"),
        }
    }

    #[test]
    fn test_expr_simple_logical() {
        // true and 1>2
        let result = ExprParser::new().parse("true and 1>2").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::And);
                assert!(matches!(*binop.e1, Expr::Constant(Constant::Bool(true))));
                assert!(matches!(*binop.e2, Expr::Binop(_)));
            }
            _ => panic!("Expected Binop"),
        }
    }

    #[test]
    fn test_expr_chained_comparison() {
        // x.a>x.b>1 should parse as (x.a>x.b)>1
        let result = ExprParser::new().parse("x.a>x.b>1").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Gt);
                assert!(matches!(*binop.e1, Expr::Binop(_)));
                assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(1))));
            }
            _ => panic!("Expected Binop"),
        }
    }

    #[test]
    fn test_expr_precedence() {
        // 11 = 10 and 1 = 2 or 1=2
        // Should parse as: (11 = 10 and 1 = 2) or 1=2
        let result = ExprParser::new().parse("11=10 and 1=2 or 1=2").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Or);
            }
            _ => panic!("Expected Binop with 'or'"),
        }
    }

    #[test]
    fn test_expr_unary_not() {
        // not x.status
        let result = ExprParser::new().parse("not x.status").unwrap();
        match result {
            Expr::Unop(unop) => {
                assert_eq!(unop.op, UnOpKind::Not);
                assert!(matches!(*unop.e, Expr::AttributeLookup(_)));
            }
            _ => panic!("Expected Unop"),
        }
    }

    #[test]
    fn test_expr_unary_neg() {
        // -x.status>0
        let result = ExprParser::new().parse("-x.status>0").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Gt);
                assert!(matches!(*binop.e1, Expr::Unop(_)));
                assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(0))));
            }
            _ => panic!("Expected Binop"),
        }
    }

    #[test]
    fn test_expr_case_insensitive() {
        // TRUE AND FALSE OR NOT TRUE
        let result = ExprParser::new().parse("TRUE AND FALSE OR NOT TRUE").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Or);
            }
            _ => panic!("Expected Binop with 'or'"),
        }
    }

    #[test]
    fn test_expr_is_as_operators() {
        // x is y
        let result = ExprParser::new().parse("x is y").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Is);
            }
            _ => panic!("Expected Binop with 'is'"),
        }
        
        // x as y
        let result = ExprParser::new().parse("x as y").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::As);
            }
            _ => panic!("Expected Binop with 'as'"),
        }
    }

    #[test]
    fn test_expr_type_literal() {
        use ast::{BaseType, SimpleType, BinOpKind};
        
        // x is int
        let result = ExprParser::new().parse("x is int").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Is);
                assert!(matches!(*binop.e1, Expr::Variable(_)));
                assert!(matches!(*binop.e2, Expr::TypeLiteral(SimpleType::Base(BaseType::Int))));
            }
            _ => panic!("Expected Binop with 'is'"),
        }
        
        // y as str
        let result = ExprParser::new().parse("y as str").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::As);
                assert!(matches!(*binop.e1, Expr::Variable(_)));
                assert!(matches!(*binop.e2, Expr::TypeLiteral(SimpleType::Base(BaseType::String))));
            }
            _ => panic!("Expected Binop with 'as'"),
        }
        
        // x is *
        let result = ExprParser::new().parse("x is *").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Is);
                assert!(matches!(*binop.e2, Expr::TypeLiteral(SimpleType::Star)));
            }
            _ => panic!("Expected Binop with 'is'"),
        }
    }

    #[test]
    fn test_expr_arithmetic() {
        // Test addition
        let result = ExprParser::new().parse("1 + 2").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Add);
                assert!(matches!(*binop.e1, Expr::Constant(Constant::Int(1))));
                assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(2))));
            }
            _ => panic!("Expected Binop"),
        }

        // Test multiplication with precedence
        let result = ExprParser::new().parse("1 + 2 * 3").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Add);
                assert!(matches!(*binop.e1, Expr::Constant(Constant::Int(1))));
                // Right side should be multiplication
                match *binop.e2 {
                    Expr::Binop(ref inner) => {
                        assert_eq!(inner.op, BinOpKind::Mul);
                    }
                    _ => panic!("Expected nested Binop"),
                }
            }
            _ => panic!("Expected Binop"),
        }
    }

    #[test]
    fn test_expr_parentheses() {
        // (1 + 2) * 3
        let result = ExprParser::new().parse("(1 + 2) * 3").unwrap();
        match result {
            Expr::Binop(binop) => {
                assert_eq!(binop.op, BinOpKind::Mul);
                // Left side should be addition due to parentheses
                match *binop.e1 {
                    Expr::Binop(ref inner) => {
                        assert_eq!(inner.op, BinOpKind::Add);
                    }
                    _ => panic!("Expected nested Binop"),
                }
                assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(3))));
            }
            _ => panic!("Expected Binop"),
        }
    }

    // ==========================================
    // TESTS FOR UNIMPLEMENTED FEATURES (commented out)
    // ==========================================

    // Edge patterns not yet implemented
    // #[test]
    // fn test_edge_right_empty() {
    //     // -> or -[]->
    //     let result = EdgePatternRightParser::new().parse("->").unwrap();
    // }

    // #[test]
    // fn test_edge_left_empty() {
    //     // <- or <-[]-
    //     let result = EdgePatternLeftParser::new().parse("<-").unwrap();
    // }

    // #[test]
    // fn test_edge_non_directional() {
    //     // ~ or ~[]~
    //     let result = EdgePatternUndirectedParser::new().parse("~").unwrap();
    // }

    // Concatenation patterns not yet implemented
    // #[test]
    // fn test_concatenation() {
    //     // (x)~[y]~(z)
    //     let result = PatternParser::new().parse("(x)~[y]~(z)").unwrap();
    // }

    // Filter patterns not yet implemented
    // #[test]
    // fn test_filter_pattern() {
    //     // (x WHERE x.a>10)
    //     let result = PatternParser::new().parse("(x WHERE x.a>10)").unwrap();
    // }

    // #[test]
    // fn test_filter_on_edge() {
    //     // (x)-[y WHERE y.a>10]->(z)
    //     let result = PatternParser::new().parse("(x)-[y WHERE y.a>10]->(z)").unwrap();
    // }

    // Repetition patterns not yet implemented
    // #[test]
    // fn test_repetition() {
    //     // (x)* or (x)+ or (x){1,2}
    //     let result = PatternParser::new().parse("(x)*").unwrap();
    // }

    // Union patterns not yet implemented
    // #[test]
    // fn test_union() {
    //     // () | ()
    //     let result = PatternParser::new().parse("() | ()").unwrap();
    // }

    // Questioned patterns not yet implemented
    // #[test]
    // fn test_questioned_edge() {
    //     // -[z]->?
    //     let result = PatternParser::new().parse("-[z]->?").unwrap();
    // }
}
