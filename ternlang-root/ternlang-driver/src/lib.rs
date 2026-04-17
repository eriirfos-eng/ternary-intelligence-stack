//! ternlang-driver: Universal Hardware Abstraction Layer (HAL) for the BET VM.
//!
//! Provides a standardized interface for the BET VM to communicate with physical 
//! and virtual ternary hardware, including FPGA clusters, Harmony OS microkernels,
//! and high-performance software emulators.

use ternlang_core::trit::Trit;

#[derive(Debug, Clone, Copy)]
pub enum BackendType {
    SoftwareEmulator,
    FpgaCluster,
    HarmonyMicrokernel,
    AsicNative,
}

pub trait TernaryDriver {
    /// Initialises the connection to the hardware backend.
    fn connect(&mut self) -> Result<(), String>;

    /// Dispatches a raw 9-trit instruction word to the hardware.
    fn dispatch_opcode(&self, opcode: u8, operands: &[Trit]) -> Result<Trit, String>;

    /// Synchronizes the VM register state with hardware register banks.
    fn sync_registers(&self, registers: &mut [Trit]) -> Result<(), String>;

    /// Engages the hardware THOLD (State 0) if consensus is lost.
    fn engage_hold(&self) -> Result<(), String>;
}

pub struct BetHal {
    pub backend: BackendType,
}

impl BetHal {
    pub fn new(backend: BackendType) -> Self {
        Self { backend }
    }
}
