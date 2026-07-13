/// SigmaOS: =============================================================================
/// Migrated from C/C++ to Rust â€” no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// â”€â”€â”€ Kernel Primitive Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// â”€â”€â”€ Module: Sigma::sigma_oop â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// SovereignObject â€” hardware-compatible struct.
/// Base class for all SigmaOS objects following OOP principles
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SovereignObject {
    pub id: SigmaU32,
    pub type_id: SigmaU32,
    pub ref_count: SigmaU32,
}

impl SovereignObject {
    pub const fn new(id: SigmaU32, type_id: SigmaU32) -> Self {
        Self {
            id,
            type_id,
            ref_count: 1,
        }
    }
    
    pub fn increment_ref(&mut self) {
        self.ref_count += 1;
    }
    
    pub fn decrement_ref(&mut self) -> bool {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
        self.ref_count == 0
    }
}

// OOP Trait System

/// Base trait for all SigmaOS objects (polymorphism)
pub trait SigmaObject {
    fn get_id(&self) -> SigmaU32;
    fn get_type_id(&self) -> SigmaU32;
}

/// Initializable interface
pub trait Initializable {
    fn initialize(&mut self) -> Result<(), SigmaError>;
}

/// Disposable interface for cleanup
pub trait Disposable {
    fn dispose(&mut self);
}

// Error Handling

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SigmaError {
    Success,
    InvalidParameter,
    OutOfMemory,
    NotImplemented,
    PermissionDenied,
    ResourceNotFound,
}

pub type SigmaResult<T> = Result<T, SigmaError>;

// Smart Pointer (Reference Counting)

/// SigmaPtr â€” smart pointer with reference counting
pub struct SigmaPtr<T> {
    ptr: *mut T,
}

impl<T> SigmaPtr<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }
    
    pub fn as_ref(&self) -> Option<&T> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe { Some(&*self.ptr) }
        }
    }
    
    pub fn as_mut(&mut self) -> Option<&mut T> {
        if self.ptr.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.ptr) }
        }
    }
}

// Factory Pattern

/// ObjectFactory trait for creating objects
pub trait ObjectFactory<T> {
    fn create(&self) -> SigmaResult<T>;
    fn destroy(&self, obj: T);
}

// Observer Pattern

/// Observer trait for event notification
pub trait Observer {
    fn notify(&self, event: SigmaEvent);
}

/// Observable trait for subjects that can be observed
pub trait Observable {
    fn attach(&mut self, observer: *mut dyn Observer);
    fn detach(&mut self, observer: *mut dyn Observer);
    fn notify_observers(&self, event: SigmaEvent);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigmaEvent {
    pub event_type: SigmaU32,
    pub data: SigmaU64,
}

// Strategy Pattern

/// Strategy trait for interchangeable algorithms
pub trait Strategy {
    fn execute(&self, context: &mut dyn Context) -> SigmaResult<()>;
}

pub trait Context {
    fn get_state(&self) -> SigmaU64;
    fn set_state(&mut self, state: SigmaU64);
}



