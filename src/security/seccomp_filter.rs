//! Seccomp-BPF Filter Compilation Engine
//!
//! Seccomp-BPF filter compilation for userland process sandboxing with
//! Berkeley Packet Filter rules for system call filtering.

/// Seccomp comparison operation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeccompCompareOp {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Greater than
    Gt,
    /// Greater than or equal
    Ge,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
    /// Masked equality
    MaskedEq,
}

impl SeccompCompareOp {
    pub fn as_bpf_op(&self) -> u16 {
        match self {
            SeccompCompareOp::Eq => 0x15,
            SeccompCompareOp::Ne => 0x15, // JEQ with true/false branches exchanged
            SeccompCompareOp::Gt => 0x25,
            SeccompCompareOp::Ge => 0x35,
            SeccompCompareOp::Lt => 0x35, // JGE with true/false branches exchanged
            SeccompCompareOp::Le => 0x25, // JGT with true/false branches exchanged
            SeccompCompareOp::MaskedEq => 0x15, // AND K followed by JEQ
        }
    }

    pub fn from_bpf_op(op: u16) -> Option<Self> {
        match op {
            0x15 => Some(SeccompCompareOp::Eq),
            0x25 => Some(SeccompCompareOp::Gt),
            0x35 => Some(SeccompCompareOp::Ge),
            _ => None,
        }
    }
}

/// Seccomp action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    /// Allow the system call
    Allow,
    /// Kill the process
    Kill,
    /// Trap the process
    Trap,
    /// Return error
    Errno(u32),
    /// Trace the process
    Trace,
    /// Log the system call
    Log,
}

impl SeccompAction {
    pub fn as_u32(&self) -> u32 {
        match self {
            SeccompAction::Allow => 0x7fff0000,
            SeccompAction::Kill => 0x00000000,
            SeccompAction::Trap => 0x00030000,
            SeccompAction::Errno(e) => 0x00050000 | (e & 0xffff),
            SeccompAction::Trace => 0x7ff00000,
            SeccompAction::Log => 0x7ffc0000,
        }
    }

    pub fn from_u32(value: u32) -> Option<Self> {
        match value & 0x7fff0000 {
            0x7fff0000 => Some(SeccompAction::Allow),
            0x00000000 => Some(SeccompAction::Kill),
            0x00030000 => Some(SeccompAction::Trap),
            0x00050000 => Some(SeccompAction::Errno(value & 0xffff)),
            0x7ff00000 => Some(SeccompAction::Trace),
            0x7ffc0000 => Some(SeccompAction::Log),
            _ => None,
        }
    }
}

/// BPF instruction
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BpfInstruction {
    pub code: u16,
    pub jt: u8,
    pub jf: u8,
    pub k: u32,
}

impl BpfInstruction {
    pub fn new(code: u16, jt: u8, jf: u8, k: u32) -> Self {
        Self { code, jt, jf, k }
    }

    /// Load word (absolute)
    pub fn ld_abs(size: u16, k: u32) -> Self {
        Self::new(0x20 | bpf_size(size), 0, 0, k)
    }

    /// Load word (indirect)
    pub fn ld_ind(size: u16, k: u32) -> Self {
        Self::new(0x40 | bpf_size(size), 0, 0, k)
    }

    /// Load half word
    pub fn ldh_abs(k: u32) -> Self {
        Self::new(0x28, 0, 0, k)
    }

    /// Load byte
    pub fn ldb_abs(k: u32) -> Self {
        Self::new(0x30, 0, 0, k)
    }

    /// Load immediate
    pub fn ld_imm(k: u32) -> Self {
        Self::new(0x00, 0, 0, k)
    }

    /// Jump if equal
    pub fn jeq(jt: u8, jf: u8, k: u32) -> Self {
        Self::new(0x15, jt, jf, k)
    }

    /// Jump if greater than
    pub fn jgt(jt: u8, jf: u8, k: u32) -> Self {
        Self::new(0x25, jt, jf, k)
    }

    /// Jump if greater than or equal
    pub fn jge(jt: u8, jf: u8, k: u32) -> Self {
        Self::new(0x35, jt, jf, k)
    }

    /// Jump if less than
    pub fn jlt(jt: u8, jf: u8, k: u32) -> Self {
        Self::new(0x35, jf, jt, k)
    }

    /// Jump if less than or equal
    pub fn jle(jt: u8, jf: u8, k: u32) -> Self {
        Self::new(0x25, jf, jt, k)
    }

