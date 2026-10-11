#[derive(Debug, Clone, Copy)]
pub struct SignalSet(pub u64);

impl SignalSet {
    pub const SIGKILL: u64 = 1 << 8;
    pub const SIGSTOP: u64 = 1 << 18;

    pub fn new() -> Self {
        SignalSet(0)
    }

    pub fn add(&mut self, sig: u32) {
        self.0 |= 1 << (sig - 1);
    }

    pub fn contains(&self, sig: u32) -> bool {
        (self.0 & (1 << (sig - 1))) != 0
    }
}

pub struct SignalState {
    pub pending: SignalSet,
    pub blocked: SignalSet,
}

impl SignalState {
    pub fn new() -> Self {
        Self {
            pending: SignalSet::new(),
            blocked: SignalSet::new(),
        }
    }

    pub fn block_signal(&mut self, sig: u32) {
        // SIGKILL and SIGSTOP cannot be blocked
        if (1 << (sig - 1)) == SignalSet::SIGKILL || (1 << (sig - 1)) == SignalSet::SIGSTOP {
            return;
        }
        self.blocked.add(sig);
    }
}

#[test]
fn test_sigkill_unblockable_contract() {
    let mut sig_state = SignalState::new();

    // Attempt to block SIGKILL (signal 9)
    sig_state.block_signal(9);
    assert!(!sig_state.blocked.contains(9));

    // Attempt to block SIGSTOP (signal 19)
    sig_state.block_signal(19);
    assert!(!sig_state.blocked.contains(19));

    // Block standard signal SIGUSR1 (signal 10)
    sig_state.block_signal(10);
    assert!(sig_state.blocked.contains(10));
}
