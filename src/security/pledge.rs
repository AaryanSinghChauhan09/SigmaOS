#![no_std]

bitflags::bitflags! {
    pub struct Promise: u32 {
        const STDIO = 0b0000_0001;
        const RPATH = 0b0000_0010;
        const WPATH = 0b0000_0100;
        const CPATH = 0b0000_1000;
        const DPATH = 0b0001_0000;
        const INET  = 0b0010_0000;
        const UNIX  = 0b0100_0000;
        const PROC  = 0b1000_0000;
    }
}

pub struct ProcessPledge {
    current_promises: Promise,
    exec_promises: Option<Promise>,
}

impl ProcessPledge {
    pub const fn new() -> Self {
        ProcessPledge {
            // Initially, no restrictions (or could default to all permissions depending on design)
            current_promises: Promise::all(),
            exec_promises: None,
        }
    }

    pub fn pledge(&mut self, promises: Promise, exec_promises: Option<Promise>) -> Result<(), &'static str> {
        // Can only reduce privileges, never increase
        if !self.current_promises.contains(promises) {
            return Err("Cannot increase privileges via pledge");
        }
        
        self.current_promises = promises;
        self.exec_promises = exec_promises;
        
        Ok(())
    }

    pub fn check_permission(&self, required: Promise) -> bool {
        self.current_promises.contains(required)
    }
}
