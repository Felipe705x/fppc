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
    // NODE PATTERN TESTS
    // ==========================================

    #[test]
    fn test_node_empty() {
        let result = PathPatternParser::new().parse("()").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
        assert_eq!(desc.variable, None);
        assert_eq!(desc.descriptor_type.label, LabelType::Star);
        assert!(matches!(desc.descriptor_type.properties, PropertyType::Open(_)));
    }

    #[test]
    fn test_node_variable() {
        let result = PathPatternParser::new().parse("(x)").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert_eq!(desc.descriptor_type.label, LabelType::Star);
    }

    #[test]
    fn test_descriptor() {
        let result = PathPatternParser::new().parse("(x:Person)").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => assert!(map.is_empty()),
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_descriptor_empty_record() {
        let result = PathPatternParser::new().parse("(x:Person {})").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => assert!(map.is_empty()),
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_descriptor_record() {
        let result = PathPatternParser::new().parse("(x :Person {a: int})").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
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
        let result = PathPatternParser::new().parse("(:Person {a: int, b: bool})").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
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
        let result = PathPatternParser::new().parse("(:{a: int, b: bool})").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
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
        let result = PathPatternParser::new().parse("(x :Person {{a: int}})").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
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
        let result = PathPatternParser::new().parse("(:Person & Company)").unwrap();
        let desc = match result {
            ast::PathPattern::Node(ref d) => d,
            _ => panic!("Expected PathPattern::Node"),
        };
        
        match &desc.descriptor_type.label {
            LabelType::And(l1, l2) => {
                assert_eq!(**l1, LabelType::Label("Person".to_string()));
                assert_eq!(**l2, LabelType::Label("Company".to_string()));
            }
            _ => panic!("Expected And label"),
        }
    }

    // ==========================================
    // FILTER PATTERN TESTS
    // ==========================================

    #[test]
    fn test_filter_attribute_gt() {
        let result = PathPatternParser::new().parse("(x where x.a>10)").unwrap();
        match result {
            ast::PathPattern::Filter(pattern, expr) => {
                // Check inner pattern is Node with variable x
                match *pattern {
                    ast::PathPattern::Node(ref desc) => {
                        assert_eq!(desc.variable, Some(Var("x".to_string())));
                    }
                    _ => panic!("Expected inner Node pattern"),
                }
                // Check filter expression
                match expr {
                    Expr::Binop(binop) => {
                        assert_eq!(binop.op, BinOpKind::Gt);
                        assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(10))));
                    }
                    _ => panic!("Expected Binop expression"),
                }
            }
            _ => panic!("Expected Filter pattern"),
        }
    }

    #[test]
    fn test_filter_and() {
        let result = PathPatternParser::new().parse("(x where 11>10 and (1 = 2 or 3>='1'))").unwrap();
        match result {
            ast::PathPattern::Filter(_, expr) => {
                match expr {
                    Expr::Binop(binop) => {
                        assert_eq!(binop.op, BinOpKind::And);
                    }
                    _ => panic!("Expected And at top level"),
                }
            }
            _ => panic!("Expected Filter pattern"),
        }
    }

    #[test]
    fn test_prioritization() {
        let result = PathPatternParser::new().parse("(x where 11 = 10 and 1 = 2 or 1=2)").unwrap();
        match result {
            ast::PathPattern::Filter(_, expr) => {
                match expr {
                    Expr::Binop(binop) => {
                        assert_eq!(binop.op, BinOpKind::Or);
                        // Left side should be And
                        match *binop.e1 {
                            Expr::Binop(ref inner) => {
                                assert_eq!(inner.op, BinOpKind::And);
                            }
                            _ => panic!("Expected And on left side"),
                        }
                    }
                    _ => panic!("Expected Or at top level"),
                }
            }
            _ => panic!("Expected Filter pattern"),
        }
    }

    #[test]
    fn test_simple_logical() {
        let result = PathPatternParser::new().parse("(x where true and 1>2)").unwrap();
        match result {
            ast::PathPattern::Filter(_, expr) => {
                match expr {
                    Expr::Binop(binop) => {
                        assert_eq!(binop.op, BinOpKind::And);
                        assert!(matches!(*binop.e1, Expr::Constant(Constant::Bool(true))));
                    }
                    _ => panic!("Expected And expression"),
                }
            }
            _ => panic!("Expected Filter pattern"),
        }
    }

    #[test]
    fn test_simple_arithmetic() {
        let result = PathPatternParser::new().parse("(x where x.a>x.b>1)").unwrap();
        match result {
            ast::PathPattern::Filter(_, expr) => {
                match expr {
                    Expr::Binop(binop) => {
                        assert_eq!(binop.op, BinOpKind::Gt);
                        // Left side should be another Gt comparison
                        match *binop.e1 {
                            Expr::Binop(ref inner) => {
                                assert_eq!(inner.op, BinOpKind::Gt);
                            }
                            _ => panic!("Expected nested Gt"),
                        }
                        assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(1))));
                    }
                    _ => panic!("Expected Gt expression"),
                }
            }
            _ => panic!("Expected Filter pattern"),
        }
    }

    #[test]
    fn test_unop_1() {
        let result = PathPatternParser::new().parse("(x WHERE not x.status)").unwrap();
        match result {
            ast::PathPattern::Filter(_, expr) => {
                match expr {
                    Expr::Unop(unop) => {
                        assert_eq!(unop.op, UnOpKind::Not);
                        assert!(matches!(*unop.e, Expr::AttributeLookup(_)));
                    }
                    _ => panic!("Expected Unop"),
                }
            }
            _ => panic!("Expected Filter pattern"),
        }
    }

    #[test]
    fn test_unop_2() {
        let result = PathPatternParser::new().parse("(x WHERE -x.status>0)").unwrap();
        match result {
            ast::PathPattern::Filter(_, expr) => {
                match expr {
                    Expr::Binop(binop) => {
                        assert_eq!(binop.op, BinOpKind::Gt);
                        match *binop.e1 {
                            Expr::Unop(ref unop) => {
                                assert_eq!(unop.op, UnOpKind::Neg);
                            }
                            _ => panic!("Expected Unop on left side"),
                        }
                        assert!(matches!(*binop.e2, Expr::Constant(Constant::Int(0))));
                    }
                    _ => panic!("Expected Binop"),
                }
            }
            _ => panic!("Expected Filter pattern"),
        }
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
