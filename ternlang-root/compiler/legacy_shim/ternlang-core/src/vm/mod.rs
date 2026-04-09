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
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::StackUnderflow =>
                write!(f, "[BET-001] Stack underflow — you tried to pop a truth that wasn't there."),
            VmError::BetFault(fault) =>
                write!(f, "[BET-002] BET encoding fault: {fault:?}. The 0b00 state is invalid — only -1, 0, +1 exist."),
            VmError::Halt =>
                write!(f, "[BET-003] VM halted cleanly. Execution reached the end."),
            VmError::InvalidOpcode(op) =>
                write!(f, "[BET-004] Unknown opcode 0x{op:02x} — the machine doesn't know this instruction. Conflict state."),
            VmError::InvalidRegister(reg) =>
                write!(f, "[BET-005] Register {reg} is out of range. The BET has 27 registers (0–26)."),
            VmError::PcOutOfBounds(pc) =>
                write!(f, "[BET-006] PC {pc} is out of bounds — you jumped outside the known universe. Recompile."),
            VmError::TypeMismatch { expected, found } =>
                write!(f, "[BET-007] Runtime type mismatch — expected {expected} but found {found}."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Trit(Trit),
    Int(i64),
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
                        _ => return Err(VmError::TypeMismatch { expected: "Trit".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x03 => { // Tmul
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match (a.clone(), b.clone()) {
                        (Value::Trit(av), Value::Trit(bv)) => {
                            self.stack.push(Value::Trit(av * bv));
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "Trit".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x04 => { // Tneg
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    match a.clone() {
                        Value::Trit(av) => self.stack.push(Value::Trit(-av)),
                        _ => return Err(VmError::TypeMismatch { expected: "Trit".into(), found: format!("{:?}", a) }),
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
                        _ => return Err(VmError::TypeMismatch { expected: "Int".into(), found: format!("{:?}", (a, b)) }),
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
                        _ => return Err(VmError::TypeMismatch { expected: "Int".into(), found: format!("{:?}", (a, b)) }),
                    }
                }
                0x16 => { // Teq
                    let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let r = if a == b { Trit::Affirm } else { Trit::Reject };
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
                0x22 => { // Tidx
                    let col = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let row = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let rf = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    let r = match row { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", row) }) };
                    let c = match col { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", col) }) };
                    match rf {
                        Value::TensorRef(idx) => {
                            let len = self.tensors[idx].len();
                            let n = (len as f64).sqrt() as usize;
                            let pos = if n * n == len { r as usize * n + c as usize } else { r as usize };
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
                    let r = match row { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", row) }) };
                    let c = match col { Value::Int(v) => v, Value::Trit(t) => t as i64, _ => return Err(VmError::TypeMismatch { expected: "Int or Trit".into(), found: format!("{:?}", col) }) };
                    match (rf, val) {
                        (Value::TensorRef(idx), Value::Trit(t)) => {
                            let len = self.tensors[idx].len();
                            let n = (len as f64).sqrt() as usize;
                            let pos = if n * n == len { r as usize * n + c as usize } else { r as usize };
                            self.tensors[idx][pos] = t;
                        }
                        _ => return Err(VmError::TypeMismatch { expected: "TensorRef, Trit".into(), found: format!("{:?}", (rf, val)) }),
                    }
                }
                0x24 => { // Tshape
                    let rf = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                    if let Value::TensorRef(idx) = rf {
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
