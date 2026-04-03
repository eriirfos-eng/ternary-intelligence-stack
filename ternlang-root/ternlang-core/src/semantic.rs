use crate::ast::*;

#[derive(Debug)]
pub enum SemanticError {
    TypeMismatch { expected: Type, found: Type },
    UndefinedVariable(String),
    UndefinedStruct(String),
    UndefinedField { struct_name: String, field: String },
    UndefinedFunction(String),
}

pub struct SemanticAnalyzer {
    scopes: Vec<std::collections::HashMap<String, Type>>,
    /// Struct definitions registered at program level
    struct_defs: std::collections::HashMap<String, Vec<(String, Type)>>,
    /// Function return types: name → return_type (user-defined + built-ins)
    func_signatures: std::collections::HashMap<String, Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut func_signatures = std::collections::HashMap::new();
        // Register built-in function return types
        func_signatures.insert("consensus".into(), Type::Trit);
        func_signatures.insert("invert".into(),    Type::Trit);
        func_signatures.insert("truth".into(),     Type::Trit);
        func_signatures.insert("hold".into(),      Type::Trit);
        func_signatures.insert("conflict".into(),  Type::Trit);
        func_signatures.insert("matmul".into(),    Type::TritTensor { dims: vec![1, 1] }); // dims are dynamic
        func_signatures.insert("sparsity".into(),  Type::Int);
        func_signatures.insert("shape".into(),     Type::Int);
        func_signatures.insert("cast".into(),      Type::Trit); // cast returns type of binding, fallback Trit

