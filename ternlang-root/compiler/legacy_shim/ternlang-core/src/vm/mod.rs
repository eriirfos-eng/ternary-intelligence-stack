pub mod bet;

use crate::trit::Trit;
use crate::vm::bet::{unpack_trits, BetFault};

use std::fmt;
use std::sync::Arc;

// ─── Remote transport trait ───────────────────────────────────────────────────

pub trait RemoteTransport: Send + Sync {
    fn remote_send(&self, node_addr: &str, agent_id: usize, trit: i8) -> std::io::Result<()>;
    fn remote_await(&self, node_addr: &str, agent_id: usize) -> std::io::Result<i8>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum VmError {
    StackUnderflow,
    BetFault(BetFault),
    Halt,
    InvalidOpcode(u8),
    InvalidRegister(u8),
    PcOutOfBounds(usize),
    TypeMismatch { expected: String, found: String },
    // ── Tensor errors ────────────────────────────────────────────────────────
    TensorIndexOutOfBounds { tensor_id: usize, index: usize, size: usize },
    TensorNotAllocated(usize),
    // ── Agent errors ─────────────────────────────────────────────────────────
    AgentTypeNotRegistered(u16),
    AgentIdInvalid(usize),
    RuntimeError(String),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::StackUnderflow =>
                write!(f, "[BET-001] Stack underflow — you tried to pop a truth that wasn't there.\n          → details: stdlib/errors/BET-001.tern  |  ternlang errors BET-001"),
            VmError::BetFault(fault) =>
                write!(f, "[BET-002] BET encoding fault: {fault:?}. The 0b00 state is forbidden — only 01/10/11 are valid trit bits.\n          → details: stdlib/errors/BET-002.tern  |  ternlang errors BET-002"),
            VmError::Halt =>
                write!(f, "[BET-003] VM halted cleanly. Execution reached the end. This is not an error — this is peace.\n          → details: stdlib/errors/BET-003.tern  |  ternlang errors BET-003"),
            VmError::InvalidOpcode(op) =>
                write!(f, "[BET-004] Unknown opcode 0x{op:02x} — the machine has never seen this instruction. Delete cached .ternbc files and recompile.\n          → details: stdlib/errors/BET-004.tern  |  ternlang errors BET-004"),
            VmError::InvalidRegister(reg) =>
                write!(f, "[BET-005] Register {reg} is out of range. The BET has exactly 27 registers (0–26). That's 3³. No more.\n          → details: stdlib/errors/BET-005.tern  |  ternlang errors BET-005"),
            VmError::PcOutOfBounds(pc) =>
                write!(f, "[BET-006] PC {pc} is out of bounds — you jumped outside the known universe. Recompile from source.\n          → details: stdlib/errors/BET-006.tern  |  ternlang errors BET-006"),
            VmError::TypeMismatch { expected, found } =>
                write!(f, "[BET-007] Runtime type mismatch — expected {expected} but found {found}. Square peg, round hole.\n          → details: stdlib/errors/BET-007.tern  |  ternlang errors BET-007"),
            VmError::TensorIndexOutOfBounds { tensor_id, index, size } =>
                write!(f, "[BET-008] Tensor[{tensor_id}]: index {index} is out of bounds — tensor only has {size} element(s). Trittensors don't grow on access.\n          → details: stdlib/errors/BET-008.tern  |  ternlang errors BET-008"),
            VmError::TensorNotAllocated(idx) =>
                write!(f, "[BET-009] TensorRef({idx}) doesn't exist — you never allocated it. TALLOC first, then TIDX.\n          → details: stdlib/errors/BET-009.tern  |  ternlang errors BET-009"),
            VmError::AgentTypeNotRegistered(type_id) =>
                write!(f, "[BET-010] Agent type_id 0x{type_id:04x} was never registered. You can't spawn what was never declared.\n          → details: stdlib/errors/BET-010.tern  |  ternlang errors BET-010"),
            VmError::AgentIdInvalid(id) =>
                write!(f, "[BET-011] Agent #{id} doesn't exist — no agent was spawned at this ID. TSEND and TAWAIT require a live agent.\n          → details: stdlib/errors/BET-011.tern  |  ternlang errors BET-011"),
            VmError::RuntimeError(msg) =>
                write!(f, "[BET-012] Runtime error: {msg}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Trit(Trit),
    Int(i64),
    Float(f64),
    String(String),
    TensorRef(usize),
    AgentRef(usize, Option<String>),
}

impl Default for Value {
    fn default() -> Self {
        Value::Trit(Trit::Tend)
    }
}

struct TensorInstance {
    data: Vec<Trit>,
    rows: usize,
    cols: usize,
}

struct AgentInstance {
    handler_addr: usize,
    mailbox: std::collections::VecDeque<Value>,
}

pub struct BetVm {
    registers: [Value; 27],
    register_stack: Vec<[Value; 27]>,
    carry_reg: Trit,
    stack: Vec<Value>,
    call_stack: Vec<usize>,
    tensors: Vec<TensorInstance>,
    agents: Vec<AgentInstance>,
    agent_types: std::collections::HashMap<u16, usize>,
    pc: usize,
    code: Vec<u8>,
    node_id: String,
    remote: Option<Arc<dyn RemoteTransport>>,
    instructions_count: u64,
    pub print_log: Vec<String>,
}

impl BetVm {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            registers: std::array::from_fn(|_| Value::default()),
            register_stack: Vec::new(),
            carry_reg: Trit::Tend,
            stack: Vec::new(),
            call_stack: Vec::new(),
            tensors: Vec::new(),
            agents: Vec::new(),
            agent_types: std::collections::HashMap::new(),
            pc: 0,
            code,
            node_id: "127.0.0.1:7373".to_string(),
            remote: None,
            instructions_count: 0,
            print_log: Vec::new(),
        }
    }

