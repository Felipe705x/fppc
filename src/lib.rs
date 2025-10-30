use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub fppc);

pub use crate::fppc::{
    LabelTypeParser, SimpleTypeParser, PropertyTypeParser, 
    DescriptorTypeParser, DescriptorParser, ElementPatternFillerParser, NodePatternParser
};

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Var, LabelType, PropertyType, SimpleType, BaseType};

    // Tests matching Python implementation
    #[test]
    fn test_python_node_empty() {
        // () -> NodePattern(Descriptor(None, DescriptorType.star()))
        let result = NodePatternParser::new().parse("()").unwrap();
        let desc = &result.filler.descriptor;
        assert_eq!(desc.variable, None);
        assert!(matches!(desc.descriptor_type.label, LabelType::Star));
    }

    #[test]
    fn test_python_node_variable() {
        // (x) -> NodePattern(Descriptor(Var("x"), DescriptorType.star()))
        let result = NodePatternParser::new().parse("(x)").unwrap();
        let desc = &result.filler.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(desc.descriptor_type.label, LabelType::Star));
    }

    #[test]
    fn test_python_descriptor() {
        // (x:Person) -> NodePattern(Descriptor(Var("x"), DescriptorType(Label("Person"), OpenPropertyType())))
        let result = NodePatternParser::new().parse("(x:Person)").unwrap();
        let desc = &result.filler.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => assert!(map.is_empty()),
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_python_descriptor_empty_record() {
        // (x:Person {}) -> NodePattern(Descriptor(Var("x"), DescriptorType(Label("Person"), OpenPropertyType())))
        let result = NodePatternParser::new().parse("(x:Person {})").unwrap();
        let desc = &result.filler.descriptor;
        assert_eq!(desc.variable, Some(Var("x".to_string())));
        assert!(matches!(&desc.descriptor_type.label, LabelType::Label(s) if s == "Person"));
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => assert!(map.is_empty()),
            _ => panic!("Expected Open properties"),
        }
    }

    #[test]
    fn test_python_descriptor_record() {
        // (x :Person {a: int})
        let result = NodePatternParser::new().parse("(x :Person {a: int})").unwrap();
        let desc = &result.filler.descriptor;
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
    fn test_python_descriptor_record_multiple() {
        // (:Person {a: int, b: bool})
        let result = NodePatternParser::new().parse("(:Person {a: int, b: bool})").unwrap();
        let desc = &result.filler.descriptor;
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
    fn test_python_descriptor_no_label() {
        // (:{a: int, b: bool}) -> DescriptorType(StarLabel(), OpenPropertyType(...))
        let result = NodePatternParser::new().parse("(:{a: int, b: bool})").unwrap();
        let desc = &result.filler.descriptor;
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
    fn test_python_descriptor_record_closed() {
        // (x :Person {{a: int}})
        let result = NodePatternParser::new().parse("(x :Person {{a: int}})").unwrap();
        let desc = &result.filler.descriptor;
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
    fn test_python_label_and() {
        // (:Person & Company)
        let result = NodePatternParser::new().parse("(:Person & Company)").unwrap();
        let desc = &result.filler.descriptor;
        assert_eq!(desc.variable, None);
        match &desc.descriptor_type.label {
            LabelType::And(left, right) => {
                assert!(matches!(&**left, LabelType::Label(s) if s == "Person"));
                assert!(matches!(&**right, LabelType::Label(s) if s == "Company"));
            }
            _ => panic!("Expected And label"),
        }
        match &desc.descriptor_type.properties {
            PropertyType::Open(map) => assert!(map.is_empty()),
            _ => panic!("Expected Open properties"),
        }
    }
}