    /// Jump if not equal
    pub fn jne(jt: u8, jf: u8, k: u32) -> Self {
        Self::new(0x15, jf, jt, k)
    }

    /// Jump if bits set
    pub fn jset(jt: u8, jf: u8, k: u32) -> Self {
        Self::new(0x45, jt, jf, k)
    }

    /// Return
    pub fn ret(k: u32) -> Self {
        Self::new(0x06, 0, 0, k)
    }

    /// Tax (transfer A to X)
    pub fn tax() -> Self {
        Self::new(0x07, 0, 0, 0)
    }

    /// Txa (transfer X to A)
    pub fn txa() -> Self {
        Self::new(0x87, 0, 0, 0)
    }
}

fn bpf_size(bytes: u16) -> u16 {
    match bytes {
        4 => 0x00,   // BPF_W
        2 => 0x08,   // BPF_H
        1 => 0x10,   // BPF_B
        _ => 0xffff, // Invalid size; rejected by the compiler.
    }
}

/// Seccomp filter rule
#[derive(Debug, Clone)]
pub struct SeccompRule {
    pub syscall: u32,
    pub action: SeccompAction,
    pub args: Vec<SeccompArgFilter>,
}

impl SeccompRule {
    pub fn new(syscall: u32, action: SeccompAction) -> Self {
        Self {
            syscall,
            action,
            args: Vec::new(),
        }
    }

    pub fn add_arg_filter(&mut self, arg_filter: SeccompArgFilter) {
        self.args.push(arg_filter);
    }
}

/// Seccomp argument filter
#[derive(Debug, Clone)]
pub struct SeccompArgFilter {
    pub arg_index: u32,
    pub op: SeccompCompareOp,
    pub value: u64,
    pub mask: u64,
}

impl SeccompArgFilter {
    pub fn new(arg_index: u32, op: SeccompCompareOp, value: u64) -> Self {
        Self {
            arg_index,
            op,
            value,
            mask: u64::MAX,
        }
    }

    pub fn with_mask(arg_index: u32, op: SeccompCompareOp, value: u64, mask: u64) -> Self {
        Self {
            arg_index,
            op,
            value,
            mask,
        }
    }
}

/// Error returned when a rule cannot be safely represented as classic BPF.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompCompileError {
    InvalidArgumentIndex(u32),
    ProgramTooLarge,
}

/// Seccomp filter compiler
#[derive(Debug)]
pub struct SeccompCompiler {
    rules: Vec<SeccompRule>,
    default_action: SeccompAction,
    instruction_count: u32,
}

impl SeccompCompiler {
    pub fn new(default_action: SeccompAction) -> Self {
        Self {
            rules: Vec::new(),
            default_action,
            instruction_count: 0,
        }
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: SeccompRule) {
        self.rules.push(rule);
    }

