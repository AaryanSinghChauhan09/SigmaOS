//! SigmaOS I/O Request Stage and Queue Definitions

/// The 10-stage authoritative I/O lifecycle contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoStage {
    Submit,
    Validate,
    Enqueue,
    Execute,
    InterruptDma,
    Complete,
    Callback,
    Reclaim,
    Free,
    ErrorRecovery,
}

/// Request priority class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoPriority {
    Idle = 0,
    BestEffort = 1,
    RealTime = 2,
}

/// Operation type for I/O requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoOperationKind {
    Read,
    Write,
    Flush,
    Discard,
}

/// An I/O request descriptor tracking lifecycle transitions.
#[derive(Debug, Clone)]
pub struct IoRequest {
    pub id: u64,
    pub dev_id: u32,
    pub op: IoOperationKind,
    pub block_num: u64,
    pub length_bytes: usize,
    pub priority: IoPriority,
    pub stage: IoStage,
    pub error_code: Option<i32>,
}

impl IoRequest {
    pub fn new(
        id: u64,
        dev_id: u32,
        op: IoOperationKind,
        block_num: u64,
        length_bytes: usize,
        priority: IoPriority,
    ) -> Self {
        Self {
            id,
            dev_id,
            op,
            block_num,
            length_bytes,
            priority,
            stage: IoStage::Submit,
            error_code: None,
        }
    }

    pub fn advance(&mut self, next: IoStage) {
        self.stage = next;
    }

    pub fn fail(&mut self, err: i32) {
        self.error_code = Some(err);
        self.stage = IoStage::ErrorRecovery;
    }
}
