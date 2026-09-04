#![allow(dead_code)]
use std::format;
// Kernel-level Illumos/Solaris DTrace D-Language bytecode interpreter and probe engine for SigmaOS
// Enables dynamic tracing, DIF (DTrace Intermediate Format) execution, and aggregation buffers



use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DTraceProbeKind {
    FunctionBoundaryTracing, // fbt
    Syscall,                 // syscall
    StaticallyDefined,       // sdt
    ProfileTimer,            // profile
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DTraceProbe {
    pub provider: String,
    pub module: String,
    pub function: String,
    pub name: String,
    pub kind: DTraceProbeKind,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifOpcode {
    Ld,     // Load variable
    SetX,   // Set constant
    Add,    // Addition
    Sub,    // Subtraction
    Cmp,    // Compare
    Be,     // Branch if equal
    Ret,    // Return value
    AggCount, // Aggregation count
    AggSum,   // Aggregation sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifInstruction {
    pub opcode: DifOpcode,
    pub reg_dest: u8,
    pub reg_src1: u8,
    pub reg_src2: u8,
    pub immediate: u64,
}

pub struct DTraceEngine {
    pub registered_probes: Vec<DTraceProbe>,
    pub registers: [u64; 8],
    pub aggregations: BTreeMap<String, u64>,
    pub trace_log: Vec<String>,
}

impl DTraceEngine {
    pub fn new() -> Self {
        Self {
            registered_probes: Vec::new(),
            registers: [0u64; 8],
            aggregations: BTreeMap::new(),
            trace_log: Vec::new(),
        }
    }

    pub fn register_probe(&mut self, provider: &str, module: &str, function: &str, name: &str, kind: DTraceProbeKind) -> usize {
        let probe = DTraceProbe {
            provider: provider.to_string(),
            module: module.to_string(),
            function: function.to_string(),
            name: name.to_string(),
            kind,
            is_enabled: true,
        };
        self.registered_probes.push(probe);
        self.registered_probes.len() - 1
    }

    /// Executes DIF (DTrace Intermediate Format) bytecode program on probe hit
    pub fn execute_dif_bytecode(&mut self, program: &[DifInstruction], arg0: u64) -> Result<u64, &'static str> {
        self.registers = [0u64; 8];
        self.registers[0] = arg0;

        for instr in program {
            match instr.opcode {
                DifOpcode::SetX => {
                    if instr.reg_dest >= 8 { return Err("Invalid destination register"); }
                    self.registers[instr.reg_dest as usize] = instr.immediate;
                }
                DifOpcode::Ld => {
                    if instr.reg_dest >= 8 || instr.reg_src1 >= 8 { return Err("Invalid register"); }
                    self.registers[instr.reg_dest as usize] = self.registers[instr.reg_src1 as usize];
                }
                DifOpcode::Add => {
                    if instr.reg_dest >= 8 || instr.reg_src1 >= 8 || instr.reg_src2 >= 8 { return Err("Invalid register"); }
                    self.registers[instr.reg_dest as usize] = self.registers[instr.reg_src1 as usize].wrapping_add(self.registers[instr.reg_src2 as usize]);
                }
                DifOpcode::Sub => {
                    if instr.reg_dest >= 8 || instr.reg_src1 >= 8 || instr.reg_src2 >= 8 { return Err("Invalid register"); }
                    self.registers[instr.reg_dest as usize] = self.registers[instr.reg_src1 as usize].wrapping_sub(self.registers[instr.reg_src2 as usize]);
                }
                DifOpcode::AggCount => {
                    let key = std::format!("count@reg{}", instr.reg_src1);
                    let val = self.aggregations.entry(key).or_insert(0);
                    *val += 1;
                }
                DifOpcode::AggSum => {
                    let key = std::format!("sum@reg{}", instr.reg_src1);
                    let add_val = self.registers[instr.reg_src1 as usize];
                    let val = self.aggregations.entry(key).or_insert(0);
                    *val += add_val;
                }
                DifOpcode::Ret => {
                    let ret_val = self.registers[instr.reg_dest as usize];
                    self.trace_log.push(std::format!("DTrace trace return: {}", ret_val));
                    return Ok(ret_val);
                }
                _ => {}
            }
        }
        Ok(self.registers[0])
    }
}

impl Default for DTraceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtrace_engine_execution() {
        let mut dtrace = DTraceEngine::new();
        dtrace.register_probe("fbt", "kernel", "sys_read", "entry", DTraceProbeKind::FunctionBoundaryTracing);

        let program = [
            DifInstruction { opcode: DifOpcode::SetX, reg_dest: 1, reg_src1: 0, reg_src2: 0, immediate: 100 },
            DifInstruction { opcode: DifOpcode::Add, reg_dest: 2, reg_src1: 0, reg_src2: 1, immediate: 0 },
            DifInstruction { opcode: DifOpcode::AggCount, reg_dest: 0, reg_src1: 0, reg_src2: 0, immediate: 0 },
            DifInstruction { opcode: DifOpcode::AggSum, reg_dest: 0, reg_src1: 2, reg_src2: 0, immediate: 0 },
            DifInstruction { opcode: DifOpcode::Ret, reg_dest: 2, reg_src1: 0, reg_src2: 0, immediate: 0 },
        ];

        let ret = dtrace.execute_dif_bytecode(&program, 50).unwrap();
        assert_eq!(ret, 150);
        assert_eq!(*dtrace.aggregations.get("count@reg0").unwrap(), 1);
        assert_eq!(*dtrace.aggregations.get("sum@reg2").unwrap(), 150);
    }
}
