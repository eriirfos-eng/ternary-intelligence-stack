use crate::ast::*;
use crate::vm::bet::pack_trits;
use crate::trit::Trit;

pub struct BytecodeEmitter {
    code: Vec<u8>,
    symbols: std::collections::HashMap<String, u8>,
    func_addrs: std::collections::HashMap<String, u16>,
    break_patches: Vec<usize>, // addresses to patch when a loop ends
    next_reg: u8,
    /// Struct layouts: struct_name → ordered field names
    struct_layouts: std::collections::HashMap<String, Vec<String>>,
    /// Agent type IDs: agent_name → type_id (u16 index)
    agent_type_ids: std::collections::HashMap<String, u16>,
    /// Agent handler addresses emitted during emit_program (type_id → addr)
    agent_handlers: Vec<(u16, u16)>,
}

impl BytecodeEmitter {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            symbols: std::collections::HashMap::new(),
            func_addrs: std::collections::HashMap::new(),
            break_patches: Vec::new(),
            next_reg: 0,
            struct_layouts: std::collections::HashMap::new(),
            agent_type_ids: std::collections::HashMap::new(),
            agent_handlers: Vec::new(),
        }
    }

    /// After emit_program, call this to wire agent handler addresses into a VM.
    pub fn register_agents(&self, vm: &mut crate::vm::BetVm) {
        for &(type_id, addr) in &self.agent_handlers {
            vm.register_agent_type(type_id, addr as usize);
        }
    }

    pub fn emit_program(&mut self, program: &Program) {
        let parent_next_reg = self.next_reg;
        // Register struct layouts so field-access codegen knows field order.
        for s in &program.structs {
            let field_names: Vec<String> = s.fields.iter().map(|(n, _)| n.clone()).collect();
            self.struct_layouts.insert(s.name.clone(), field_names);
        }
        // Register agent type IDs before emitting bodies.
        for (idx, agent) in program.agents.iter().enumerate() {
            self.agent_type_ids.insert(agent.name.clone(), idx as u16);
        }

        // Two-pass: first emit a TJMP over all function/agent bodies.
        let entry_jmp_patch = self.code.len() + 1;
        self.code.push(0x0b); // TJMP — skip over function bodies
        self.code.extend_from_slice(&[0u8, 0u8]);

        // PASS 1: Collect addresses by doing a "dry run" of emission.
        // We temporarily swap the code buffer to a dummy.
        let real_code = std::mem::take(&mut self.code);
        let real_func_addrs = std::mem::take(&mut self.func_addrs);
        let real_agent_handlers = std::mem::take(&mut self.agent_handlers);
        
        // Setup initial address for Pass 1 (should match real code len)
        let base_addr = real_code.len() as u16;

        let mut func_depth = 0;
        for agent in &program.agents {
            let type_id = self.agent_type_ids[&agent.name];
            let mut handler_addr: Option<u16> = None;
            for method in &agent.methods {
                let addr = base_addr + self.code.len() as u16;
                if handler_addr.is_none() {
                    handler_addr = Some(addr);
                }
                self.emit_function(method, func_depth);
                let fq = format!("{}::{}", agent.name, method.name);
                self.func_addrs.insert(fq, addr);
                func_depth += 1;
            }
            if let Some(addr) = handler_addr {
                self.agent_handlers.push((type_id, addr));
            }
        }

        for func in &program.functions {
            let addr = base_addr + self.code.len() as u16;
            self.func_addrs.insert(func.name.clone(), addr);
            self.emit_function(func, func_depth);
            func_depth += 1;
        }

        // Now we have all addresses in self.func_addrs. 
        // Save them and restore real state for PASS 2.
        let final_func_addrs = std::mem::replace(&mut self.func_addrs, real_func_addrs);
        let final_agent_handlers = std::mem::replace(&mut self.agent_handlers, real_agent_handlers);
        self.code = real_code;
        self.func_addrs = final_func_addrs;
        self.agent_handlers = final_agent_handlers;
        self.next_reg = parent_next_reg; // Reset next_reg for Pass 2

        // PASS 2: Real emission
        let mut func_depth = 0;
        for agent in &program.agents {
            for method in &agent.methods {
                self.emit_function(method, func_depth);
                func_depth += 1;
            }
        }
        for func in &program.functions {
            self.emit_function(func, func_depth);
            func_depth += 1;
        }

        // Patch entry jump to land after all bodies.
        let after_funcs = self.code.len() as u16;
        self.patch_u16(entry_jmp_patch, after_funcs);
    }

    pub fn emit_function(&mut self, func: &Function, depth: u8) {
        // Record address of this function's first instruction.
        let func_addr = self.code.len() as u16;
        self.func_addrs.insert(func.name.clone(), func_addr);

        // Function-local scope for symbols and registers
        let parent_symbols = self.symbols.clone();
        let parent_next_reg = self.next_reg;
        
        // Offset registers by depth to avoid clobbering caller
        let base_reg = depth * 8;
        self.next_reg = base_reg;

        // Map parameters to registers.
        for (name, _) in func.params.iter().rev() {
            let reg = self.next_reg;
            self.symbols.insert(name.clone(), reg);
            self.next_reg += 1;
            self.code.push(0x08); // TSTORE
            self.code.push(reg);
        }
        
        // Locals follow parameters
        for stmt in &func.body {
            self.emit_stmt(stmt);
        }

        // Restore parent scope
        self.symbols = parent_symbols;
        self.next_reg = parent_next_reg;

        // Emit TRET at end of every function body.
        self.code.push(0x11); // TRET
    }

    pub fn emit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty, value } => {
                match ty {
                    Type::TritTensor { dims } => {
                        let size: usize = dims.iter().product();
                        self.code.push(0x0f); // TALLOC
                        self.code.extend_from_slice(&(size as u16).to_le_bytes());
                        let reg = self.next_reg;
                        self.symbols.insert(name.clone(), reg);
                        self.next_reg += 1;
                        self.code.push(0x08); // TSTORE
                        self.code.push(reg);
                    }
                    Type::Named(struct_name) => {
                        let fields = self.struct_layouts.get(struct_name)
                            .cloned()
                            .unwrap_or_default();
                        let base_reg = self.next_reg;
                        self.symbols.insert(name.clone(), base_reg);
                        for field in &fields {
                            let reg = self.next_reg;
                            self.next_reg += 1;
                            self.symbols.insert(format!("{}.{}", name, field), reg);
                            self.code.push(0x01); // TPUSH hold
                            self.code.extend(crate::vm::bet::pack_trits(&[crate::trit::Trit::Tend]));
                            self.code.push(0x08); // TSTORE
                            self.code.push(reg);
                        }
                        if fields.is_empty() {
                            self.next_reg += 1;
                            self.code.push(0x01);
                            self.code.extend(crate::vm::bet::pack_trits(&[crate::trit::Trit::Tend]));
                            self.code.push(0x08);
                            self.code.push(base_reg);
                        }
                    }
                    _ => {
                        self.emit_expr(value);
                        let reg = self.next_reg;
                        self.symbols.insert(name.clone(), reg);
                        self.next_reg += 1;
                        self.code.push(0x08); // TSTORE
                        self.code.push(reg);
                    }
                }
            }
            Stmt::FieldSet { object, field, value } => {
                let key = format!("{}.{}", object, field);
                self.emit_expr(value);
                if let Some(&reg) = self.symbols.get(&key) {
                    self.code.push(0x08); // TSTORE
                    self.code.push(reg);
                }
            }
            Stmt::IndexSet { object, row, col, value } => {
                if let Some(&reg) = self.symbols.get(object) {
                    self.code.push(0x09); self.code.push(reg);
                    self.emit_expr(row);
                    self.emit_expr(col);
                    self.emit_expr(value);
                    self.code.push(0x23); // TSET
                }
            }
            Stmt::IfTernary { condition, on_pos, on_zero, on_neg } => {
                self.emit_expr(condition);
                self.code.push(0x0a); // TDUP
                let jmp_pos_patch = self.code.len() + 1;
                self.code.push(0x05); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0a); // TDUP
                let jmp_zero_patch = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_neg);
                let end_jmp_neg_patch = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                let pos_addr = self.code.len() as u16;
                self.patch_u16(jmp_pos_patch, pos_addr);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_pos);
                let end_jmp_pos_patch = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                let zero_addr = self.code.len() as u16;
                self.patch_u16(jmp_zero_patch, zero_addr);
                self.code.push(0x0c); // TPOP
                self.emit_stmt(on_zero);
                let end_addr = self.code.len() as u16;
                self.patch_u16(end_jmp_neg_patch, end_addr);
                self.patch_u16(end_jmp_pos_patch, end_addr);
            }
            Stmt::Match { condition, arms } => {
                self.emit_expr(condition);
                let cond_reg = self.next_reg;
                self.next_reg += 1;
                self.code.push(0x08); // TSTORE
                self.code.push(cond_reg);

                let mut end_patches = Vec::new();
                let mut next_arm_start_patch: Option<usize> = None;

                for (val, stmt) in arms {
                    if let Some(patch) = next_arm_start_patch {
                        let addr = self.code.len() as u16;
                        self.patch_u16(patch, addr);
                    }

                    self.code.push(0x09); // TLOAD
                    self.code.push(cond_reg);
                    
                    let match_jmp_patch = self.code.len() + 1;
                    match val {
                        1  => self.code.push(0x05), // TJMP_POS
                        0  => self.code.push(0x06), // TJMP_ZERO
                        -1 => self.code.push(0x07), // TJMP_NEG
                        _  => unreachable!(),
                    }
                    self.code.extend_from_slice(&[0, 0]);
                    
                    let skip_body_patch = self.code.len() + 1;
                    self.code.push(0x0b); // TJMP to next arm
                    self.code.extend_from_slice(&[0, 0]);
                    next_arm_start_patch = Some(skip_body_patch);

                    let arm_body_addr = self.code.len() as u16;
                    // println!("Debug Emitter: Match arm {} at 0x{:04x}", val, arm_body_addr);
                    self.patch_u16(match_jmp_patch, arm_body_addr);
                    self.emit_stmt(stmt);
                    
                    let end_patch = self.code.len() + 1;
                    self.code.push(0x0b); // TJMP to end
                    self.code.extend_from_slice(&[0, 0]);
                    end_patches.push(end_patch);
                }

                if let Some(patch) = next_arm_start_patch {
                    let addr = self.code.len() as u16;
                    self.patch_u16(patch, addr);
                }

                let end_addr = self.code.len() as u16;
                for p in end_patches {
                    self.patch_u16(p, end_addr);
                }
            }
            Stmt::ForIn { var, iter, body } => {
                self.emit_expr(iter);
                let iter_reg = self.next_reg;
                self.symbols.insert(format!("__iter_{}", var), iter_reg);
                self.next_reg += 1;
                self.code.push(0x08); self.code.push(iter_reg);
                let idx_reg = self.next_reg;
                self.next_reg += 1;
                self.code.push(0x09); self.code.push(iter_reg);
                self.code.push(0x24);
                let bound_reg = self.next_reg; self.next_reg += 1;
                self.code.push(0x08); self.code.push(bound_reg);
                self.code.push(0x0c);
                self.code.push(0x01);
                self.code.extend(pack_trits(&[Trit::Tend]));
                self.code.push(0x08); self.code.push(idx_reg);
                let loop_top = self.code.len() as u16;
                self.code.push(0x09); self.code.push(iter_reg);
                self.code.push(0x01); self.code.extend(pack_trits(&[Trit::Tend]));
                self.code.push(0x09); self.code.push(idx_reg);
                self.code.push(0x22);
                let var_reg = self.next_reg; self.next_reg += 1;
                self.symbols.insert(var.clone(), var_reg);
                self.code.push(0x08); self.code.push(var_reg);
                self.emit_stmt(body);
                let jmp_back = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(jmp_back, loop_top);
            }
            Stmt::Loop { body } => {
                let loop_top = self.code.len() as u16;
                let pre_break_count = self.break_patches.len();
                self.emit_stmt(body);
                let jmp_back = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(jmp_back, loop_top);
                let after_loop = self.code.len() as u16;
                let patches: Vec<usize> = self.break_patches.drain(pre_break_count..).collect();
                for patch in patches { self.patch_u16(patch, after_loop); }
            }
            Stmt::Break => {
                let patch = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.break_patches.push(patch);
            }
            Stmt::Continue => {}
            Stmt::Use { .. } => {}
            Stmt::Send { target, message } => {
                self.emit_expr(target);
                self.emit_expr(message);
                self.code.push(0x31);
            }
            Stmt::WhileTernary { condition, on_pos, on_zero, on_neg } => {
                let loop_top = self.code.len() as u16;
                self.emit_expr(condition);
                self.code.push(0x0a);
                let jmp_pos_patch = self.code.len() + 1;
                self.code.push(0x05); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0a);
                let jmp_zero_patch = self.code.len() + 1;
                self.code.push(0x06); self.code.extend_from_slice(&[0, 0]);
                self.code.push(0x0c);
                self.emit_stmt(on_neg);
                let exit_patch = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                let pos_addr = self.code.len() as u16;
                self.patch_u16(jmp_pos_patch, pos_addr);
                self.code.push(0x0c);
                self.emit_stmt(on_pos);
                let back_pos = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(back_pos, loop_top);
                let zero_addr = self.code.len() as u16;
                self.patch_u16(jmp_zero_patch, zero_addr);
                self.code.push(0x0c);
                self.emit_stmt(on_zero);
                let back_zero = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0, 0]);
                self.patch_u16(back_zero, loop_top);
                let exit_addr = self.code.len() as u16;
                self.patch_u16(exit_patch, exit_addr);
            }
            Stmt::Return(expr) => {
                self.emit_expr(expr);
                self.code.push(0x11);
            }
            Stmt::Block(stmts) => {
                for stmt in stmts { self.emit_stmt(stmt); }
            }
            Stmt::Decorated { directive, stmt } => {
                if directive == "sparseskip" {
                    if let Stmt::Expr(inner_expr) = stmt.as_ref() {
                        if let Expr::Call { callee, args } = inner_expr {
                            if callee == "matmul" && args.len() == 2 {
                                self.emit_expr(&args[0]);
                                self.emit_expr(&args[1]);
                                self.code.push(0x21);
                                return;
                            }
                        }
                    }
                    if let Stmt::Let { name, value, .. } = stmt.as_ref() {
                        if let Expr::Call { callee, args } = value {
                            if callee == "matmul" && args.len() == 2 {
                                self.emit_expr(&args[0]);
                                self.emit_expr(&args[1]);
                                self.code.push(0x21);
                                self.code.push(0x0c);
                                let reg = self.next_reg;
                                self.symbols.insert(name.clone(), reg);
                                self.next_reg += 1;
                                self.code.push(0x08);
                                self.code.push(reg);
                                return;
                            }
                        }
                    }
                }
                self.emit_stmt(stmt);
            }
            Stmt::Expr(expr) => {
                self.emit_expr(expr);
                // For top-level expressions that aren't calls, pop the result.
                // But we need to be careful not to pop return values from main.
                // For v0.1, we'll just allow it to leak onto stack.
            }
            _ => {}
        }
    }

    fn emit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::TritLiteral(val) => {
                self.code.push(0x01);
                self.code.extend(pack_trits(&[Trit::from(*val)]));
            }
            Expr::IntLiteral(val) => {
                // HACK: Use 0x17 as TPUSH_INT
                self.code.push(0x17);
                self.code.extend_from_slice(&val.to_le_bytes());
            }
            Expr::Ident(name) => {
                if let Some(&reg) = self.symbols.get(name) {
                    self.code.push(0x09);
                    self.code.push(reg);
                }
            }
            Expr::BinaryOp { op, lhs, rhs } => {
                self.emit_expr(lhs);
                self.emit_expr(rhs);
                match op {
                    BinOp::Add      => self.code.push(0x02),
                    BinOp::Mul      => self.code.push(0x03),
                    BinOp::Sub      => { self.code.push(0x04); self.code.push(0x02); }
                    BinOp::Equal    => self.code.push(0x16),
                    BinOp::NotEqual => { self.code.push(0x16); self.code.push(0x04); }
                    BinOp::And      => self.code.push(0x03),
                    BinOp::Or       => self.code.push(0x0e),
                    BinOp::Less     => self.code.push(0x14),
                    BinOp::Greater  => self.code.push(0x15),
                }
            }
            Expr::UnaryOp { op, expr } => {
                self.emit_expr(expr);
                match op {
                    UnOp::Neg => self.code.push(0x04),
                }
            }
            Expr::Call { callee, args } => {
                // For user-defined functions:
                // We push args in forward order [arg0, arg1, arg2]
                // emit_function pops them in reverse order [arg2, arg1, arg0]
                // This is correct.
                for arg in args { self.emit_expr(arg); }
                match callee.as_str() {
                    "consensus" => { if args.len() == 2 { self.code.push(0x0e); } }
                    "invert" => { if args.len() == 1 { self.code.push(0x04); } }
                    "truth" => {
                        self.code.push(0x01);
                        self.code.extend(pack_trits(&[Trit::Affirm]));
                    }
                    "hold" => {
                        self.code.push(0x01);
                        self.code.extend(pack_trits(&[Trit::Tend]));
                    }
                    "conflict" => {
                        self.code.push(0x01);
                        self.code.extend(pack_trits(&[Trit::Reject]));
                    }
                    "matmul" => { if args.len() == 2 { self.code.push(0x20); } }
                    "sparsity" => { if args.len() == 1 { self.code.push(0x25); } }
                    "shape" => { if args.len() == 1 { self.code.push(0x24); } }
                    _ => {
                        if let Some(&addr) = self.func_addrs.get(callee) {
                            self.code.push(0x10);
                            self.code.extend_from_slice(&addr.to_le_bytes());
                        } else {
                            // Forward reference or missing: push a placeholder tend
                            self.code.push(0x01);
                            self.code.extend(pack_trits(&[Trit::Tend]));
                        }
                    }
                }
            }
            Expr::FieldAccess { object, field } => {
                if let Expr::Ident(obj_name) = object.as_ref() {
                    let key = format!("{}.{}", obj_name, field);
                    if let Some(&reg) = self.symbols.get(&key) {
                        self.code.push(0x09);
                        self.code.push(reg);
                        return;
                    }
                }
                self.code.push(0x01);
                self.code.extend(pack_trits(&[Trit::Tend]));
            }
            Expr::Index { object, row, col } => {
                self.emit_expr(object);
                self.emit_expr(row);
                self.emit_expr(col);
                self.code.push(0x22);
            }
            Expr::Propagate { expr } => {
                self.emit_expr(expr);
                self.code.push(0x0a);
                let neg_patch = self.code.len() + 1;
                self.code.push(0x07); self.code.extend_from_slice(&[0u8, 0u8]);
                let skip_patch = self.code.len() + 1;
                self.code.push(0x0b); self.code.extend_from_slice(&[0u8, 0u8]);
                let prop_addr = self.code.len() as u16;
                self.patch_u16(neg_patch, prop_addr);
                self.code.push(0x11);
                let skip_addr = self.code.len() as u16;
                self.patch_u16(skip_patch, skip_addr);
            }
            Expr::Cast { expr, .. } => { self.emit_expr(expr); }
            Expr::Spawn { agent_name, node_addr } => {
                if let Some(addr) = node_addr {
                    self.emit_expr(&Expr::StringLiteral(addr.clone()));
                    if let Some(&type_id) = self.agent_type_ids.get(agent_name) {
                        self.code.push(0x33);
                        self.code.extend_from_slice(&type_id.to_le_bytes());
                    } else {
                        self.code.push(0x01);
                        self.code.extend(pack_trits(&[Trit::Tend]));
                    }
                } else if let Some(&type_id) = self.agent_type_ids.get(agent_name) {
                    self.code.push(0x30);
                    self.code.extend_from_slice(&type_id.to_le_bytes());
                } else {
                    self.code.push(0x01);
                    self.code.extend(pack_trits(&[Trit::Tend]));
                }
            }
            Expr::StringLiteral(_s) => {
                self.code.push(0x01);
                self.code.extend(pack_trits(&[Trit::Tend]));
            }
            Expr::NodeId => { self.code.push(0x12); }
            Expr::Await { target } => {
                self.emit_expr(target);
                self.code.push(0x32);
            }
            _ => {}
        }
    }

    pub fn emit_entry_call(&mut self, func_name: &str) {
        if let Some(&addr) = self.func_addrs.get(func_name) {
            // Push a dummy return address that signals HALT. 
            // In our VM, TRET with empty call_stack halts, but if we are inside
            // a nested call, it might not work as expected if the entry isn't a real call.
            // Actually, BytecodeEmitter::finalize adds a THALT at the end.
            // If we just jump to main, main's TRET will see an empty call_stack and halt.
            // The issue might be that TCALL itself expects to push a return address.
            
            self.code.push(0x10); // TCALL
            self.code.extend_from_slice(&addr.to_le_bytes());
        }
    }

    pub fn finalize(mut self) -> Vec<u8> {
        self.code.push(0x00); // THALT
        self.code
    }

    fn patch_u16(&mut self, pos: usize, val: u16) {
        let bytes = val.to_le_bytes();
        self.code[pos] = bytes[0];
        self.code[pos + 1] = bytes[1];
    }
}
