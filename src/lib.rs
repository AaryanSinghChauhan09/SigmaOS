#![no_std]

pub mod kernel {
    pub mod scheduler;
}

pub mod memory {
    pub mod buddy_allocator;
}

pub mod ipc {
    pub mod pipes;
}

pub mod security {
    pub mod pledge;
}

pub mod net {
    pub mod socket;
}