    /// Add an allow rule for a syscall
    pub fn allow_syscall(&mut self, syscall: u32) {
        let rule = SeccompRule::new(syscall, SeccompAction::Allow);
        self.add_rule(rule);
    /// Remove a rule by syscall number
    pub fn remove_rule(&mut self, syscall_number: u64) -> bool {
        if let Some(pos) = self
            .rules
            .iter()
            .position(|r| r.syscall_number == syscall_number)
        {
            self.rules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Add a deny rule for a syscall
    pub fn deny_syscall(&mut self, syscall: u32, action: SeccompAction) {
        let rule = SeccompRule::new(syscall, action);
        self.add_rule(rule);
    }

    /// Compile rules to BPF instructions
    pub fn compile(&mut self) -> Result<Vec<BpfInstruction>, SeccompCompileError> {
        const MAX_FILTER_INSNS: usize = 4096;
        let arg_count = self
            .rules
            .iter()
            .try_fold(0usize, |count, rule| count.checked_add(rule.args.len()))
            .ok_or(SeccompCompileError::ProgramTooLarge)?;
        let upper_bound = self
            .rules
            .len()
            .checked_mul(5)
            .and_then(|n| {
                arg_count
                    .checked_mul(11)
                    .and_then(|args| n.checked_add(args))
            })
            .and_then(|n| n.checked_add(1))
            .ok_or(SeccompCompileError::ProgramTooLarge)?;
        if upper_bound > MAX_FILTER_INSNS {
            return Err(SeccompCompileError::ProgramTooLarge);
        }
        let mut asm = BpfAssembler::default();
        for rule in &self.rules {
            for arg in &rule.args {
                if arg.arg_index >= 6 {
                    return Err(SeccompCompileError::InvalidArgumentIndex(arg.arg_index));
            if rule.syscall_number == syscall_number {
                // Check argument filters
                let matches = rule.arg_filters.iter().all(|filter| {
                    let arg_value = if (filter.arg_num as usize) < args.len() {
                        args[filter.arg_num as usize]
                    } else {
                        0
                    };

                    let masked_value = arg_value & filter.value_mask;
                    let masked_target = filter.value & filter.value_mask;

                    match filter.op {
                        SeccompCmpOp::Equal => masked_value == masked_target,
                        SeccompCmpOp::NotEqual => masked_value != masked_target,
                        SeccompCmpOp::LessThan => masked_value < masked_target,
                        SeccompCmpOp::LessThanOrEqual => masked_value <= masked_target,
                        SeccompCmpOp::GreaterThanOrEqual => masked_value >= masked_target,
                        SeccompCmpOp::GreaterThan => masked_value > masked_target,
                        SeccompCmpOp::MaskedEqual => {
                            (arg_value & filter.value_mask) == filter.value
                        }
                    }
                });

                if matches {
                    return rule.action;
                }
            }
        }
        let labels: Vec<_> = self.rules.iter().map(|_| asm.new_label()).collect();
        let default = asm.new_label();
        for (i, rule) in self.rules.iter().enumerate() {
            let next = labels.get(i + 1).copied().unwrap_or(default);
            asm.mark(labels[i]);
            asm.emit(BpfInstruction::ld_abs(4, 0));
            let body = asm.new_label();
            asm.branch(BpfInstruction::jeq(0, 0, rule.syscall), body, next);
            asm.mark(body);
            for arg in &rule.args {
                emit_arg_comparison(&mut asm, arg, next)?;
            }
            asm.emit(BpfInstruction::ret(rule.action.as_u32()));
        }
        asm.mark(default);
        asm.emit(BpfInstruction::ret(self.default_action.as_u32()));
        let instructions = asm.finish()?;
        if instructions.len() > MAX_FILTER_INSNS {
            return Err(SeccompCompileError::ProgramTooLarge);
        }
        self.instruction_count = instructions.len() as u32;
        Ok(instructions)
    }

    /// Get instruction count
    pub fn instruction_count(&self) -> u32 {
        self.instruction_count
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> SeccompStatistics {
        let arg_filter_count: usize = self.rules.iter().map(|r| r.args.len()).sum();

        SeccompStatistics {
            rule_count: self.rules.len(),
            arg_filter_count,
            instruction_count: self.instruction_count,
            default_action: self.default_action,
        }
    }

    /// Clear all rules
    pub fn clear(&mut self) {
        self.rules.clear();
        self.instruction_count = 0;
    }
}

#[derive(Default)]
struct BpfAssembler {
    instructions: Vec<BpfInstruction>,
    labels: Vec<Option<usize>>,
    fixups: Vec<(usize, usize)>,
}

impl BpfAssembler {
    fn new_label(&mut self) -> usize {
        self.labels.push(None);
        self.labels.len() - 1
    }

    fn mark(&mut self, label: usize) {
        self.labels[label] = Some(self.instructions.len());
    }

    fn emit(&mut self, instruction: BpfInstruction) {
        self.instructions.push(instruction);
    }

    fn jump(&mut self, label: usize) {
        let index = self.instructions.len();
        self.emit(BpfInstruction::new(0x05, 0, 0, 0)); // BPF_JMP | BPF_JA
        self.fixups.push((index, label));
    }

    fn branch(&mut self, condition: BpfInstruction, yes: usize, no: usize) {
        self.emit(BpfInstruction::new(condition.code, 0, 1, condition.k));
        self.jump(yes);
        self.jump(no);
    }

    fn finish(mut self) -> Result<Vec<BpfInstruction>, SeccompCompileError> {
        for (index, label) in self.fixups {
            let target = self.labels[label].ok_or(SeccompCompileError::ProgramTooLarge)?;
            let displacement = target
                .checked_sub(index + 1)
                .ok_or(SeccompCompileError::ProgramTooLarge)?;
            self.instructions[index].k =
                u32::try_from(displacement).map_err(|_| SeccompCompileError::ProgramTooLarge)?;
        }
        Ok(self.instructions)
    }
}

fn emit_arg_comparison(
    asm: &mut BpfAssembler,
    filter: &SeccompArgFilter,
    failure: usize,
) -> Result<(), SeccompCompileError> {
    let base = 16 + filter.arg_index * 8;
    let (high_offset, low_offset) = if cfg!(target_endian = "little") {
        (base + 4, base)
    } else {
        (base, base + 4)
    };
    let success = asm.new_label();
    if filter.op == SeccompCompareOp::MaskedEq {
        let low_half = asm.new_label();
        asm.emit(BpfInstruction::ld_abs(4, high_offset));
        asm.emit(BpfInstruction::new(0x54, 0, 0, (filter.mask >> 32) as u32));
        asm.branch(
            BpfInstruction::jeq(0, 0, ((filter.value & filter.mask) >> 32) as u32),
            low_half,
            failure,
        );
        asm.mark(low_half);
        asm.emit(BpfInstruction::ld_abs(4, low_offset));
        asm.emit(BpfInstruction::new(0x54, 0, 0, filter.mask as u32));
        asm.branch(
            BpfInstruction::jeq(0, 0, (filter.value & filter.mask) as u32),
            success,
            failure,
        );
    } else {
        let low_half = asm.new_label();
        let high = (filter.value >> 32) as u32;
        asm.emit(BpfInstruction::ld_abs(4, high_offset));
        match filter.op {
            SeccompCompareOp::Eq => asm.branch(BpfInstruction::jeq(0, 0, high), low_half, failure),
            SeccompCompareOp::Ne => asm.branch(BpfInstruction::jeq(0, 0, high), low_half, success),
            SeccompCompareOp::Gt | SeccompCompareOp::Ge => {
                let equal = asm.new_label();
                asm.branch(BpfInstruction::jgt(0, 0, high), success, equal);
                asm.mark(equal);
                asm.branch(BpfInstruction::jeq(0, 0, high), low_half, failure);
    /// Create a new filter
    pub fn create_filter(
        &mut self,
        name: String,
        default_action: SeccompAction,
        architecture: String,
    ) -> Result<(), &'static str> {
        if self.filters.contains_key(&name) {
            return Err("Filter already exists");
        }

        let filter = SeccompFilter::new(name.clone(), default_action, architecture);
        self.filters.insert(name, filter);

        Ok(())
    }

    /// Delete a filter
    pub fn delete_filter(&mut self, name: &str) -> Result<(), &'static str> {
        if self.active_filter.as_ref() == Some(&name.to_string()) {
            return Err("Cannot delete active filter");
        }

        if self.filters.remove(name).is_some() {
            Ok(())
        } else {
            Err("Filter not found")
        }
    }

    /// Get a filter
    pub fn get_filter(&self, name: &str) -> Option<&SeccompFilter> {
        self.filters.get(name)
    }

    /// Get mutable filter
    pub fn get_filter_mut(&mut self, name: &str) -> Option<&mut SeccompFilter> {
        self.filters.get_mut(name)
    }

    /// Set active filter
    pub fn set_active_filter(&mut self, name: String) -> Result<(), &'static str> {
        if !self.filters.contains_key(&name) {
            return Err("Filter not found");
        }

        self.active_filter = Some(name);
        Ok(())
    }

    /// Filter a syscall using active filter
    pub fn filter_syscall(&self, syscall_number: u64, args: &[u64]) -> SeccompAction {
        if let Some(ref active_name) = self.active_filter {
            if let Some(filter) = self.filters.get(active_name) {
                return filter.filter_syscall(syscall_number, args);
            }
            SeccompCompareOp::Lt | SeccompCompareOp::Le => {
                let equal = asm.new_label();
                asm.branch(BpfInstruction::jgt(0, 0, high), failure, equal);
                asm.mark(equal);
                asm.branch(BpfInstruction::jeq(0, 0, high), low_half, success);
            }
            SeccompCompareOp::MaskedEq => unreachable!(),
        }
        asm.mark(low_half);
        asm.emit(BpfInstruction::ld_abs(4, low_offset));
        let low = filter.value as u32;
        match filter.op {
            SeccompCompareOp::Eq => asm.branch(BpfInstruction::jeq(0, 0, low), success, failure),
            SeccompCompareOp::Ne => asm.branch(BpfInstruction::jeq(0, 0, low), failure, success),
            SeccompCompareOp::Gt => asm.branch(BpfInstruction::jgt(0, 0, low), success, failure),
            SeccompCompareOp::Ge => asm.branch(BpfInstruction::jge(0, 0, low), success, failure),
            SeccompCompareOp::Lt => asm.branch(BpfInstruction::jge(0, 0, low), failure, success),
            SeccompCompareOp::Le => asm.branch(BpfInstruction::jgt(0, 0, low), failure, success),
            SeccompCompareOp::MaskedEq => unreachable!(),

        // No active filter - allow by default
        SeccompAction::Allow
    }

    /// Get active filter name
    pub fn active_filter(&self) -> Option<&str> {
        self.active_filter.as_deref()
    }

    /// Get filter count
    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }

    /// List all filters
    pub fn list_filters(&self) -> Vec<String> {
        self.filters.keys().cloned().collect()
    }

    /// Create a strict filter (deny all except allowed)
    pub fn create_strict_filter(
        &mut self,
        name: String,
        allowed_syscalls: Vec<u64>,
    ) -> Result<(), &'static str> {
        self.create_filter(
            name.clone(),
            SeccompAction::KillProcess,
            "x86_64".to_string(),
        )?;

        let filter = self.get_filter_mut(&name).unwrap();

        // Add allow rules for allowed syscalls
        for syscall_num in allowed_syscalls {
            let rule = SeccompRule {
                syscall_number: syscall_num,
                action: SeccompAction::Allow,
                arg_filters: Vec::new(),
            };
            filter.add_rule(rule);
        }
    }
    asm.mark(success);
    Ok(())
}

impl Default for SeccompCompiler {
    fn default() -> Self {
        Self::new(SeccompAction::Kill)
    }
}

/// Seccomp statistics
#[derive(Debug, Clone)]
pub struct SeccompStatistics {
    pub rule_count: usize,
    pub arg_filter_count: usize,
    pub instruction_count: u32,
    pub default_action: SeccompAction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seccomp_compare_op() {
        assert_eq!(SeccompCompareOp::Eq.as_bpf_op(), 0x15);
        assert_eq!(SeccompCompareOp::Ne.as_bpf_op(), 0x15);
        assert_eq!(
            SeccompCompareOp::from_bpf_op(0x15),
            Some(SeccompCompareOp::Eq)
        );
    }

    #[test]
    fn test_seccomp_action() {
        assert_eq!(SeccompAction::Allow.as_u32(), 0x7fff0000);
        assert_eq!(SeccompAction::Kill.as_u32(), 0x00000000);
        assert_eq!(SeccompAction::Errno(1).as_u32(), 0x00050001);
        assert_eq!(
            SeccompAction::from_u32(0x7fff0000),
            Some(SeccompAction::Allow)
        );
    fn test_create_filter() {
        let mut manager = SeccompManager::new();

        assert!(manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string()
            )
            .is_ok());
        assert_eq!(manager.filter_count(), 1);
    }

    #[test]
    fn test_add_rule() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

        let rule = SeccompRule {
            syscall_number: 1,
            action: SeccompAction::Allow,
            arg_filters: Vec::new(),
        };

        let filter = manager.get_filter_mut("test").unwrap();
        filter.add_rule(rule);

        assert_eq!(filter.rule_count(), 1);
    }

    #[test]
    fn test_bpf_instruction() {
        let instr = BpfInstruction::ld_abs(4, 0);
        assert_eq!(instr.code, 0x20);
        assert_eq!(instr.k, 0);

        let instr = BpfInstruction::jeq(1, 0, 42);
        assert_eq!(instr.code, 0x15);
        assert_eq!(instr.jt, 1);
        assert_eq!(instr.k, 42);
    }

    #[test]
    fn test_seccomp_rule() {
        let rule = SeccompRule::new(1, SeccompAction::Allow);
        assert_eq!(rule.syscall, 1);
        assert_eq!(rule.action, SeccompAction::Allow);
        assert!(rule.args.is_empty());
    }

    #[test]
    fn test_seccomp_rule_with_args() {
        let mut rule = SeccompRule::new(1, SeccompAction::Allow);
        let arg_filter = SeccompArgFilter::new(0, SeccompCompareOp::Eq, 42);
        rule.add_arg_filter(arg_filter);
        assert_eq!(rule.args.len(), 1);
    }

    #[test]
    fn test_seccomp_compiler_creation() {
        let compiler = SeccompCompiler::new(SeccompAction::Kill);
        assert_eq!(compiler.rule_count(), 0);
        assert_eq!(compiler.default_action, SeccompAction::Kill);
        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

        let rule = SeccompRule {
            syscall_number: 1,
            action: SeccompAction::Allow,
            arg_filters: Vec::new(),
        };

        let filter = manager.get_filter_mut("test").unwrap();
        filter.add_rule(rule);

        let action = filter.filter_syscall(1, &[]);
        assert_eq!(action, SeccompAction::Allow);

        let action = filter.filter_syscall(2, &[]);
        assert_eq!(action, SeccompAction::KillProcess);
    }

    #[test]
    fn test_arg_filter() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

        let arg_filter = SeccompArgFilter {
            arg_num: 0,
            op: SeccompCmpOp::Equal,
            value: 100,
            value_mask: u64::MAX,
        };

        let rule = SeccompRule {
            syscall_number: 1,
            action: SeccompAction::Allow,
            arg_filters: vec![arg_filter],
        };

        let filter = manager.get_filter_mut("test").unwrap();
        filter.add_rule(rule);

        let action = filter.filter_syscall(1, &[100]);
        assert_eq!(action, SeccompAction::Allow);

        let action = filter.filter_syscall(1, &[200]);
        assert_eq!(action, SeccompAction::KillProcess);
    }

    #[test]
    fn test_set_active_filter() {
        let mut manager = SeccompManager::new();

        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();
        assert!(manager.set_active_filter("test".to_string()).is_ok());

        assert_eq!(manager.active_filter(), Some("test"));
    }

    #[test]
    fn test_strict_filter() {
        let mut manager = SeccompManager::new();

        assert!(manager
            .create_strict_filter("strict".to_string(), vec![1, 2, 3])
            .is_ok());

        let filter = manager.get_filter("strict").unwrap();
        assert_eq!(filter.default_action, SeccompAction::KillProcess);
        assert_eq!(filter.rule_count(), 3);
    }

    #[test]
    fn test_seccomp_compiler_add_rule() {
        let mut compiler = SeccompCompiler::new(SeccompAction::Kill);
        compiler.allow_syscall(1);
        assert_eq!(compiler.rule_count(), 1);
    }

    #[test]
    fn test_seccomp_compiler_compile() {
        let mut compiler = SeccompCompiler::new(SeccompAction::Kill);
        compiler.allow_syscall(1);
        let instructions = compiler.compile().unwrap();
        assert!(!instructions.is_empty());
        assert!(compiler.instruction_count() > 0);
    }
        manager
            .create_filter(
                "test".to_string(),
                SeccompAction::KillProcess,
                "x86_64".to_string(),
            )
            .unwrap();

    #[test]
    fn test_seccomp_compiler_compile_with_args() {
        let mut compiler = SeccompCompiler::new(SeccompAction::Kill);
        let mut rule = SeccompRule::new(1, SeccompAction::Allow);
        rule.add_arg_filter(SeccompArgFilter::new(0, SeccompCompareOp::Eq, 42));
        compiler.add_rule(rule);
        let instructions = compiler.compile().unwrap();
        assert!(!instructions.is_empty());
    }

    #[test]
    fn test_seccomp_compiler_clear() {
        let mut compiler = SeccompCompiler::new(SeccompAction::Kill);
        compiler.allow_syscall(1);
        compiler.allow_syscall(2);
        assert_eq!(compiler.rule_count(), 2);
        compiler.clear();
        assert_eq!(compiler.rule_count(), 0);
    }

    #[test]
    fn test_seccomp_statistics() {
        let mut compiler = SeccompCompiler::new(SeccompAction::Kill);
        compiler.allow_syscall(1);
        let mut rule = SeccompRule::new(2, SeccompAction::Allow);
        rule.add_arg_filter(SeccompArgFilter::new(0, SeccompCompareOp::Eq, 42));
        compiler.add_rule(rule);

        compiler.compile().unwrap();
        let stats = compiler.get_statistics();
        assert_eq!(stats.rule_count, 2);
        assert_eq!(stats.arg_filter_count, 1);
        assert!(stats.instruction_count > 0);
    }

    #[test]
    fn test_arg_filter_with_mask() {
        let filter = SeccompArgFilter::with_mask(0, SeccompCompareOp::MaskedEq, 0xFF, 0xFF00);
        assert_eq!(filter.arg_index, 0);
        assert_eq!(filter.mask, 0xFF00);
    }
}
