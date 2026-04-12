use crate::ast::*;
use crate::vm::bet::pack_trits;
use crate::trit::Trit;

pub struct BytecodeEmitter {
    code: Vec<u8>,
    symbols: std::collections::HashMap<String, u8>,
    func_addrs: std::collections::HashMap<String, u16>,
    function_patches: std::collections::HashMap<String, Vec<usize>>,
    break_patches: Vec<usize>,
    continue_patches: Vec<usize>,
    next_reg: usize,
    struct_layouts: std::collections::HashMap<String, Vec<String>>,
    agent_type_ids: std::collections::HashMap<String, u16>,
    agent_handlers: Vec<(u16, u16)>,
    /// Snapshots of the local symbol table for each function, keyed by function name.
    /// Captured just before scope is restored so callers can map reg→varname after execution.
    function_symbols: std::collections::HashMap<String, std::collections::HashMap<String, u8>>,
}

impl BytecodeEmitter {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            symbols: std::collections::HashMap::new(),
            func_addrs: std::collections::HashMap::new(),
            function_patches: std::collections::HashMap::new(),
            break_patches: Vec::new(),
            continue_patches: Vec::new(),
            next_reg: 0,
            struct_layouts: std::collections::HashMap::new(),
            agent_type_ids: std::collections::HashMap::new(),
            agent_handlers: Vec::new(),
            function_symbols: std::collections::HashMap::new(),
        }
    }

    /// Returns the top-level variable-name → register-number map.
    pub fn get_symbols(&self) -> &std::collections::HashMap<String, u8> {
        &self.symbols
    }

    /// Returns the local symbol snapshot for a specific function (e.g. "main").
    /// Used by `ternlang-cli --emit-symbols` to correlate VM register dumps with source variable names.
    pub fn get_function_symbols(&self, name: &str) -> Option<&std::collections::HashMap<String, u8>> {
        self.function_symbols.get(name)
    }

    pub fn register_agents(&self, vm: &mut crate::vm::BetVm) {
        for &(type_id, addr) in &self.agent_handlers {
            vm.register_agent_type(type_id, addr as usize);
        }
    }

    pub fn emit_header_jump(&mut self) -> usize {
        let patch_pos = self.code.len() + 1;
        self.code.push(0x0b); // TJMP
        self.code.extend_from_slice(&[0u8, 0u8]);
        patch_pos
    }

    pub fn patch_header_jump(&mut self, patch_pos: usize) {
        let addr = self.code.len() as u16;
        self.patch_u16(patch_pos, addr);
    }

    pub fn emit_program(&mut self, program: &Program) {
        let parent_next_reg = self.next_reg;
        for s in &program.structs {
            let names: Vec<String> = s.fields.iter().map(|(n, _)| n.clone()).collect();
            self.struct_layouts.insert(s.name.clone(), names);
        }
        for (idx, agent) in program.agents.iter().enumerate() {
            self.agent_type_ids.insert(agent.name.clone(), idx as u16);
        }

        // PASS 1: Addresses
        let real_code = std::mem::take(&mut self.code);
        let real_func_addrs = std::mem::take(&mut self.func_addrs);
        let real_agent_handlers = std::mem::take(&mut self.agent_handlers);
        let base_addr = real_code.len() as u16;

        for agent in &program.agents {
            let type_id = self.agent_type_ids[&agent.name];
            let mut handler_addr = None;
            for method in &agent.methods {
                let addr = base_addr + self.code.len() as u16;
                if handler_addr.is_none() { handler_addr = Some(addr); }
                self.emit_function(method);
                self.func_addrs.insert(format!("{}::{}", agent.name, method.name), addr);
            }
            if let Some(addr) = handler_addr { self.agent_handlers.push((type_id, addr)); }
        }
        for func in &program.functions {
            let addr = base_addr + self.code.len() as u16;
            self.func_addrs.insert(func.name.clone(), addr);
            // Ensure any global symbols or previous definitions are visible
            self.emit_function(func);
        }

        let final_func_addrs = std::mem::replace(&mut self.func_addrs, real_func_addrs);
        let final_agent_handlers = std::mem::replace(&mut self.agent_handlers, real_agent_handlers);
        self.code = real_code;
        self.func_addrs = final_func_addrs;
        self.agent_handlers = final_agent_handlers;
        self.next_reg = parent_next_reg;

        // PASS 2: Real
        for agent in &program.agents {
            for method in &agent.methods { self.emit_function(method); }
        }
        for func in &program.functions { self.emit_function(func); }
    }

    pub fn emit_function(&mut self, func: &Function) {
        let func_addr = self.code.len() as u16;
        self.func_addrs.insert(func.name.clone(), func_addr);
        if let Some(patches) = self.function_patches.remove(&func.name) {
            for p in patches {
                self.code[p..p + 2].copy_from_slice(&func_addr.to_le_bytes());
            }
        }
        let parent_symbols = self.symbols.clone();
        let parent_next_reg = self.next_reg;
        self.next_reg = 0;

        // If function has @sparseskip, we could emit a special header here.
        // For now, it's just a marker in the AST.

        for (name, ty) in func.params.iter().rev() {
            if let Type::Named(s_name) = ty {
                if let Some(fields) = self.struct_layouts.get(s_name).cloned() {
                    // Structs are passed as a bundle: [field1, field2, ..., root_dummy]
                    // We must pop root dummy first, then fields.
                    
                    // Pop root dummy
                    let root_reg = self.alloc_reg();
                    self.symbols.insert(name.clone(), root_reg);
                    self.code.push(0x08); self.code.push(root_reg);

                    // Pop fields in reverse order of how they were pushed
                    for f_name in fields.iter().rev() {
                        let f_reg = self.alloc_reg();
                        let key = format!("{}.{}", name, f_name);
                        self.symbols.insert(key, f_reg);
                        self.code.push(0x08); self.code.push(f_reg);
                    }
                    continue;
                }
            }
            let reg = self.alloc_reg();
            self.symbols.insert(name.clone(), reg);
            self.code.push(0x08); self.code.push(reg);
        }
        for stmt in &func.body { self.emit_stmt(stmt); }
        // Snapshot local symbols before scope is restored — used by --emit-symbols
        self.function_symbols.insert(func.name.clone(), self.symbols.clone());
        self.symbols = parent_symbols;
        self.next_reg = parent_next_reg;
        self.code.push(0x11); // TRET
    }

    pub fn emit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty, value } => {
                let mut handled = false;
                if let Type::TritTensor { dims } = ty {
                    // Only auto-allocate if size is fixed (>0) and no literal is provided
                    if !dims.is_empty() && !dims.contains(&0) && !matches!(value, Expr::TritTensorLiteral(_)) {
                        let rows = dims[0];
                        let cols = if dims.len() > 1 { dims[1] } else { 1 };
                        self.code.push(0x0f);
                        self.code.extend_from_slice(&(rows as u16).to_le_bytes());
                        self.code.extend_from_slice(&(cols as u16).to_le_bytes());
                        handled = true;
                    }
                } else if let Type::Named(_) = ty {
                    if let Expr::StructLiteral { fields, .. } = value {
                        // Flatten struct fields into mangled registers
                        for (f_name, f_val) in fields {
                            self.emit_expr(f_val);
                            let reg = self.alloc_reg();
                            let key = format!("{}.{}", name, f_name);
                            self.symbols.insert(key, reg);
                            self.code.push(0x08); self.code.push(reg);
                        }
                        // Now we let the normal path emit the root variable's dummy value
                    }
                }
                if !handled {
                    self.emit_expr(value);
                }
                let reg = self.alloc_reg();
                self.symbols.insert(name.clone(), reg);
                self.code.push(0x08); self.code.push(reg); // TSTORE
            }
            Stmt::Set { name, value } => {
                self.emit_expr(value);
                if let Some(&reg) = self.symbols.get(name) {
                    self.code.push(0x08); self.code.push(reg);
                }
            }
            Stmt::FieldSet { object, field, value } => {
                let key = format!("{}.{}", object, field);
                self.emit_expr(value);
                if let Some(&reg) = self.symbols.get(&key) {
                    self.code.push(0x08); self.code.push(reg);
                }
            }
            Stmt::IndexSet { object, row, col, value } => {
                if let Some(&reg) = self.symbols.get(object) {
                    self.code.push(0x09); self.code.push(reg);
                    self.emit_expr(row);
                    self.emit_expr(col);
                    self.emit_expr(value);
                    self.code.push(0x23);
                }
            }
            Stmt::IfTernary { condition, on_pos, on_zero, on_neg } => {
                self.emit_expr(condition);
                self.code.push(0x0a);
                let pos_patch = self.code.len() + 1;
                self.code.push(0x05); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0a);
                let zero_patch = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0c);
                self.emit_stmt(on_neg);
                let exit_patch = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                let pos_addr = self.code.len() as u16;
                self.patch_u16(pos_patch, pos_addr);
                self.code.push(0x0c);
                self.emit_stmt(on_pos);
                let exit_pos = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                let zero_addr = self.code.len() as u16;
                self.patch_u16(zero_patch, zero_addr);
                self.code.push(0x0c);
                self.emit_stmt(on_zero);
                let end = self.code.len() as u16;
                self.patch_u16(exit_patch, end);
                self.patch_u16(exit_pos, end);
            }
            Stmt::Match { condition, arms } => {
                self.emit_expr(condition);
                let cond_reg = self.alloc_reg();
                self.code.push(0x08); self.code.push(cond_reg); // Tstore

                let mut end_patches = Vec::new();
                let mut next_arm_patch = None;

                for (val, stmt) in arms {
                    if let Some(p) = next_arm_patch {
                        let addr = self.code.len() as u16;
                        self.patch_u16(p, addr);
                    }

                    // Load condition for this arm
                    self.code.push(0x09); self.code.push(cond_reg); // Tload

                    let match_patch;
                    match val {
                        1 => {
                            self.code.push(0x05); // TjmpPos (peeks)
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        0 => {
                            self.code.push(0x06); // TjmpZero (peeks)
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        -1 => {
                            self.code.push(0x07); // TjmpNeg (peeks)
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        v => {
                            self.code.push(0x25); // TjmpEqInt (peeks)
                            self.code.extend_from_slice(&v.to_le_bytes());
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                    }

                    // Mismatch: Jump past body to the next arm's check
                    let skip_patch = self.code.len() + 1;
                    self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                    next_arm_patch = Some(skip_patch);

                    // Match found: execute body
                    let body_addr = self.code.len() as u16;
                    self.patch_u16(match_patch, body_addr);
                    
                    // Body: first pop the condition we were peeking at
                    self.code.push(0x0c); // Tpop
                    self.emit_stmt(stmt);
                    
                    // After body, jump to end of match
                    let end_patch = self.code.len() + 1;
                    self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                    end_patches.push(end_patch);
                }

                if let Some(p) = next_arm_patch {
                    let addr = self.code.len() as u16;
                    self.patch_u16(p, addr);
                }
                
                // If no arms matched, we still have one Tload on stack from the last failed arm check
                // unless arms was empty (but semantic enforces it isn't for Trit, and for Int it might be)
                if !arms.is_empty() {
                    self.code.push(0x0c); // Tpop
                }

                let end_addr = self.code.len() as u16;
                for p in end_patches { self.patch_u16(p, end_addr); }
                self.next_reg -= 1;
            }
            Stmt::ForIn { var, iter, body } => {
                // Save next_reg so loop-internal registers are freed after the loop ends.
                // Without this, each for-in permanently consumes 4 registers, exhausting
                // the register file after 6-7 loops in a single function.
                let pre_loop_reg = self.next_reg;

                self.emit_expr(iter);
                let it_reg = self.alloc_reg();
                self.code.push(0x08); self.code.push(it_reg);
                self.code.push(0x09); self.code.push(it_reg);
                self.code.push(0x24); // TSHAPE: pushes rows then cols (cols on top)
                self.code.push(0x0c); // pop cols — iterate over rows, not cols
                let r_reg = self.alloc_reg();
                self.code.push(0x08); self.code.push(r_reg); // store rows as loop bound
                let i_reg = self.alloc_reg();
                self.code.push(0x17); self.code.extend_from_slice(&0i64.to_le_bytes());
                self.code.push(0x08); self.code.push(i_reg);

                let top = self.code.len() as u16;
                let pre_break = self.break_patches.len();
                let pre_cont = self.continue_patches.len();

                self.code.push(0x09); self.code.push(i_reg);
                self.code.push(0x09); self.code.push(r_reg);
                self.code.push(0x14);
                self.code.push(0x0a);
                let neg = self.code.len() + 1;
                self.code.push(0x07); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0a);
                let zero = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0c);
                self.code.push(0x09); self.code.push(it_reg);
                self.code.push(0x09); self.code.push(i_reg);
                self.code.push(0x17); self.code.extend_from_slice(&0i64.to_le_bytes());
                self.code.push(0x22);
                let v_reg = self.alloc_reg();
                self.symbols.insert(var.clone(), v_reg);
                self.code.push(0x08); self.code.push(v_reg);
                self.emit_stmt(body);

                let cont_addr = self.code.len() as u16;
                let cs: Vec<usize> = self.continue_patches.drain(pre_cont..).collect();
                for p in cs { self.patch_u16(p, cont_addr); }

                self.code.push(0x09); self.code.push(i_reg);
                self.code.push(0x17); self.code.extend_from_slice(&1i64.to_le_bytes());
                self.code.push(0x18);
                self.code.push(0x08); self.code.push(i_reg);
                let back = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(back, top);
                let end = self.code.len() as u16;
                self.patch_u16(neg, end); self.patch_u16(zero, end);
                let bs: Vec<usize> = self.break_patches.drain(pre_break..).collect();
                for p in bs { self.patch_u16(p, end); }

                // Free loop registers: loop variable is out of scope, and the 4
                // internal registers (it, r, i, v) are no longer needed.
                self.symbols.remove(var);
                self.next_reg = pre_loop_reg;
            }
            Stmt::WhileTernary { condition, on_pos, on_zero, on_neg } => {
                let top = self.code.len() as u16;
                let pre_break = self.break_patches.len();
                let pre_cont = self.continue_patches.len();

                self.emit_expr(condition);
                self.code.push(0x0a); // TDUP
                let pos_patch = self.code.len() + 1;
                self.code.push(0x05); self.code.extend_from_slice(&[0, 0]); // TJMP_POS
                self.code.push(0x0a); // TDUP
                let zero_patch = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]); // TJMP_ZERO
                
                // NEG ARM: execute and EXIT (don't loop back)
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_neg);
                let exit_neg = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]); // TJMP to end

                // POS ARM: execute and LOOP BACK
                let pos_addr = self.code.len() as u16;
                self.patch_u16(pos_patch, pos_addr);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_pos);
                let back_pos = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(back_pos, top);

                // ZERO ARM: execute and EXIT (don't loop back)
                let zero_addr = self.code.len() as u16;
                self.patch_u16(zero_patch, zero_addr);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_zero);
                
                let end = self.code.len() as u16;
                self.patch_u16(exit_neg, end);

                let cs: Vec<usize> = self.continue_patches.drain(pre_cont..).collect();
                for p in cs { self.patch_u16(p, top); }
                let bs: Vec<usize> = self.break_patches.drain(pre_break..).collect();
                for p in bs { self.patch_u16(p, end); }
            }
            Stmt::Loop { body } => {
                let top = self.code.len() as u16;
                let pre_break = self.break_patches.len();
                let pre_cont = self.continue_patches.len();
                self.emit_stmt(body);
                let back = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(back, top);
                let end = self.code.len() as u16;
                let cs: Vec<usize> = self.continue_patches.drain(pre_cont..).collect();
                for p in cs { self.patch_u16(p, top); }
                let bs: Vec<usize> = self.break_patches.drain(pre_break..).collect();
                for p in bs { self.patch_u16(p, end); }
            }
            Stmt::Break => {
                let p = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.break_patches.push(p);
            }
            Stmt::Continue => {
                let p = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.continue_patches.push(p);
            }
            Stmt::Send { target, message } => {
                self.emit_expr(target);
                self.emit_expr(message);
                self.code.push(0x31); // TSEND
            }
            Stmt::Return(e) => { self.emit_expr(e); self.code.push(0x11); }
            Stmt::Block(ss) => { for s in ss { self.emit_stmt(s); } }
            Stmt::Expr(e) => { self.emit_expr(e); self.code.push(0x0c); }
            Stmt::Decorated { directive: _, stmt } => { self.emit_stmt(stmt); }
            _ => {}
        }
    }

    fn emit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::TritLiteral(v) => {
                self.code.push(0x01);
                self.code.extend(pack_trits(&[Trit::from(*v)]));
            }
            Expr::IntLiteral(v) => {
                self.code.push(0x17);
                self.code.extend_from_slice(&v.to_le_bytes());
            }
            Expr::FloatLiteral(val) => {
                self.code.push(0x19);
                self.code.extend_from_slice(&val.to_le_bytes());
            }
            Expr::StringLiteral(val) => {
                self.code.push(0x21); // TPUSH_STRING
                let bytes = val.as_bytes();
                self.code.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                self.code.extend_from_slice(bytes);
            }
            Expr::Ident(name) => {
                if let Some(&r) = self.symbols.get(name) {
                    self.code.push(0x09); self.code.push(r);
                }
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                self.emit_expr(lhs); self.emit_expr(rhs);
                match op {
                    BinOp::Add => self.code.push(0x02),
                    BinOp::Mul => self.code.push(0x03),
                    BinOp::Div => self.code.push(0x1e),
                    BinOp::Mod => self.code.push(0x1f),
                    BinOp::Sub => { self.code.push(0x04); self.code.push(0x02); }
                    BinOp::Equal => self.code.push(0x16),
                    BinOp::NotEqual => { self.code.push(0x16); self.code.push(0x04); }
                    BinOp::And => self.code.push(0x28), // TAND = min(a,b)
                    BinOp::Or  => self.code.push(0x29), // TOR  = max(a,b)
                    BinOp::Less => self.code.push(0x14),
                    BinOp::Greater => self.code.push(0x15),
                    BinOp::LessEqual => self.code.push(0x26),
                    BinOp::GreaterEqual => self.code.push(0x27),
                }
            }
            Expr::UnaryOp { op, expr } => {
                self.emit_expr(expr);
                match op { UnOp::Neg => self.code.push(0x04) }
            }
            Expr::Call { callee, args } => {
                match callee.as_str() {
                    "print" | "println" => {
                        for a in args {
                            self.emit_expr(a);
                            self.code.push(0x20); // TPRINT
                        }
                        self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend])); // return hold()
                    }
                    "consensus" => {
                        for a in args { self.emit_expr(a); }
                        if args.len() == 2 { self.code.push(0x0e); }
                    }
                    "length" => {
                        if args.len() == 1 {
                            self.emit_expr(&args[0]);
                            self.code.push(0x24); // TSHAPE
                            self.code.push(0x0c); // TPOP (cols)
                        }
                    }
                    "mul" => {
                        for a in args { self.emit_expr(a); }
                        if args.len() == 2 { self.code.push(0x03); }
                    }
                    "truth" => { self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Affirm])); }
                    "hold" => { self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend])); }
                    "conflict" => { self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Reject])); }
                    _ => {
                        for a in args {
                            // If argument is a struct, we need to push all its flattened fields + root dummy
                            let mut pushed_as_struct = false;
                            if let Expr::Ident(name) = a {
                                // We don't have the variable type here, but we can try to find if it's a struct
                                // by looking for any mangled keys starting with "name.".
                                // To get the correct field order, we'd need the struct name.
                                // Let's try to find which struct layout matches the existing mangled keys.
                                let mut fields_found = Vec::new();
                                for (_s_name, s_fields) in &self.struct_layouts {
                                    let mut all_present = true;
                                    let mut current_regs = Vec::new();
                                    for f in s_fields {
                                        let key = format!("{}.{}", name, f);
                                        if let Some(&r) = self.symbols.get(&key) {
                                            current_regs.push(r);
                                        } else {
                                            all_present = false;
                                            break;
                                        }
                                    }
                                    if all_present && !s_fields.is_empty() {
                                        fields_found = current_regs;
                                        break;
                                    }
                                }

                                if !fields_found.is_empty() {
                                    for reg in fields_found {
                                        self.code.push(0x09); self.code.push(reg); // TLOAD field
                                    }
                                    // Push root dummy
                                    if let Some(&reg) = self.symbols.get(name) {
                                        self.code.push(0x09); self.code.push(reg); // TLOAD root
                                    }
                                    pushed_as_struct = true;
                                }
                            }
                            
                            if !pushed_as_struct {
                                self.emit_expr(a);
                            }
                        }
                        self.code.push(0x10); // TCALL
                        if let Some(&addr) = self.func_addrs.get(callee) {
                            self.code.extend_from_slice(&addr.to_le_bytes());
                        } else {
                            let patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                            self.function_patches.entry(callee.to_string()).or_default().push(patch);
                        }
                    }
                }
            }
            Expr::Spawn { agent_name, .. } => {
                if let Some(&type_id) = self.agent_type_ids.get(agent_name) {
                    self.code.push(0x30); // TSPAWN
                    self.code.extend_from_slice(&type_id.to_le_bytes());
                } else {
                    self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend]));
                }
            }
            Expr::Await { target } => {
                self.emit_expr(target);
                self.code.push(0x32); // TAWAIT
            }
            Expr::TritTensorLiteral(vs) => {
                let rows = vs.len();
                let cols = 1;
                self.code.push(0x0f);
                self.code.extend_from_slice(&(rows as u16).to_le_bytes());
                self.code.extend_from_slice(&(cols as u16).to_le_bytes());
                let tr = self.next_reg; self.next_reg += 1;
                self.code.push(0x08); self.code.push(tr.try_into().unwrap());
                for (idx, &v) in vs.iter().enumerate() {
                    self.code.push(0x09); self.code.push(tr.try_into().unwrap());
                    self.code.push(0x17); self.code.extend_from_slice(&(idx as i64).to_le_bytes());
                    self.code.push(0x17); self.code.extend_from_slice(&0i64.to_le_bytes());
                    self.code.push(0x01); self.code.extend(pack_trits(&[Trit::from(v)]));
                    self.code.push(0x23);
                }
                self.code.push(0x09); self.code.push(tr.try_into().unwrap());
            }
            Expr::StructLiteral { .. } => {
                // Struct literals are largely handled by Stmt::Let by flattening.
                // We push a dummy hold so Return(struct_lit) or Let works consistently.
                self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend]));
            }
            Expr::Propagate { expr } => {
                self.emit_expr(expr);
                self.code.push(0x0a); // TDUP
                let patch = self.code.len() + 1;
                self.code.push(0x07); self.code.extend_from_slice(&[0, 0]); // TJMP_NEG
                let skip = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]); // TJMP
                let early_ret = self.code.len() as u16;
                self.patch_u16(patch, early_ret);
                self.code.push(0x11); // TRET
                let next = self.code.len() as u16;
                self.patch_u16(skip, next);
            }
            Expr::Index { object, row, col } => {
                self.emit_expr(object); self.emit_expr(row); self.emit_expr(col);
                self.code.push(0x22);
            }
            Expr::FieldAccess { object, field } => {
                if let Expr::Ident(obj_name) = object.as_ref() {
                    let key = format!("{}.{}", obj_name, field);
                    if let Some(&r) = self.symbols.get(&key) {
                        self.code.push(0x09); self.code.push(r); // TLOAD
                    }
                }
            }
            Expr::Cast { expr, .. } => {
                // cast() is a type annotation hint only — pass inner expression through
                self.emit_expr(expr);
            }
            Expr::NodeId => {
                let bytes = b"127.0.0.1:7373";
                self.code.push(0x21);
                self.code.extend_from_slice(&(bytes.len() as u16).to_le_bytes());
                self.code.extend_from_slice(bytes);
            }
        }
    }

    pub fn emit_entry_call(&mut self, name: &str) {
        if let Some(&addr) = self.func_addrs.get(name) {
            self.code.push(0x10); self.code.extend_from_slice(&addr.to_le_bytes());
        }
    }

    /// Allocate the next register, returning its index as `u8` (the bytecode register width).
    /// Emits a stderr diagnostic if the function requires more than 255 registers — programs
    /// that hit this have much bigger structural problems anyway.
    fn alloc_reg(&mut self) -> u8 {
        let r = self.next_reg;
        self.next_reg += 1;
        if r > 255 {
            eprintln!(
                "[CODEGEN] Warning: register #{r} exceeds u8 range — \
                 this function has too many local variables (max 255). \
                 Split the function or reduce scope depth."
            );
        }
        r as u8
    }

    pub fn get_agent_handlers(&self) -> Vec<(u16, usize)> {
        self.agent_handlers.iter().map(|&(id, addr)| (id, addr as usize)).collect()
    }

    pub fn finalize(&mut self) -> Vec<u8> { std::mem::take(&mut self.code) }

    fn patch_u16(&mut self, pos: usize, val: u16) {
        let b = val.to_le_bytes();
        self.code[pos] = b[0]; self.code[pos + 1] = b[1];
    }
}
