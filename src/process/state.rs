//! Process lifecycle state machine implementation.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessState {
    New,
    Ready,
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Reaped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStateError {
    InvalidStateTransition {
        from: ProcessState,
        to: ProcessState,
    },
    ProcessAlreadyReaped,
}

impl ProcessState {
    /// Validates if a state transition from `self` to `target` is allowed.
    pub fn can_transition_to(&self, target: ProcessState) -> bool {
        match (self, target) {
            // NEW transitions
            (ProcessState::New, ProcessState::Ready) => true,
            // Cannot move directly from NEW to ZOMBIE without cleanup
            (ProcessState::New, ProcessState::Zombie) => false,

            // READY transitions
            (ProcessState::Ready, ProcessState::Running) => true,
            (ProcessState::Ready, ProcessState::Zombie) => true,

            // RUNNING transitions
            (ProcessState::Running, ProcessState::Ready) => true,
            (ProcessState::Running, ProcessState::Sleeping) => true,
            (ProcessState::Running, ProcessState::Stopped) => true,
            (ProcessState::Running, ProcessState::Zombie) => true,

            // SLEEPING transitions
            (ProcessState::Sleeping, ProcessState::Ready) => true,
            (ProcessState::Sleeping, ProcessState::Stopped) => true,
            (ProcessState::Sleeping, ProcessState::Zombie) => true,

            // STOPPED transitions
            (ProcessState::Stopped, ProcessState::Ready) => true,
            (ProcessState::Stopped, ProcessState::Zombie) => true,

            // ZOMBIE transitions
            (ProcessState::Zombie, ProcessState::Reaped) => true,

            // REAPED is a terminal state
            (ProcessState::Reaped, _) => false,

            // Reflexive same-state transition
            (s, t) if *s == t => true,

            _ => false,
        }
    }
}

/// Tracks process state lifecycle and transition logs.
#[derive(Debug, Clone)]
pub struct ProcessLifecycleTracker {
    current_state: ProcessState,
    exit_code: Option<i32>,
}

impl ProcessLifecycleTracker {
    pub fn new() -> Self {
        Self {
            current_state: ProcessState::New,
            exit_code: None,
        }
    }

    pub fn state(&self) -> ProcessState {
        self.current_state
    }

    pub fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn transition_to(&mut self, next: ProcessState) -> Result<(), ProcessStateError> {
        if !self.current_state.can_transition_to(next) {
            return Err(ProcessStateError::InvalidStateTransition {
                from: self.current_state,
                to: next,
            });
        }
        self.current_state = next;
        Ok(())
    }

    pub fn exit(&mut self, code: i32) -> Result<(), ProcessStateError> {
        self.transition_to(ProcessState::Zombie)?;
        self.exit_code = Some(code);
        Ok(())
    }

    pub fn reap(&mut self) -> Result<i32, ProcessStateError> {
        if self.current_state != ProcessState::Zombie {
            return Err(ProcessStateError::InvalidStateTransition {
                from: self.current_state,
                to: ProcessState::Reaped,
            });
        }
        let code = self.exit_code.unwrap_or(0);
        self.transition_to(ProcessState::Reaped)?;
        Ok(code)
    }
}

impl Default for ProcessLifecycleTracker {
    fn default() -> Self {
        Self::new()
    }
}
