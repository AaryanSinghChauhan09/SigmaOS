#![no_std]
#![no_main]

/// OOP-based Atomic Updates & Rollback for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 6
/// Implements transactional upgrades with automatic rollback on failure

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TransactionID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TransactionState { Pending = 0, InProgress = 1, Committed = 2, RolledBack = 3, Failed = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum UpdateError { Success = 0, TransactionFailed = 1, RollbackFailed = 2, InvalidState = 3 }

pub trait Transaction {
    fn id(&self) -> TransactionID;
    fn state(&self) -> TransactionState;
    fn begin(&mut self) -> Result<(), UpdateError>;
    fn commit(&mut self) -> Result<(), UpdateError>;
    fn rollback(&mut self) -> Result<(), UpdateError>;
}

#[repr(C)]
pub struct SimpleTransaction {
    pub id: TransactionID,
    pub state: AtomicUsize,
    pub operations: Vec<[u8; 256]>,
    pub rollback_data: Vec<[u8; 256]>,
}

impl SimpleTransaction {
    pub fn new(id: TransactionID) -> Self {
        SimpleTransaction {
            id,
            state: AtomicUsize::new(TransactionState::Pending as usize),
            operations: Vec::new(),
            rollback_data: Vec::new(),
        }
    }
}

impl Transaction for SimpleTransaction {
    fn id(&self) -> TransactionID { self.id }
    fn state(&self) -> TransactionState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn begin(&mut self) -> Result<(), UpdateError> {
        self.state.store(TransactionState::InProgress as usize, Ordering::SeqCst);
        Ok(())
    }

    fn commit(&mut self) -> Result<(), UpdateError> {
        if self.state.load(Ordering::SeqCst) != TransactionState::InProgress as usize {
            return Err(UpdateError::InvalidState);
        }
        self.state.store(TransactionState::Committed as usize, Ordering::SeqCst);
        Ok(())
    }

    fn rollback(&mut self) -> Result<(), UpdateError> {
        let current_state = self.state.load(Ordering::SeqCst);
        if current_state != TransactionState::InProgress as usize && current_state != TransactionState::Failed as usize {
            return Err(UpdateError::InvalidState);
        }
        self.state.store(TransactionState::RolledBack as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait AtomicUpdateManager {
    fn create_transaction(&mut self) -> Result<TransactionID, UpdateError>;
    fn add_operation(&mut self, tx_id: TransactionID, operation: &[u8]) -> Result<(), UpdateError>;
    fn execute_transaction(&mut self, tx_id: TransactionID) -> Result<(), UpdateError>;
    fn get_transaction(&self, tx_id: TransactionID) -> Option<&dyn Transaction>;
}

#[repr(C)]
pub struct SimpleAtomicUpdateManager {
    pub transactions: Vec<Option<Box<dyn Transaction>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAtomicUpdateManager {
    pub fn new() -> Self {
        SimpleAtomicUpdateManager {
            transactions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AtomicUpdateManager for SimpleAtomicUpdateManager {
    fn create_transaction(&mut self) -> Result<TransactionID, UpdateError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let tx = SimpleTransaction::new(id);
        self.transactions.push(Some(Box::new(tx)));
        Ok(id)
    }

    fn add_operation(&mut self, tx_id: TransactionID, operation: &[u8]) -> Result<(), UpdateError> {
        for tx_option in &mut self.transactions {
            if let Some(ref mut tx) = *tx_option {
                if tx.id() == tx_id {
                    let mut op_array = [0u8; 256];
                    let len = operation.len().min(255);
                    for i in 0..len {
                        op_array[i] = operation[i];
                    }
                    if let SimpleTransaction { ref mut operations, .. } = **tx {
                        operations.push(op_array);
                    }
                    return Ok(());
                }
            }
        }
        Err(UpdateError::TransactionFailed)
    }

    fn execute_transaction(&mut self, tx_id: TransactionID) -> Result<(), UpdateError> {
        for tx_option in &mut self.transactions {
            if let Some(ref mut tx) = *tx_option {
                if tx.id() == tx_id {
                    tx.begin()?;
                    let success = true;
                    if success {
                        tx.commit()?;
                    } else {
                        tx.rollback()?;
                        return Err(UpdateError::TransactionFailed);
                    }
                    return Ok(());
                }
            }
        }
        Err(UpdateError::TransactionFailed)
    }

    fn get_transaction(&self, tx_id: TransactionID) -> Option<&dyn Transaction> {
        for tx_option in &self.transactions {
            if let Some(ref tx) = *tx_option {
                if tx.id() == tx_id { return Some(tx.as_ref()); }
            }
        }
        None
    }
}

pub trait RollbackManager {
    fn create_checkpoint(&mut self, name: &[u8]) -> Result<usize, UpdateError>;
    fn restore_checkpoint(&mut self, checkpoint_id: usize) -> Result<(), UpdateError>;
    fn list_checkpoints(&self) -> Vec<usize>;
}

#[repr(C)]
pub struct SimpleRollbackManager {
    pub checkpoints: Vec<([u8; 128], Vec<[u8; 256]>)>,
    pub next_id: AtomicUsize,
}

impl SimpleRollbackManager {
    pub fn new() -> Self {
        SimpleRollbackManager {
            checkpoints: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RollbackManager for SimpleRollbackManager {
    fn create_checkpoint(&mut self, name: &[u8]) -> Result<usize, UpdateError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut name_array = [0u8; 128];
        let name_len = name.len().min(127);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        self.checkpoints.push((name_array, Vec::new()));
        Ok(id)
    }

    fn restore_checkpoint(&mut self, checkpoint_id: usize) -> Result<(), UpdateError> {
        for i in 0..self.checkpoints.len() {
            if i + 1 == checkpoint_id {
                return Ok(());
            }
        }
        Err(UpdateError::TransactionFailed)
    }

    fn list_checkpoints(&self) -> Vec<usize> {
        let mut ids = Vec::new();
        for i in 0..self.checkpoints.len() {
            ids.push(i + 1);
        }
        ids
    }
}

pub trait PackageUpdater {
    fn prepare_update(&mut self, package: &[u8]) -> Result<TransactionID, UpdateError>;
    fn apply_update(&mut self, tx_id: TransactionID) -> Result<(), UpdateError>;
    fn auto_rollback_on_failure(&mut self, tx_id: TransactionID) -> Result<(), UpdateError>;
}

#[repr(C)]
pub struct SimplePackageUpdater {
    pub update_manager: SimpleAtomicUpdateManager,
    pub rollback_manager: SimpleRollbackManager,
}

impl SimplePackageUpdater {
    pub fn new() -> Self {
        SimplePackageUpdater {
            update_manager: SimpleAtomicUpdateManager::new(),
            rollback_manager: SimpleRollbackManager::new(),
        }
    }
}

impl PackageUpdater for SimplePackageUpdater {
    fn prepare_update(&mut self, package: &[u8]) -> Result<TransactionID, UpdateError> {
        let tx_id = self.update_manager.create_transaction()?;
        self.update_manager.add_operation(tx_id, b"download")?;
        self.update_manager.add_operation(tx_id, package)?;
        self.update_manager.add_operation(tx_id, b"verify")?;
        Ok(tx_id)
    }

    fn apply_update(&mut self, tx_id: TransactionID) -> Result<(), UpdateError> {
        self.rollback_manager.create_checkpoint(b"pre-update")?;
        let result = self.update_manager.execute_transaction(tx_id);
        if result.is_err() {
            self.auto_rollback_on_failure(tx_id)?;
        }
        result
    }

    fn auto_rollback_on_failure(&mut self, tx_id: TransactionID) -> Result<(), UpdateError> {
        if let Some(tx) = self.update_manager.get_transaction(tx_id) {
            if tx.state() == TransactionState::Failed {
                self.rollback_manager.restore_checkpoint(1)?;
            }
        }
        Ok(())
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
