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
    pub struct_layouts: std::collections::HashMap<String, Vec<String>>,
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

    /// Emit a single agent definition incrementally (used by the WASM fallback loop
    /// when the full `parse_program` path is unavailable).
    pub fn emit_agent_def(&mut self, agent: &crate::AgentDef) {
        let type_id = self.agent_type_ids.len() as u16;
        self.agent_type_ids.insert(agent.name.clone(), type_id);
        let mut handler_addr: Option<u16> = None;
        for method in &agent.methods {
            let addr = self.code.len() as u16;
            if handler_addr.is_none() {
                handler_addr = Some(addr);
            }
            self.emit_function(method);
            self.func_addrs.insert(format!("{}::{}", agent.name, method.name), addr);
        }
        if let Some(addr) = handler_addr {
            self.agent_handlers.push((type_id, addr));
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
                // Restore correct absolute address overwritten by emit_function (TCALL-BUG fix):
                self.func_addrs.insert(format!("{}::{}", agent.name, method.name), addr);
            }
            if let Some(addr) = handler_addr { self.agent_handlers.push((type_id, addr)); }
        }
        for func in &program.functions {
            let addr = base_addr + self.code.len() as u16;
            self.func_addrs.insert(func.name.clone(), addr);
            // Ensure any global symbols or previous definitions are visible
            self.emit_function(func);
            // emit_function overwrites func_addrs[name] with a temp-buffer offset that
            // omits base_addr. Restore the correct absolute address so that forward
            // references resolved later in PASS 1 get the right TCALL target.
            self.func_addrs.insert(func.name.clone(), addr);
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
                match ty {
                    Type::TritTensor { dims } => {
                        // Auto-alloc for any zero-initializer (TritLiteral(0) or IntLiteral(0))
                        let is_zero_init = matches!(value, Expr::TritLiteral(0) | Expr::IntLiteral(0));
                        if !dims.is_empty() && !dims.contains(&0) && is_zero_init {
                            let rows = dims[0];
                            let cols = if dims.len() > 1 { dims[1] } else { 1 };
                            self.code.push(0x0f); // TALLOC (trit)
                            self.code.extend_from_slice(&(rows as u16).to_le_bytes());
                            self.code.extend_from_slice(&(cols as u16).to_le_bytes());
                            handled = true;
                        }
                    }
                    Type::IntTensor { dims } => {
                        let is_zero_init = matches!(value, Expr::TritLiteral(0) | Expr::IntLiteral(0));
                        if !dims.is_empty() && !dims.contains(&0) && is_zero_init {
                            let rows = dims[0];
                            let cols = if dims.len() > 1 { dims[1] } else { 1 };
                            self.code.push(0x3c); // TALLOC_Int
                            self.code.extend_from_slice(&(rows as u16).to_le_bytes());
                            self.code.extend_from_slice(&(cols as u16).to_le_bytes());
                            handled = true;
                        }
                    }
                    Type::FloatTensor { dims } => {
                        let is_zero_init = matches!(value, Expr::TritLiteral(0) | Expr::IntLiteral(0));
                        if !dims.is_empty() && !dims.contains(&0) && is_zero_init {
                            let rows = dims[0];
                            let cols = if dims.len() > 1 { dims[1] } else { 1 };
                            self.code.push(0x3d); // TALLOC_Float
                            self.code.extend_from_slice(&(rows as u16).to_le_bytes());
                            self.code.extend_from_slice(&(cols as u16).to_le_bytes());
                            handled = true;
                        }
                    }
                    Type::Named(_) => {
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
                    _ => {}
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
                let pre_reg = self.next_reg;
                self.emit_expr(condition);
                let cond_reg = self.alloc_reg();
                self.code.push(0x08); self.code.push(cond_reg); // Tstore
                
                // Load condition for checks
                self.code.push(0x09); self.code.push(cond_reg); // Tload
                
                // Check POS
                let pos_patch = self.code.len() + 1;
                self.code.push(0x05); self.code.extend_from_slice(&[0, 0]); // TJMP_POS
                
                // Check ZERO
                let zero_patch = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]); // TJMP_ZERO
                
                // NEG arm: pop the condition and execute
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_neg);
                let exit_patch = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]); // TJMP to end
                
                // POS arm
                let pos_addr = self.code.len() as u16;
                self.patch_u16(pos_patch, pos_addr);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_pos);
                let exit_pos = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                
                // ZERO arm
                let zero_addr = self.code.len() as u16;
                self.patch_u16(zero_patch, zero_addr);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_zero);
                
                let end = self.code.len() as u16;
                self.patch_u16(exit_patch, end);
                self.patch_u16(exit_pos, end);
                self.next_reg = pre_reg;
            }
            Stmt::Match { condition, arms } => {
                let pre_reg = self.next_reg;
                self.emit_expr(condition);
                let cond_reg = self.alloc_reg();
                self.code.push(0x08); self.code.push(cond_reg); // Tstore

                let mut end_patches = Vec::new();
                let mut next_arm_patch = None;

                for (pattern, stmt) in arms {
                    if let Some(p) = next_arm_patch {
                        let addr = self.code.len() as u16;
                        self.patch_u16(p, addr);
                    }

                    // Load condition for this arm
                    self.code.push(0x09); self.code.push(cond_reg); // Tload

                    let match_patch;
                    match pattern {
                        Pattern::Trit(1) | Pattern::Int(1) => {
                            self.code.push(0x05); // TjmpPos (peeks)
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        Pattern::Trit(0) | Pattern::Int(0) => {
                            self.code.push(0x06); // TjmpZero (peeks)
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        Pattern::Trit(-1) | Pattern::Int(-1) => {
                            self.code.push(0x07); // TjmpNeg (peeks)
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        Pattern::Int(v) => {
                            self.code.push(0x25); // TjmpEqInt (peeks)
                            self.code.extend_from_slice(&v.to_le_bytes());
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        Pattern::Trit(v) => {
                            self.code.push(0x25); // TjmpEqInt (peeks)
                            self.code.extend_from_slice(&(*v as i64).to_le_bytes());
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        Pattern::Float(v) => {
                            self.code.push(0x2a); // TjmpEqFloat (peeks)
                            self.code.extend_from_slice(&v.to_le_bytes());
                            match_patch = self.code.len();
                            self.code.extend_from_slice(&[0, 0]);
                        }
                        Pattern::Wildcard => {
                            // Wildcard always matches — unconditional jump to body.
                            // Do NOT pop here: the body's shared TPOP below will clean
                            // the TLOAD value from the stack, keeping it balanced.
                            match_patch = self.code.len() + 1;
                            self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]); // TJMP placeholder
                        }
                    }

                    // Mismatch: the conditional test above PEEKS (doesn't pop), so if it
                    // didn't jump the TLOAD result is still on the stack. Pop it before
                    // jumping to the next arm to keep the stack balanced.
                    // (Wildcard never reaches here — it always jumps above.)
                    self.code.push(0x0c); // TPOP — discard unmatched arm's cond value
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
                
                if !arms.is_empty() {
                    // Each arm's mismatch path now does its own TPOP (see per-arm fix above),
                    // so the stack is already clean when we reach the fallback.
                    // VM-MATCH-001: non-exhaustive match — no arm was taken.
                    // Push a Tend (hold/undefined) placeholder so the stack is balanced
                    // even if the caller expects a return value from this match expression.
                    self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend]));
                }

                let end_addr = self.code.len() as u16;
                for p in end_patches { self.patch_u16(p, end_addr); }
                self.next_reg = pre_reg;
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

                // Use a register for the loop comparison so we avoid TDUP accumulation.
                // Previously: TDUP + TjmpNeg/TjmpZero (peek) left 2 values on the stack per
                // iteration, causing a stack leak that corrupted subsequent operations.
                let cmp_reg = self.alloc_reg();

                let top = self.code.len() as u16;
                let pre_break = self.break_patches.len();
                let pre_cont = self.continue_patches.len();

                // Compute i < r → cmp_reg (stack neutral: push then immediately store)
                self.code.push(0x09); self.code.push(i_reg);
                self.code.push(0x09); self.code.push(r_reg);
                self.code.push(0x14);                        // Tless → [cmp]
                self.code.push(0x08); self.code.push(cmp_reg); // TSTORE cmp → []

                // Load and test for NEG (i >= r → Reject → exit)
                self.code.push(0x09); self.code.push(cmp_reg); // [cmp]
                let neg = self.code.len() + 1;
                self.code.push(0x07); self.code.extend_from_slice(&[0, 0]); // TjmpNeg → peeks
                self.code.push(0x0c); // TPOP — clean up after failed neg check

                // Load and test for ZERO (i == r → Tend → exit)
                self.code.push(0x09); self.code.push(cmp_reg); // [cmp]
                let zero = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]); // TjmpZero → peeks
                self.code.push(0x0c); // TPOP — clean up after failed zero check, body runs clean

                // Body: load element tensor[it, i, 0] → v_reg
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

                // Exit paths for neg/zero: the TjmpNeg/TjmpZero PEEK so the cmp value
                // is still on the stack when they jump. Add a TPOP cleanup then TJMP end.
                let neg_exit_addr = self.code.len() as u16;
                self.patch_u16(neg, neg_exit_addr);
                self.code.push(0x0c); // TPOP — clean peeked cmp
                let neg_to_end = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);

                let zero_exit_addr = self.code.len() as u16;
                self.patch_u16(zero, zero_exit_addr);
                self.code.push(0x0c); // TPOP — clean peeked cmp
                let zero_to_end = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);

                let end = self.code.len() as u16;
                self.patch_u16(neg_to_end, end);
                self.patch_u16(zero_to_end, end);
                let bs: Vec<usize> = self.break_patches.drain(pre_break..).collect();
                for p in bs { self.patch_u16(p, end); }

                // Free loop registers: loop variable is out of scope, and the 4
                // internal registers (it, r, i, v) are no longer needed.
                self.symbols.remove(var);
                self.next_reg = pre_loop_reg;
            }
            Stmt::WhileTernary { condition, on_pos, on_zero, on_neg } => {
                let pre_reg = self.next_reg;
                let cond_reg = self.alloc_reg();
                let top = self.code.len() as u16;
                let pre_break = self.break_patches.len();
                let pre_cont = self.continue_patches.len();

                self.emit_expr(condition);
                self.code.push(0x08); self.code.push(cond_reg); // Tstore
                
                // Load condition for checks
                self.code.push(0x09); self.code.push(cond_reg); // Tload
                
                // Check POS
                let pos_patch = self.code.len() + 1;
                self.code.push(0x05); self.code.extend_from_slice(&[0, 0]); // TJMP_POS
                
                // Check ZERO
                let zero_patch = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]); // TJMP_ZERO
                
                // NEG ARM: pop and execute and EXIT (don't loop back)
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_neg);
                let exit_neg = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]); // TJMP to end

                // POS ARM: pop and execute and LOOP BACK
                let pos_addr = self.code.len() as u16;
                self.patch_u16(pos_patch, pos_addr);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_pos);
                let back_pos = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(back_pos, top);

                // ZERO ARM: pop and execute and EXIT (don't loop back)
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
                self.next_reg = pre_reg;
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
                // COMP-BOOL-001: `true`/`false` are not keywords in the lexer — they arrive
                // as Token::Ident. Handle them here so they produce a value instead of
                // causing a stack underflow when no symbol matches.
                match name.as_str() {
                    "true" => {
                        self.code.push(0x17); // TpushInt
                        self.code.extend_from_slice(&1i64.to_le_bytes());
                    }
                    "false" => {
                        self.code.push(0x17); // TpushInt
                        self.code.extend_from_slice(&0i64.to_le_bytes());
                    }
                    // COMP-TRIT-001: trit aliases that arrive as Ident if lexer misses them
                    "affirm" => { self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Affirm])); }
                    "hold" | "tend" => { self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend])); }
                    "reject" => { self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Reject])); }
                    _ => {
                        if let Some(&r) = self.symbols.get(name) {
                            self.code.push(0x09); self.code.push(r);
                        }
                    }
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
                    // `print` is an alias for `println` — same TPRINT opcode (0x20)
                    "println" | "print" => {
                        if args.is_empty() {
                            // print newline only (not implemented, but let's push dummy)
                        } else {
                            for a in args {
                                self.emit_expr(a);
                                self.code.push(0x20); // TPRINT
                            }
                        }
                        self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend])); // return hold()
                    }
                    "opent" => {
                        if args.len() == 2 {
                            for a in args { self.emit_expr(a); }
                            self.code.push(0x33); // TOPENT (pushes Int handle)
                        } else {
                            // error but push dummy
                            self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend]));
                        }
                    }
                    "readt" => {
                        if args.len() == 1 {
                            self.emit_expr(&args[0]);
                            self.code.push(0x34); // TREADT (pushes Trit)
                        } else {
                            self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend]));
                        }
                    }
                    "writet" => {
                        if args.len() == 2 {
                            for a in args { self.emit_expr(a); }
                            self.code.push(0x35); // TWRITET
                        }
                        self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend])); // push void/hold result
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
                    // VM-BUILTIN-001: `invert(t)` = ternary negation (Tneg, opcode 0x04)
                    "invert" => {
                        if args.len() == 1 {
                            self.emit_expr(&args[0]);
                            self.code.push(0x04); // Tneg
                        }
                    }
                    // VM-BUILTIN-002: `len(arr)` is an alias for `length(arr)`
                    "len" => {
                        if args.len() == 1 {
                            self.emit_expr(&args[0]);
                            self.code.push(0x24); // TSHAPE
                            self.code.push(0x0c); // TPOP (cols — TSHAPE pushes rows then cols)
                        }
                    }
                    // VM-BUILTIN-001: `abs(n)` — inline: dup, push 0, less-than, branch on negative
                    "abs" => {
                        if args.len() == 1 {
                            self.emit_expr(&args[0]);          // stack: [x]
                            self.code.push(0x0a);              // TDUP   → [x, x]
                            self.code.push(0x17);              // TpushInt 0
                            self.code.extend_from_slice(&0i64.to_le_bytes()); // → [x, x, 0]
                            self.code.push(0x14);              // Tless: (x < 0) → Affirm; [x, cmp]
                            // TjmpPos (peek) to negate branch
                            let neg_patch = self.code.len() + 1;
                            self.code.push(0x05); self.code.extend_from_slice(&[0, 0]);
                            // not negative: pop cmp, jump to end
                            self.code.push(0x0c);              // TPOP → [x]
                            let end_patch = self.code.len() + 1;
                            self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                            // negate branch: pop cmp, negate x
                            let neg_addr = self.code.len() as u16;
                            self.patch_u16(neg_patch, neg_addr);
                            self.code.push(0x0c);              // TPOP → [x]
                            self.code.push(0x04);              // Tneg → [-x] (positive when x<0)
                            let end_addr = self.code.len() as u16;
                            self.patch_u16(end_patch, end_addr);
                        }
                    }
                    // VM-BUILTIN-001: `min(a, b)` — inline with temp registers
                    "min" => {
                        if args.len() == 2 {
                            let a_reg = self.alloc_reg();
                            let b_reg = self.alloc_reg();
                            self.emit_expr(&args[0]);
                            self.code.push(0x08); self.code.push(a_reg); // TSTORE a
                            self.emit_expr(&args[1]);
                            self.code.push(0x08); self.code.push(b_reg); // TSTORE b
                            self.code.push(0x09); self.code.push(a_reg); // TLOAD a
                            self.code.push(0x09); self.code.push(b_reg); // TLOAD b
                            self.code.push(0x14);                         // Tless: a < b → Affirm
                            // TjmpPos → a is smaller, return a
                            let a_smaller_patch = self.code.len() + 1;
                            self.code.push(0x05); self.code.extend_from_slice(&[0, 0]);
                            // a >= b: return b
                            self.code.push(0x0c);              // TPOP cmp
                            self.code.push(0x09); self.code.push(b_reg); // TLOAD b
                            let end_patch = self.code.len() + 1;
                            self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                            // a < b: return a
                            let a_smaller_addr = self.code.len() as u16;
                            self.patch_u16(a_smaller_patch, a_smaller_addr);
                            self.code.push(0x0c);              // TPOP cmp
                            self.code.push(0x09); self.code.push(a_reg); // TLOAD a
                            let end_addr = self.code.len() as u16;
                            self.patch_u16(end_patch, end_addr);
                        }
                    }
                    // VM-BUILTIN-001: `max(a, b)` — inline with temp registers
                    "max" => {
                        if args.len() == 2 {
                            let a_reg = self.alloc_reg();
                            let b_reg = self.alloc_reg();
                            self.emit_expr(&args[0]);
                            self.code.push(0x08); self.code.push(a_reg); // TSTORE a
                            self.emit_expr(&args[1]);
                            self.code.push(0x08); self.code.push(b_reg); // TSTORE b
                            self.code.push(0x09); self.code.push(b_reg); // TLOAD b
                            self.code.push(0x09); self.code.push(a_reg); // TLOAD a
                            self.code.push(0x14);                         // Tless: b < a → Affirm
                            // TjmpPos → a is larger, return a
                            let a_larger_patch = self.code.len() + 1;
                            self.code.push(0x05); self.code.extend_from_slice(&[0, 0]);
                            // b >= a: return b
                            self.code.push(0x0c);              // TPOP cmp
                            self.code.push(0x09); self.code.push(b_reg); // TLOAD b
                            let end_patch = self.code.len() + 1;
                            self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                            // b < a: return a
                            let a_larger_addr = self.code.len() as u16;
                            self.patch_u16(a_larger_patch, a_larger_addr);
                            self.code.push(0x0c);              // TPOP cmp
                            self.code.push(0x09); self.code.push(a_reg); // TLOAD a
                            let end_addr = self.code.len() as u16;
                            self.patch_u16(end_patch, end_addr);
                        }
                    }
                    // `pow(base, exp)` — integer power via loop: result = 1; while exp>0 { result*=base; exp-=1; }
                    "pow" => {
                        if args.len() == 2 {
                            let b_reg = self.alloc_reg(); // base
                            let e_reg = self.alloc_reg(); // exponent
                            let r_reg = self.alloc_reg(); // result
                            // store base
                            self.emit_expr(&args[0]);
                            self.code.push(0x08); self.code.push(b_reg);
                            // store exp
                            self.emit_expr(&args[1]);
                            self.code.push(0x08); self.code.push(e_reg);
                            // result = 1
                            self.code.push(0x17); self.code.extend_from_slice(&1i64.to_le_bytes());
                            self.code.push(0x08); self.code.push(r_reg);
                            // loop_start: check e > 0
                            let loop_start = self.code.len() as u16;
                            self.code.push(0x09); self.code.push(e_reg);  // TLOAD e
                            self.code.push(0x17); self.code.extend_from_slice(&0i64.to_le_bytes()); // push 0
                            self.code.push(0x15);  // Tgreater: e > 0 → Affirm
                            // TjmpPos → jump to loop body
                            let body_patch = self.code.len() + 1;
                            self.code.push(0x05); self.code.extend_from_slice(&[0, 0]);
                            // e <= 0: pop cmp, jump to end
                            self.code.push(0x0c);
                            let end_patch = self.code.len() + 1;
                            self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                            // loop body:
                            let body_addr = self.code.len() as u16;
                            self.patch_u16(body_patch, body_addr);
                            self.code.push(0x0c);  // TPOP cmp
                            // r = r * b
                            self.code.push(0x09); self.code.push(r_reg);
                            self.code.push(0x09); self.code.push(b_reg);
                            self.code.push(0x03);  // Tmul (handles int*int)
                            self.code.push(0x08); self.code.push(r_reg);
                            // e = e - 1
                            self.code.push(0x09); self.code.push(e_reg);
                            self.code.push(0x17); self.code.extend_from_slice(&(-1i64).to_le_bytes());
                            self.code.push(0x18);  // TaddInt
                            self.code.push(0x08); self.code.push(e_reg);
                            // jump back to loop_start
                            self.code.push(0x0b); self.code.extend_from_slice(&loop_start.to_le_bytes());
                            // end:
                            let end_addr = self.code.len() as u16;
                            self.patch_u16(end_patch, end_addr);
                            // push result
                            self.code.push(0x09); self.code.push(r_reg);
                        }
                    }
                    // `push(arr, val)` / `pop(arr)` — tensor mutation not yet implemented.
                    // Emit argument expressions for side-effects then push a Tend stub so
                    // callers get a value without falling through to an unresolved TCALL 0x0000
                    // (which causes infinite recursion via jump-to-program-start).
                    "push" => {
                        for a in args { self.emit_expr(a); self.code.push(0x0c); } // eval + discard
                        self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend])); // stub result
                    }
                    "pop" => {
                        for a in args { self.emit_expr(a); self.code.push(0x0c); } // eval + discard
                        self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend])); // stub result
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
                // Emit TNODEID (0x36): defers binding to runtime so that
                // `--node-addr` / vm.set_node_id() is actually respected.
                // Previously this emitted a hardcoded "127.0.0.1:7373" string
                // literal at compile time, which meant distributed modules always
                // announced the wrong address when deployed with a custom node addr.
                self.code.push(0x36); // TNODEID — pushes Value::String(vm.node_id)
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
