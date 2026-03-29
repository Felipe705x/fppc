use std::collections::HashMap;

use crate::ast::{Descriptor, Var};

use super::schema::Schema;
use super::variable_type::VariableType;

/// A type environment mapping variable names to their inferred `VariableType`.
///
/// Used during type checking to track the types of all variables in scope.
#[derive(PartialEq, Clone, Debug)]
pub struct TypeEnvironment {
    bindings: HashMap<String, VariableType>,
}

impl TypeEnvironment {
    /// Creates a new, empty type environment.
    pub fn new() -> Self {
        TypeEnvironment {
            bindings: HashMap::new(),
        }
    }

    /// Creates a type environment from a descriptor and its inferred type.
    /// If the descriptor has a variable, binds it to the given type.
    /// If no variable is present, returns an empty environment.
    pub fn create_context(descriptor: &Descriptor, t: VariableType) -> Self {
        let mut env = TypeEnvironment::new();
        if let Some(var) = &descriptor.variable {
            env.set(&var.0, t);
        }
        env
    }

    /// Inserts or updates a variable binding.
    pub fn set(&mut self, key: &str, value: VariableType) {
        self.bindings.insert(key.to_string(), value);
    }

    /// Inserts or updates a variable binding using a `Var`.
    pub fn set_var(&mut self, var: &Var, value: VariableType) {
        self.bindings.insert(var.0.clone(), value);
    }

    /// Returns the type of a variable, or `None` if not bound.
    pub fn get(&self, key: &str) -> Option<&VariableType> {
        self.bindings.get(key)
    }

    /// Returns the type of a variable using a `Var`, or `None` if not bound.
    pub fn get_var(&self, var: &Var) -> Option<&VariableType> {
        self.bindings.get(&var.0)
    }

    /// Returns all variable names in this environment.
    pub fn keys(&self) -> Vec<&String> {
        self.bindings.keys().collect()
    }

    /// Pointwise join (least upper bound) of two environments.
    ///
    /// For each variable in either environment:
    /// - If present in both, join their types.
    /// - If present in only one, keep as-is (join with Zero = identity).
    pub fn union(a: &TypeEnvironment, b: &TypeEnvironment) -> TypeEnvironment {
        let mut result = a.bindings.clone();

        for (key, other_type) in &b.bindings {
            let merged = match result.get(key) {
                Some(self_type) => VariableType::join(self_type.clone(), other_type.clone()),
                None => other_type.clone(),
            };
            result.insert(key.clone(), merged);
        }

        TypeEnvironment { bindings: result }
    }

    /// Pointwise meet (greatest lower bound) of two environments with schema refinement.
    ///
    /// For each variable in either environment:
    /// - If present in both, meet their types and refine against the schema.
    /// - If present in only one, keep as-is (unconstrained in the other).
    ///
    /// Returns `Err` if any pointwise meet fails.
    pub fn meet(
        schema: &Schema,
        a: &TypeEnvironment,
        b: &TypeEnvironment,
    ) -> Result<TypeEnvironment, String> {
        let mut result = a.bindings.clone();

        for (key, other_type) in &b.bindings {
            let merged = match result.get(key) {
                Some(self_type) => {
                    let met = VariableType::meet(self_type, other_type)?;
                    VariableType::refine(schema, &met)
                }
                None => other_type.clone(),
            };
            result.insert(key.clone(), merged);
        }

        Ok(TypeEnvironment { bindings: result })
    }

    /// Returns true if any variable has an empty (unsatisfiable) type.
    pub fn is_empty(&self) -> bool {
        self.bindings.values().any(VariableType::is_empty)
    }

    /// Creates a new environment where each variable type is wrapped in `List`.
    /// Used for repeated/quantified patterns where variables become lists of values.
    pub fn to_list(&self) -> TypeEnvironment {
        TypeEnvironment {
            bindings: self
                .bindings
                .iter()
                .map(|(k, v)| (k.clone(), VariableType::list(v.clone())))
                .collect(),
        }
    }
}

impl Default for TypeEnvironment {
    fn default() -> Self {
        TypeEnvironment::new()
    }
}
