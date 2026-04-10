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
    tensors: Vec<Vec<Trit>>,
    agents: Vec<AgentInstance>,
    agent_types: std::collections::HashMap<u16, usize>,
    pc: usize,
    code: Vec<u8>,
    node_id: String,
    remote: Option<Arc<dyn RemoteTransport>>,
    instructions_count: u64,
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
        }
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
                0x05 => { // TjmpPos
                    let addr = self.read_u16()?;
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if let Value::Trit(Trit::Affirm) = val { self.pc = addr as usize; }
                }
                0x06 => { // TjmpZero
                    let addr = self.read_u16()?;
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if let Value::Trit(Trit::Tend) = val { self.pc = addr as usize; }
                }
                0x07 => { // TjmpNeg
                    let addr = self.read_u16()?;
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if let Value::Trit(Trit::Reject) = val { self.pc = addr as usize; }
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
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Trit(av), Value::Trit(bv)) => {
                            let result = match (av, bv) {
                                (Trit::Affirm, Trit::Affirm) => Trit::Affirm,
                                (Trit::Reject, Trit::Reject) => Trit::Reject,
                                (Trit::Tend, x) => x,
                                (x, Trit::Tend) => x,
                                _ => Trit::Tend,
                            };
                            self.stack.push(Value::Trit(result));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Trit".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x0f => { // Talloc
                    let size = self.read_u16()? as usize;
                    let idx = self.tensors.len();
                    self.tensors.push(vec![Trit::Tend; size]);
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
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Int(av), Value::Int(bv)) => {
                            if bv == 0 { return Err(VmError::RuntimeError("Division by zero".into())); }
                            self.stack.push(Value::Int(av / bv));
                        }
                        (Value::Float(av), Value::Float(bv)) => {
                            if bv == 0.0 { return Err(VmError::RuntimeError("Division by zero".into())); }
                            self.stack.push(Value::Float(av / bv));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Numeric".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x1f => { // Tmod
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Int(av), Value::Int(bv)) => {
                            if bv == 0 { return Err(VmError::RuntimeError("Modulo by zero".into())); }
                            self.stack.push(Value::Int(av % bv));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Int".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x20 => { // Tprint
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match val {
                        Value::Trit(t) => println!("{:?}", t),
                        Value::Int(i) => println!("{}", i),
                        Value::Float(f) => println!("{}", f),
                        Value::String(s) => println!("{}", s),
                        Value::TensorRef(idx) => println!("TensorRef({})", idx),
                        Value::AgentRef(idx, addr) => println!("AgentRef({}, {:?})", idx, addr),
                    }
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
                            let len = self.tensors[idx].len();
                            let n = (len as f64).sqrt() as usize;
                            let pos = if n * n == len { r as usize * n + c as usize } else { r as usize };
                            if pos >= len {
                                return Err(VmError::TensorIndexOutOfBounds { tensor_id: idx, index: pos, size: len });
                            }
                            self.stack.push(Value::Trit(self.tensors[idx][pos]));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "TensorRef".into(), found: format!("{:?}", rf) }),
                    }
                }
                0x23 => { // Tset
                    let val = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let col = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let row = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let rf = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let r = match row { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", col) }) };
                    let c = match col { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", col) }) };
                    match (rf.clone(), val.clone()) {
                        (Value::TensorRef(idx), Value::Trit(t)) => {
                            if idx >= self.tensors.len() {
                                return Err(VmError::TensorNotAllocated(idx));
                            }
                            let len = self.tensors[idx].len();
                            let n = (len as f64).sqrt() as usize;
                            let pos = if n * n == len { r as usize * n + c as usize } else { r as usize };
                            if pos >= len {
                                return Err(VmError::TensorIndexOutOfBounds { tensor_id: idx, index: pos, size: len });
                            }
                            self.tensors[idx][pos] = t;
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
                        let len = self.tensors[idx].len();
                        let n = (len as f64).sqrt() as usize;
                        if n * n == len {
                            self.stack.push(Value::Int(n as i64));
                            self.stack.push(Value::Int(n as i64));
                        } else {
                            self.stack.push(Value::Int(len as i64));
                            self.stack.push(Value::Int(1));
                        }
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