    /// Drain all lines printed by `print()`/`println()` during execution.
    pub fn take_output(&mut self) -> Vec<String> {
        std::mem::take(&mut self.print_log)
    }

    pub fn set_node_id(&mut self, node_id: String) {
        self.node_id = node_id;
    }

    pub fn set_remote(&mut self, transport: Arc<dyn RemoteTransport>) {
        self.remote = Some(transport);
    }

    pub fn register_agent_type(&mut self, type_id: u16, handler_addr: usize) {
        self.agent_types.insert(type_id, handler_addr);
    }

    pub fn peek_stack(&self) -> Option<Value> {
        self.stack.last().cloned()
    }

    pub fn get_register(&self, reg: u8) -> Value {
        if reg < 27 { self.registers[reg as usize].clone() } else { Value::default() }
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        loop {
            if self.pc >= self.code.len() { break; }
            let opcode = self.code[self.pc];
            self.pc += 1;

            match opcode {
                0x01 => { // Tpush
                    let packed = self.read_u8()?;
                    let trits = unpack_trits(&[packed], 1).map_err(VmError::BetFault)?;
                    self.stack.push(Value::Trit(trits[0]));
                }
                0x02 => { // Tadd
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Trit(av), Value::Trit(bv)) => {
                            let (sum, carry) = av + bv;
                            self.stack.push(Value::Trit(sum));
                            self.carry_reg = carry;
                        }
                        (Value::Int(av), Value::Int(bv)) => self.stack.push(Value::Int(av + bv)),
                        (Value::Float(av), Value::Float(bv)) => self.stack.push(Value::Float(av + bv)),
                        (Value::Int(av), Value::Trit(bv)) => self.stack.push(Value::Int(av + bv as i64)),
                        (Value::Trit(av), Value::Int(bv)) => self.stack.push(Value::Int(av as i64 + bv)),
                        _ => return Err(VmError::TypeMismatch { expected: "Numeric".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x03 => { // Tmul
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Trit(av), Value::Trit(bv)) => self.stack.push(Value::Trit(av * bv)),
                        (Value::Int(av), Value::Int(bv)) => self.stack.push(Value::Int(av * bv)),
                        (Value::Float(av), Value::Float(bv)) => self.stack.push(Value::Float(av * bv)),
                        (Value::Int(av), Value::Trit(bv)) => self.stack.push(Value::Int(av * bv as i64)),
                        (Value::Trit(av), Value::Int(bv)) => self.stack.push(Value::Int(av as i64 * bv)),
                        _ => return Err(VmError::TypeMismatch { expected: "Numeric".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x04 => { // Tneg
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match a.clone() {
                        Value::Trit(av) => self.stack.push(Value::Trit(-av)),
                        Value::Int(av) => self.stack.push(Value::Int(-av)),
                        Value::Float(av) => self.stack.push(Value::Float(-av)),
                        _ => return Err(VmError::TypeMismatch { expected: "Numeric".into(), found: format!("{:?}", a) }),
                    }
                }
                0x05 => { // TjmpPos — jumps if top is +1
                    let addr = self.read_u16()?;
                    let val = self.stack.last().ok_or(VmError::StackUnderflow)?;
                    let is_pos = match val {
                        Value::Trit(Trit::Affirm) => true,
                        Value::Int(1) => true,
                        _ => false,
                    };
                    if is_pos { self.pc = addr as usize; }
                }
                0x06 => { // TjmpZero — jumps if top is 0
                    let addr = self.read_u16()?;
                    let val = self.stack.last().ok_or(VmError::StackUnderflow)?;
                    let is_zero = match val {
                        Value::Trit(Trit::Tend) => true,
                        Value::Int(0) => true,
                        _ => false,
                    };
                    if is_zero { self.pc = addr as usize; }
                }
                0x07 => { // TjmpNeg — jumps if top is -1
                    let addr = self.read_u16()?;
                    let val = self.stack.last().ok_or(VmError::StackUnderflow)?;
                    let is_neg = match val {
                        Value::Trit(Trit::Reject) => true,
                        Value::Int(-1) => true,
                        _ => false,
                    };
                    if is_neg { self.pc = addr as usize; }
                }
                0x08 => { // Tstore
                    let reg = self.read_u8()?;
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if reg < 27 { self.registers[reg as usize] = val; }
                }
                0x09 => { // Tload
                    let reg = self.read_u8()?;
                    if reg < 27 { self.stack.push(self.registers[reg as usize].clone()); }
                    else { self.stack.push(Value::default()); }
                }
                0x0a => { // Tdup
                    let val = self.stack.last().ok_or(VmError::StackUnderflow)?;
                    self.stack.push(val.clone());
                }
                0x0b => { // Tjmp
                    let addr = self.read_u16()?;
                    self.pc = addr as usize;
                }
                0x0c => { // Tpop
                    self.stack.pop().ok_or(VmError::StackUnderflow)?;
                }
                0x0e => { // Tcons
                    let b_val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a_val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    
                    let a = match a_val {
                        Value::Trit(t) => t,
                        Value::Int(v) if v == 1 => Trit::Affirm,
                        Value::Int(v) if v == 0 => Trit::Tend,
                        Value::Int(v) if v == -1 => Trit::Reject,
                        _ => return Err(VmError::TypeMismatch { expected: "Trit or Int(-1..1)".into(), found: format!("{:?}", a_val) }),
                    };
                    let b = match b_val {
                        Value::Trit(t) => t,
                        Value::Int(v) if v == 1 => Trit::Affirm,
                        Value::Int(v) if v == 0 => Trit::Tend,
                        Value::Int(v) if v == -1 => Trit::Reject,
                        _ => return Err(VmError::TypeMismatch { expected: "Trit or Int(-1..1)".into(), found: format!("{:?}", b_val) }),
                    };

                    let result = match (a, b) {
                        (Trit::Affirm, Trit::Affirm) => Trit::Affirm,
                        (Trit::Reject, Trit::Reject) => Trit::Reject,
                        (Trit::Tend, x) => x,
                        (x, Trit::Tend) => x,
                        _ => Trit::Tend,
                    };
                    self.stack.push(Value::Trit(result));
                }
                0x0f => { // Talloc
                    let rows = self.read_u16()? as usize;
                    let cols = self.read_u16()? as usize;
                    let size = rows * cols;
                    let idx = self.tensors.len();
                    self.tensors.push(TensorInstance {
                        data: vec![Trit::Tend; size],
                        rows,
                        cols,
                    });
                    self.stack.push(Value::TensorRef(idx));
                }
                0x10 => { // Tcall
                    let addr = self.read_u16()? as usize;
                    self.register_stack.push(self.registers.clone());
                    self.call_stack.push(self.pc);
                    self.pc = addr;
                }
                0x11 => { // Tret
                    if let Some(prev) = self.register_stack.pop() {
                        self.registers = prev;
                    }
                    match self.call_stack.pop() {
                        Some(ret) => self.pc = ret,
                        None => return Ok(()),
                    }
                }
                0x14 => { // Tless
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Int(x), Value::Int(y)) => {
                            let r = if x < y { Trit::Affirm } else if x == y { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Float(x), Value::Float(y)) => {
                            let r = if x < y { Trit::Affirm } else if (x - y).abs() < f64::EPSILON { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Int(x), Value::Trit(y)) => {
                            let bv = y as i64;
                            let r = if x < bv { Trit::Affirm } else if x == bv { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Trit(x), Value::Int(y)) => {
                            let av = x as i64;
                            let r = if av < y { Trit::Affirm } else if av == y { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Trit(x), Value::Trit(y)) => {
                            let av = x as i64;
                            let bv = y as i64;
                            let r = if av < bv { Trit::Affirm } else if av == bv { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Numeric".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x15 => { // Tgreater
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Int(x), Value::Int(y)) => {
                            let r = if x > y { Trit::Affirm } else if x == y { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Float(x), Value::Float(y)) => {
                            let r = if x > y { Trit::Affirm } else if (x - y).abs() < f64::EPSILON { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Int(x), Value::Trit(y)) => {
                            let bv = y as i64;
                            let r = if x > bv { Trit::Affirm } else if x == bv { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Trit(x), Value::Int(y)) => {
                            let av = x as i64;
                            let r = if av > y { Trit::Affirm } else if av == y { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        (Value::Trit(x), Value::Trit(y)) => {
                            let av = x as i64;
                            let bv = y as i64;
                            let r = if av > bv { Trit::Affirm } else if av == bv { Trit::Tend } else { Trit::Reject };
                            self.stack.push(Value::Trit(r));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Numeric".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x16 => { // Teq
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let is_eq = match (a.clone(), b.clone()) {
                        (Value::Int(av), Value::Trit(bv)) => av == bv as i64,
                        (Value::Trit(av), Value::Int(bv)) => av as i64 == bv,
                        (Value::Float(av), Value::Float(bv)) => (av - bv).abs() < f64::EPSILON,
                        _ => a == b,
                    };
                    let r = if is_eq { Trit::Affirm } else { Trit::Reject };
                    self.stack.push(Value::Trit(r));
                }
                0x17 => { // TpushInt
                    let mut b = [0u8; 8];
                    for i in 0..8 { b[i] = self.read_u8()?; }
                    self.stack.push(Value::Int(i64::from_le_bytes(b)));
                }
                0x18 => { // TaddInt
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Int(x), Value::Int(y)) => self.stack.push(Value::Int(x + y)),
                        _ => return Err(VmError::TypeMismatch { expected: "Int".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x19 => { // TpushFloat
                    let mut b = [0u8; 8];
                    for i in 0..8 { b[i] = self.read_u8()?; }
                    self.stack.push(Value::Float(f64::from_le_bytes(b)));
                }
                0x1e => { // Tdiv
                    let b_val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a_val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a_val.clone(), b_val.clone()) {
                        (Value::Int(av), Value::Int(bv)) => {
                            if bv == 0 { return Err(VmError::RuntimeError("Division by zero".into())); }
                            self.stack.push(Value::Int(av / bv));
                        }
                        (Value::Float(av), Value::Float(bv)) => {
                            if bv == 0.0 { return Err(VmError::RuntimeError("Division by zero".into())); }
                            self.stack.push(Value::Float(av / bv));
                        }
                        (Value::Int(av), Value::Trit(bv)) => {
                            let b = bv as i64;
                            if b == 0 { return Err(VmError::RuntimeError("Division by zero".into())); }
                            self.stack.push(Value::Int(av / b));
                        }
                        (Value::Trit(av), Value::Int(bv)) => {
                            if bv == 0 { return Err(VmError::RuntimeError("Division by zero".into())); }
                            self.stack.push(Value::Int(av as i64 / bv));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Numeric".into(), found: format!("{:?}", (a_val, b_val)) }),
                    }
                }
                0x1f => { // Tmod
                    let b_val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a_val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a_val.clone(), b_val.clone()) {
                        (Value::Int(av), Value::Int(bv)) => {
                            if bv == 0 { return Err(VmError::RuntimeError("Modulo by zero".into())); }
                            self.stack.push(Value::Int(av % bv));
                        }
                        (Value::Int(av), Value::Trit(bv)) => {
                            let b = bv as i64;
                            if b == 0 { return Err(VmError::RuntimeError("Modulo by zero".into())); }
                            self.stack.push(Value::Int(av % b));
                        }
                        (Value::Trit(av), Value::Int(bv)) => {
                            if bv == 0 { return Err(VmError::RuntimeError("Modulo by zero".into())); }
                            self.stack.push(Value::Int(av as i64 % bv));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", (a_val, b_val)) }),
                    }
                }
                0x20 => { // Tprint
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let line = match &val {
                        Value::Trit(t) => format!("{:?}", t),
                        Value::Int(i) => format!("{}", i),
                        Value::Float(f) => format!("{}", f),
                        Value::String(s) => s.clone(),
                        Value::TensorRef(idx) => format!("TensorRef({})", idx),
                        Value::AgentRef(idx, addr) => format!("AgentRef({}, {:?})", idx, addr),
                    };
                    println!("{}", line);
                    self.print_log.push(line);
                }
                0x21 => { // TpushString
                    let len = self.read_u16()? as usize;
                    let mut bytes = vec![0u8; len];
                    for i in 0..len { bytes[i] = self.read_u8()?; }
                    let s = String::from_utf8(bytes).map_err(|_| VmError::RuntimeError("Invalid UTF-8 string".into()))?;
                    self.stack.push(Value::String(s));
                }
                0x22 => { // Tidx
                    let col = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let row = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let rf = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let r = match row { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", row) }) };
                    let c = match col { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", col) }) };
                    match rf {
                        Value::TensorRef(idx) => {
                            if idx >= self.tensors.len() {
                                return Err(VmError::TensorNotAllocated(idx));
                            }
                            let tensor = &self.tensors[idx];
                            let pos = if tensor.cols > 1 { r as usize * tensor.cols + c as usize } else { r as usize };
                            if pos >= tensor.data.len() {
                                return Err(VmError::TensorIndexOutOfBounds { tensor_id: idx, index: pos, size: tensor.data.len() });
                            }
                            self.stack.push(Value::Trit(tensor.data[pos]));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "TensorRef".into(), found: format!("{:?}", rf) }),
                    }
                }
                0x23 => { // Tset
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let col = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let row = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let rf = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let r = match row { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", row) }) };
                    let c = match col { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", col) }) };
                    match (rf.clone(), val.clone()) {
                        (Value::TensorRef(idx), Value::Trit(t)) => {
                            if idx >= self.tensors.len() { return Err(VmError::TensorNotAllocated(idx)); }
                            let tensor = &mut self.tensors[idx];
                            let pos = if tensor.cols > 1 { r as usize * tensor.cols + c as usize } else { r as usize };
                            if pos >= tensor.data.len() { return Err(VmError::TensorIndexOutOfBounds { tensor_id: idx, index: pos, size: tensor.data.len() }); }
                            tensor.data[pos] = t;
                        }
                        (Value::TensorRef(idx), Value::Int(v)) => {
                            if idx >= self.tensors.len() { return Err(VmError::TensorNotAllocated(idx)); }
                            let tensor = &mut self.tensors[idx];
                            let pos = if tensor.cols > 1 { r as usize * tensor.cols + c as usize } else { r as usize };
                            if pos >= tensor.data.len() { return Err(VmError::TensorIndexOutOfBounds { tensor_id: idx, index: pos, size: tensor.data.len() }); }
                            tensor.data[pos] = Trit::from(v as i8);
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "TensorRef, Trit".into(), found: format!("{:?}", (rf, val)) }),
                    }
                }
                0x24 => { // Tshape
                    let rf = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if let Value::TensorRef(idx) = rf {
                        if idx >= self.tensors.len() {
                            return Err(VmError::TensorNotAllocated(idx));
                        }
                        let tensor = &self.tensors[idx];
                        self.stack.push(Value::Int(tensor.rows as i64));
                        self.stack.push(Value::Int(tensor.cols as i64));
                    } else { return Err(VmError::TypeMismatch { expected: "TensorRef".into(), found: format!("{:?}", rf) }); }
                }
                0x30 => { // Tspawn — (type_id) → AgentRef
                    let type_id = self.read_u16()?;
                    if let Some(&handler_addr) = self.agent_types.get(&type_id) {
                        let id = self.agents.len();
                        self.agents.push(AgentInstance { handler_addr, mailbox: Default::default() });
                        self.stack.push(Value::AgentRef(id, None));
                    } else {
                        return Err(VmError::AgentTypeNotRegistered(type_id));
                    }
                }
                0x31 => { // Tsend — msg, target → void
                    let msg = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let target = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if let Value::AgentRef(id, None) = target {
                        if id < self.agents.len() {
                            self.agents[id].mailbox.push_back(msg);
                        } else {
                            return Err(VmError::AgentIdInvalid(id));
                        }
                    } else {
                        return Err(VmError::TypeMismatch { expected: "Local AgentRef".into(), found: format!("{:?}", target) });
                    }
                }
                0x32 => { // Tawait — target → result
                    let target = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if let Value::AgentRef(id, None) = target {
                        if id < self.agents.len() {
                            let handler_addr = self.agents[id].handler_addr;
                            let msg = self.agents[id].mailbox.pop_front().unwrap_or(Value::default());
                            // Synchronous handler dispatch — identical to TCALL
                            self.register_stack.push(self.registers.clone());
                            self.call_stack.push(self.pc);
                            self.pc = handler_addr;
                            self.stack.push(msg);
                        } else {
                            return Err(VmError::AgentIdInvalid(id));
                        }
                    } else {
                        return Err(VmError::TypeMismatch { expected: "Local AgentRef".into(), found: format!("{:?}", target) });
                    }
                }
                0x25 => { // TjmpEqInt — imm_int, imm_addr → peek, jumps if eq
                    let mut b = [0u8; 8];
                    for i in 0..8 { b[i] = self.read_u8()?; }
                    let target_val = i64::from_le_bytes(b);
                    let addr = self.read_u16()?;
                    let val = self.stack.last().ok_or(VmError::StackUnderflow)?;
                    let is_eq = match val {
                        Value::Int(v) => *v == target_val,
                        Value::Trit(t) => (*t as i8) as i64 == target_val,
                        _ => false,
                    };
                    if is_eq { self.pc = addr as usize; }
                }
                0x26 => { // TlessEqual
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let is_le = match (a.clone(), b.clone()) {
                        (Value::Int(x), Value::Int(y)) => x <= y,
                        (Value::Float(x), Value::Float(y)) => x <= y || (x - y).abs() < f64::EPSILON,
                        (Value::Int(x), Value::Trit(y)) => x <= y as i64,
                        (Value::Trit(x), Value::Int(y)) => (x as i64) <= y,
                        (Value::Trit(x), Value::Trit(y)) => (x as i64) <= (y as i64),
                        _ => false,
                    };
                    self.stack.push(Value::Trit(if is_le { Trit::Affirm } else { Trit::Reject }));
                }
                0x27 => { // TgreaterEqual
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let is_ge = match (a.clone(), b.clone()) {
                        (Value::Int(x), Value::Int(y)) => x >= y,
                        (Value::Float(x), Value::Float(y)) => x >= y || (x - y).abs() < f64::EPSILON,
                        (Value::Int(x), Value::Trit(y)) => x >= y as i64,
                        (Value::Trit(x), Value::Int(y)) => (x as i64) >= y,
                        (Value::Trit(x), Value::Trit(y)) => (x as i64) >= (y as i64),
                        _ => false,
                    };
                    self.stack.push(Value::Trit(if is_ge { Trit::Affirm } else { Trit::Reject }));
                }
                0x00 => return Ok(()),
                _ => return Err(VmError::InvalidOpcode(opcode)),
            }
        }
        Ok(())
    }

    fn read_u8(&mut self) -> Result<u8, VmError> {
        if self.pc >= self.code.len() { return Err(VmError::PcOutOfBounds(self.pc)); }
        let val = self.code[self.pc];
        self.pc += 1;
        Ok(val)
    }

    fn read_u16(&mut self) -> Result<u16, VmError> {
        if self.pc + 1 >= self.code.len() { return Err(VmError::PcOutOfBounds(self.pc)); }
        let val = u16::from_le_bytes([self.code[self.pc], self.code[self.pc + 1]]);
        self.pc += 2;
        Ok(val)
    }
}