        Self {
            scopes: vec![std::collections::HashMap::new()],
            struct_defs: std::collections::HashMap::new(),
            func_signatures,
        }
    }

    /// Register all struct definitions before checking functions.
    pub fn register_structs(&mut self, structs: &[StructDef]) {
        for s in structs {
            self.struct_defs.insert(s.name.clone(), s.fields.clone());
        }
    }

    /// Register user-defined function signatures (name → return type).
    pub fn register_functions(&mut self, functions: &[Function]) {
        for f in functions {
            self.func_signatures.insert(f.name.clone(), f.return_type.clone());
        }
    }

    /// Register agent method signatures under "AgentName::method" and "method".
    pub fn register_agents(&mut self, agents: &[AgentDef]) {
        for agent in agents {
            for method in &agent.methods {
                self.func_signatures.insert(method.name.clone(), method.return_type.clone());
                let fq = format!("{}::{}", agent.name, method.name);
                self.func_signatures.insert(fq, method.return_type.clone());
            }
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), SemanticError> {
        self.register_structs(&program.structs);
        // Register all signatures first so forward calls resolve correctly.
        self.register_functions(&program.functions);
        self.register_agents(&program.agents);
        // Then type-check bodies.
        for agent in &program.agents {
            for method in &agent.methods {
                self.check_function(method)?;
            }
        }
        for func in &program.functions {
            self.check_function(func)?;
        }
        Ok(())
    }

    fn check_function(&mut self, func: &Function) -> Result<(), SemanticError> {
        self.scopes.push(std::collections::HashMap::new());
        for (name, ty) in &func.params {
            self.scopes.last_mut().unwrap().insert(name.clone(), ty.clone());
        }
        for stmt in &func.body {
            self.check_stmt(stmt)?;
        }
        self.scopes.pop();
        Ok(())
    }

    pub fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        match stmt {
            Stmt::Let { name, ty, value } => {
                let val_ty = self.infer_expr_type(value)?;
                let type_ok = val_ty == *ty
                    || matches!(value, Expr::Cast { .. })
                    || (matches!(ty, Type::Named(_)) && val_ty == Type::Trit)
                    // TritTensor dims are dynamically inferred — accept any TritTensor for now
                    || (matches!(ty, Type::TritTensor { .. }) && matches!(val_ty, Type::TritTensor { .. }))
                    // AgentRef from spawn
                    || (*ty == Type::AgentRef && val_ty == Type::AgentRef);
                if !type_ok {
                    return Err(SemanticError::TypeMismatch { expected: ty.clone(), found: val_ty });
                }
                self.scopes.last_mut().unwrap().insert(name.clone(), ty.clone());
                Ok(())
            }
            Stmt::IfTernary { condition, on_pos, on_zero, on_neg } => {
                let cond_ty = self.infer_expr_type(condition)?;
                if cond_ty != Type::Trit {
                    return Err(SemanticError::TypeMismatch { expected: Type::Trit, found: cond_ty });
                }
                self.check_stmt(on_pos)?;
                self.check_stmt(on_zero)?;
                self.check_stmt(on_neg)?;
                Ok(())
            }
            Stmt::Match { condition, arms } => {
                let cond_ty = self.infer_expr_type(condition)?;
                if cond_ty != Type::Trit {
                    return Err(SemanticError::TypeMismatch { expected: Type::Trit, found: cond_ty });
                }
                for (_val, stmt) in arms {
                    self.check_stmt(stmt)?;
                }
                Ok(())
            }
            Stmt::Block(stmts) => {
                self.scopes.push(std::collections::HashMap::new());
                for s in stmts {
                    self.check_stmt(s)?;
                }
                self.scopes.pop();
                Ok(())
            }
            Stmt::Decorated { stmt, .. } => self.check_stmt(stmt),
            Stmt::Return(expr) => {
                self.infer_expr_type(expr)?;
                Ok(())
            }
            Stmt::Expr(expr) => {
                self.infer_expr_type(expr)?;
                Ok(())
            }
            Stmt::ForIn { var, iter, body } => {
                self.infer_expr_type(iter)?;
                self.scopes.push(std::collections::HashMap::new());
                self.scopes.last_mut().unwrap().insert(var.clone(), Type::Trit);
                self.check_stmt(body)?;
                self.scopes.pop();
                Ok(())
            }
            Stmt::WhileTernary { condition, on_pos, on_zero, on_neg } => {
                let cond_ty = self.infer_expr_type(condition)?;
                if cond_ty != Type::Trit {
                    return Err(SemanticError::TypeMismatch { expected: Type::Trit, found: cond_ty });
                }
                self.check_stmt(on_pos)?;
                self.check_stmt(on_zero)?;
                self.check_stmt(on_neg)?;
                Ok(())
            }
            Stmt::Loop { body } => self.check_stmt(body),
            Stmt::Break | Stmt::Continue => Ok(()),
            Stmt::Use { .. } => Ok(()),
            Stmt::Send { target, message } => {
                self.infer_expr_type(target)?;
                self.infer_expr_type(message)?;
                Ok(())
            }
            Stmt::FieldSet { object, field, value } => {
                let obj_ty = self.lookup_var(object)?;
                if let Type::Named(struct_name) = obj_ty {
                    let field_ty = self.lookup_field(&struct_name, field)?;
                    let val_ty = self.infer_expr_type(value)?;
                    if val_ty != field_ty {
                        return Err(SemanticError::TypeMismatch { expected: field_ty, found: val_ty });
                    }
                    Ok(())
                } else {
                    self.infer_expr_type(value)?;
                    Ok(())
                }
            }
        }
    }

    fn lookup_var(&self, name: &str) -> Result<Type, SemanticError> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Ok(ty.clone());
            }
        }
        Err(SemanticError::UndefinedVariable(name.to_string()))
    }

    fn lookup_field(&self, struct_name: &str, field: &str) -> Result<Type, SemanticError> {
        let fields = self.struct_defs.get(struct_name)
            .ok_or_else(|| SemanticError::UndefinedStruct(struct_name.to_string()))?;
        fields.iter()
            .find(|(f, _)| f == field)
            .map(|(_, ty)| ty.clone())
            .ok_or_else(|| SemanticError::UndefinedField {
                struct_name: struct_name.to_string(),
                field: field.to_string(),
            })
    }

    fn infer_expr_type(&self, expr: &Expr) -> Result<Type, SemanticError> {
        match expr {
            Expr::TritLiteral(_)    => Ok(Type::Trit),
            Expr::IntLiteral(_)     => Ok(Type::Int),
            Expr::StringLiteral(_)  => Ok(Type::String),
            Expr::Ident(name)       => self.lookup_var(name),
            Expr::BinaryOp { op: _, lhs, rhs } => {
                let l_ty = self.infer_expr_type(lhs)?;
                let r_ty = self.infer_expr_type(rhs)?;
                if l_ty != r_ty {
                    return Err(SemanticError::TypeMismatch { expected: l_ty, found: r_ty });
                }
                Ok(l_ty)
            }
            Expr::UnaryOp { expr, .. } => self.infer_expr_type(expr),
            Expr::Call { callee, .. } => {
                // Real function signature lookup — no more mock
                self.func_signatures.get(callee.as_str())
                    .cloned()
                    .ok_or_else(|| SemanticError::UndefinedFunction(callee.clone()))
            }
            Expr::Cast { ty, .. }      => Ok(ty.clone()),
            Expr::Spawn { .. }         => Ok(Type::AgentRef),
            Expr::Await { .. }         => Ok(Type::Trit),
            Expr::FieldAccess { object, field } => {
                let obj_ty = self.infer_expr_type(object)?;
                if let Type::Named(struct_name) = obj_ty {
                    self.lookup_field(&struct_name, field)
                } else {
                    Ok(Type::Trit)
                }
            }
        }
    }
}
