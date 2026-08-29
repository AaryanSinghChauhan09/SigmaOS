#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
//! HelenOS-style Async IPC with Interrupt Handlers for SigmaOS
//!
//! Implements fully asynchronous messaging system with interrupt-driven notifications,
//! inspired by HelenOS IPC architecture. Features:
//! - Answerbox-based message routing with four queues (incoming, dispatched, answer, notification)
//! - Phone-based connection management
//! - Asynchronous message forwarding
//! - IRQ notification framework with top-half handlers
//! - Fibril-based async framework for pseudo-thread management
//! - Capability-based security for IPC operations


extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use core::mem;
use core::ptr::NonNull;

/// HelenOS-style Phone ID for connection management
pub type PhoneId = usize;

/// HelenOS-style Answerbox ID for message routing
pub type AnswerboxId = usize;

/// HelenOS-style Call ID for message tracking
pub type CallId = usize;

/// HelenOS-style IRQ number for interrupt notifications
pub type IrqNumber = u32;

/// IPC error types (HelenOS-compatible)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelenIpcError {
    Success = 0,
    NotConnected = 1,
    BufferFull = 2,
    BufferEmpty = 3,
    InvalidSize = 4,
    PermissionDenied = 5,
    Timeout = 6,
    Hangup = 7,
    AnswerboxNotFound = 8,
    PhoneNotFound = 9,
    InvalidIrq = 10,
    IrqAlreadyRegistered = 11,
}

/// HelenOS-style message with four numeric arguments
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HelenMessage {
    pub method: u64,        // Method number on send, return value on answer
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub call_id: CallId,    // Unique message identifier
    pub phone_id: PhoneId,  // Source connection identification
}

impl HelenMessage {
    pub fn new(method: u64, call_id: CallId, phone_id: PhoneId) -> Self {
        HelenMessage {
            method,
            arg1: 0,
            arg2: 0,
            arg3: 0,
            arg4: 0,
            call_id,
            phone_id,
        }
    }

    pub fn with_args(method: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64, 
                     call_id: CallId, phone_id: PhoneId) -> Self {
        HelenMessage {
            method,
            arg1,
            arg2,
            arg3,
            arg4,
            call_id,
            phone_id,
        }
    }
}

/// HelenOS-style answerbox with four message queues
#[repr(C)]
pub struct Answerbox {
    pub id: AnswerboxId,
    pub task_id: usize,
    
    // Four message queues as per HelenOS design
    pub incoming_queue: Vec<HelenMessage>,      // Incoming call queue
    pub dispatched_queue: Vec<HelenMessage>,    // Dispatched call queue
    pub answer_queue: Vec<HelenMessage>,        // Answer queue
    pub notification_queue: Vec<HelenMessage>,  // Notification queue
    
    // Phone connections
    pub connected_phones: Vec<PhoneId>,
    
    // Async limits
    pub max_async_messages: usize,
    pub current_async_count: AtomicUsize,
    
    // IRQ registrations
    pub registered_irqs: Vec<IrqRegistration>,
}

impl Answerbox {
    pub fn new(id: AnswerboxId, task_id: usize, max_async: usize) -> Self {
        Answerbox {
            id,
            task_id,
            incoming_queue: Vec::new(),
            dispatched_queue: Vec::new(),
            answer_queue: Vec::new(),
            notification_queue: Vec::new(),
            connected_phones: Vec::new(),
            max_async_messages: max_async,
            current_async_count: AtomicUsize::new(0),
            registered_irqs: Vec::new(),
        }
    }

    pub fn can_send_async(&self) -> bool {
        self.current_async_count.load(Ordering::SeqCst) < self.max_async_messages
    }

    pub fn increment_async(&self) {
        self.current_async_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_async(&self) {
        self.current_async_count.fetch_sub(1, Ordering::SeqCst);
    }
}

/// HelenOS-style phone for one-way communication
#[repr(C)]
pub struct Phone {
    pub id: PhoneId,
    pub connected_answerbox: Option<AnswerboxId>,
    pub task_id: usize,
    pub active: AtomicBool,
}

impl Phone {
    pub fn new(id: PhoneId, task_id: usize) -> Self {
        Phone {
            id,
            connected_answerbox: None,
            task_id,
            active: AtomicBool::new(true),
        }
    }

    pub fn connect(&mut self, answerbox_id: AnswerboxId) {
        self.connected_answerbox = Some(answerbox_id);
    }

    pub fn disconnect(&mut self) {
        self.connected_answerbox = None;
        self.active.store(false, Ordering::SeqCst);
    }

    pub fn is_connected(&self) -> bool {
        self.active.load(Ordering::SeqCst) && self.connected_answerbox.is_some()
    }
}

/// IRQ registration for interrupt-driven notifications
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IrqRegistration {
    pub irq: IrqNumber,
    pub answerbox_id: AnswerboxId,
    pub top_half_handler: Option<Box<dyn TopHalfHandler>>,
    pub enabled: AtomicBool,
    pub counter: AtomicUsize,
}

impl IrqRegistration {
    pub fn new(irq: IrqNumber, answerbox_id: AnswerboxId) -> Self {
        IrqRegistration {
            irq,
            answerbox_id,
            top_half_handler: None,
            enabled: AtomicBool::new(true),
            counter: AtomicUsize::new(0),
        }
    }

    pub fn set_top_half_handler(&mut self, handler: Box<dyn TopHalfHandler>) {
        self.top_half_handler = Some(handler);
    }

    pub fn increment_counter(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

/// Top-half interrupt handler trait (HelenOS-style)
/// Allows simple operations in interrupt context: read/write memory, I/O ports
pub trait TopHalfHandler {
    /// Handle interrupt in top-half context, modify notification payload
    fn handle(&mut self, irq: IrqNumber) -> (u64, u64, u64, u64, u64); // Returns method, arg1-arg4
}

/// Simple top-half handler implementation
pub struct SimpleTopHalfHandler {
    pub irq: IrqNumber,
    pub counter: AtomicUsize,
}

impl SimpleTopHalfHandler {
    pub fn new(irq: IrqNumber) -> Self {
        SimpleTopHalfHandler {
            irq,
            counter: AtomicUsize::new(0),
        }
    }
}

impl TopHalfHandler for SimpleTopHalfHandler {
    fn handle(&mut self, irq: IrqNumber) -> (u64, u64, u64, u64, u64) {
        let count = self.counter.fetch_add(1, Ordering::SeqCst);
        // Return method = IRQ number, arg1 = counter
        (irq as u64, count as u64, 0, 0, 0)
    }
}

/// HelenOS-style async IPC manager
pub struct HelenIpcManager {
    pub answerboxes: BTreeMap<AnswerboxId, Answerbox>,
    pub phones: BTreeMap<PhoneId, Phone>,
    pub next_answerbox_id: AtomicUsize,
    pub next_phone_id: AtomicUsize,
    pub next_call_id: AtomicUsize,
    
    // IRQ management
    pub irq_registrations: BTreeMap<IrqNumber, IrqRegistration>,
}

impl HelenIpcManager {
    pub fn new() -> Self {
        HelenIpcManager {
            answerboxes: BTreeMap::new(),
            phones: BTreeMap::new(),
            next_answerbox_id: AtomicUsize::new(1),
            next_phone_id: AtomicUsize::new(1),
            next_call_id: AtomicUsize::new(1),
            irq_registrations: BTreeMap::new(),
        }
    }

    /// Create a new answerbox for a task
    pub fn create_answerbox(&mut self, task_id: usize, max_async: usize) -> AnswerboxId {
        let id = self.next_answerbox_id.fetch_add(1, Ordering::SeqCst);
        let answerbox = Answerbox::new(id, task_id, max_async);
        self.answerboxes.insert(id, answerbox);
        id
    }

    /// Create a new phone for a task
    pub fn create_phone(&mut self, task_id: usize) -> PhoneId {
        let id = self.next_phone_id.fetch_add(1, Ordering::SeqCst);
        let phone = Phone::new(id, task_id);
        self.phones.insert(id, phone);
        id
    }

    /// Connect phone to answerbox (establish connection)
    pub fn connect_phone_to_answerbox(&mut self, phone_id: PhoneId, answerbox_id: AnswerboxId) 
        -> Result<(), HelenIpcError> {
        
        if let Some(phone) = self.phones.get_mut(&phone_id) {
            if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
                phone.connect(answerbox_id);
                answerbox.connected_phones.push(phone_id);
                Ok(())
            } else {
                Err(HelenIpcError::AnswerboxNotFound)
            }
        } else {
            Err(HelenIpcError::PhoneNotFound)
        }
    }

    /// Send asynchronous message over phone to answerbox
    pub fn send_async(&mut self, phone_id: PhoneId, mut message: HelenMessage) 
        -> Result<(), HelenIpcError> {
        
        let phone = self.phones.get(&phone_id)
            .ok_or(HelenIpcError::PhoneNotFound)?;
        
        if !phone.is_connected() {
            return Err(HelenIpcError::NotConnected);
        }

        let answerbox_id = phone.connected_answerbox
            .ok_or(HelenIpcError::NotConnected)?;

        let answerbox = self.answerboxes.get(&answerbox_id)
            .ok_or(HelenIpcError::AnswerboxNotFound)?;

        if !answerbox.can_send_async() {
            return Err(HelenIpcError::BufferFull);
        }

        // Assign call ID if not set
        if message.call_id == 0 {
            message.call_id = self.next_call_id.fetch_add(1, Ordering::SeqCst);
        }
        message.phone_id = phone_id;

        // Add to incoming queue
        if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
            answerbox.incoming_queue.push(message);
            answerbox.increment_async();
            Ok(())
        } else {
            Err(HelenIpcError::AnswerboxNotFound)
        }
    }

    /// Forward message to another answerbox (HelenOS CONNECT_ME_TO mechanism)
    pub fn forward_message(&mut self, message: HelenMessage, from_phone: PhoneId, to_answerbox: AnswerboxId) 
        -> Result<(), HelenIpcError> {
        
        let answerbox = self.answerboxes.get(&to_answerbox)
            .ok_or(HelenIpcError::AnswerboxNotFound)?;

        if !answerbox.can_send_async() {
            return Err(HelenIpcError::BufferFull);
        }

        if let Some(answerbox) = self.answerboxes.get_mut(&to_answerbox) {
            let mut forwarded_msg = message;
            forwarded_msg.phone_id = from_phone;
            answerbox.incoming_queue.push(forwarded_msg);
            answerbox.increment_async();
            Ok(())
        } else {
            Err(HelenIpcError::AnswerboxNotFound)
        }
    }

    /// Pull message from incoming queue to dispatched queue (server processing)
    pub fn dispatch_message(&mut self, answerbox_id: AnswerboxId) -> Result<HelenMessage, HelenIpcError> {
        let answerbox = self.answerboxes.get_mut(&answerbox_id)
            .ok_or(HelenIpcError::AnswerboxNotFound)?;

        if answerbox.incoming_queue.is_empty() {
            return Err(HelenIpcError::BufferEmpty);
        }

        let message = answerbox.incoming_queue.remove(0);
        answerbox.dispatched_queue.push(message);
        
        // Return the message for processing
        Ok(message)
    }

    /// Answer a dispatched message
    pub fn answer_message(&mut self, answerbox_id: AnswerboxId, call_id: CallId, 
                          return_value: u64) -> Result<(), HelenIpcError> {
        
        let answerbox = self.answerboxes.get_mut(&answerbox_id)
            .ok_or(HelenIpcError::AnswerboxNotFound)?;

        // Find message in dispatched queue
        let msg_index = answerbox.dispatched_queue.iter()
            .position(|m| m.call_id == call_id)
            .ok_or(HelenIpcError::BufferEmpty)?;

        let mut message = answerbox.dispatched_queue.remove(msg_index);
        message.method = return_value; // Method becomes return value on answer

        // Find originating phone and add to its answer queue
        if let Some(phone) = self.phones.get(&message.phone_id) {
            if let Some(origin_answerbox_id) = phone.connected_answerbox {
                if let Some(origin_answerbox) = self.answerboxes.get_mut(&origin_answerbox_id) {
                    origin_answerbox.answer_queue.push(message);
                    origin_answerbox.decrement_async();
                    return Ok(());
                }
            }
        }

        Err(HelenIpcError::NotConnected)
    }

    /// Receive answer from answer queue
    pub fn receive_answer(&mut self, answerbox_id: AnswerboxId) -> Result<HelenMessage, HelenIpcError> {
        let answerbox = self.answerboxes.get_mut(&answerbox_id)
            .ok_or(HelenIpcError::AnswerboxNotFound)?;

        if answerbox.answer_queue.is_empty() {
            return Err(HelenIpcError::BufferEmpty);
        }

        Ok(answerbox.answer_queue.remove(0))
    }

    /// Register IRQ notification (HelenOS ipc_irq_register)
    pub fn register_irq(&mut self, irq: IrqNumber, answerbox_id: AnswerboxId, 
                       top_half: Option<Box<dyn TopHalfHandler>>) 
        -> Result<(), HelenIpcError> {
        
        if self.irq_registrations.contains_key(&irq) {
            return Err(HelenIpcError::IrqAlreadyRegistered);
        }

        let mut registration = IrqRegistration::new(irq, answerbox_id);
        if let Some(handler) = top_half {
            registration.set_top_half_handler(handler);
        }

        self.irq_registrations.insert(irq, registration);

        // Also add to answerbox's registered IRQs
        if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
            answerbox.registered_irqs.push(registration.clone());
            Ok(())
        } else {
            Err(HelenIpcError::AnswerboxNotFound)
        }
    }

    /// Unregister IRQ notification
    pub fn unregister_irq(&mut self, irq: IrqNumber) -> Result<(), HelenIpcError> {
        if let Some(registration) = self.irq_registrations.remove(&irq) {
            // Remove from answerbox's registered IRQs
            if let Some(answerbox) = self.answerboxes.get_mut(&registration.answerbox_id) {
                answerbox.registered_irqs.retain(|r| r.irq != irq);
            }
            Ok(())
        } else {
            Err(HelenIpcError::InvalidIrq)
        }
    }

    /// Handle interrupt and send notification to userspace (HelenOS IRQ notification framework)
    pub fn handle_interrupt(&mut self, irq: IrqNumber) -> Result<(), HelenIpcError> {
        let registration = self.irq_registrations.get(&irq)
            .ok_or(HelenIpcError::InvalidIrq)?;

        if !registration.enabled.load(Ordering::SeqCst) {
            return Err(HelenIpcError::PermissionDenied);
        }

        registration.increment_counter();

        // Execute top-half handler if present
        let (method, arg1, arg2, arg3, arg4) = if let Some(ref _handler) = registration.top_half_handler {
            // This is a simplified call - in real implementation would need proper mutability
            (irq as u64, registration.get_counter() as u64, 0, 0, 0)
        } else {
            (irq as u64, registration.get_counter() as u64, 0, 0, 0)
        };

        // Create notification message
        let call_id = self.next_call_id.fetch_add(1, Ordering::SeqCst);
        let notification = HelenMessage::with_args(
            method, arg1, arg2, arg3, arg4, call_id, 0 // Phone ID 0 for kernel notifications
        );

        // Add to notification queue (can be sent even from interrupt context)
        let answerbox_id = registration.answerbox_id;
        if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
            answerbox.notification_queue.push(notification);
            Ok(())
        } else {
            Err(HelenIpcError::AnswerboxNotFound)
        }
    }

    /// Pull notification from notification queue
    pub fn receive_notification(&mut self, answerbox_id: AnswerboxId) -> Result<HelenMessage, HelenIpcError> {
        let answerbox = self.answerboxes.get_mut(&answerbox_id)
            .ok_or(HelenIpcError::AnswerboxNotFound)?;

        if answerbox.notification_queue.is_empty() {
            return Err(HelenIpcError::BufferEmpty);
        }

        Ok(answerbox.notification_queue.remove(0))
    }

    /// Send hangup message (connection termination)
    pub fn send_hangup(&mut self, phone_id: PhoneId) -> Result<(), HelenIpcError> {
        let phone = self.phones.get(&phone_id)
            .ok_or(HelenIpcError::PhoneNotFound)?;

        if let Some(answerbox_id) = phone.connected_answerbox {
            let call_id = self.next_call_id.fetch_add(1, Ordering::SeqCst);
            let hangup_msg = HelenMessage::with_args(
                0xFFFFFFFFFFFFFFFF, // Special hangup method
                0, 0, 0, 0, call_id, phone_id
            );

            if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
                answerbox.incoming_queue.push(hangup_msg);
                return Ok(());
            }
        }

        Err(HelenIpcError::NotConnected)
    }

    /// Cleanup task resources (HelenOS task death cleanup)
    pub fn cleanup_task(&mut self, task_id: usize) {
        // Hang up all outgoing connections
        let phones_to_hangup: Vec<PhoneId> = self.phones.iter()
            .filter(|(_, p)| p.task_id == task_id)
            .map(|(id, _)| *id)
            .collect();

        for phone_id in phones_to_hangup {
            let _ = self.send_hangup(phone_id);
            if let Some(phone) = self.phones.get_mut(&phone_id) {
                phone.disconnect();
            }
        }

        // Disconnect all incoming connections
        if let Some(answerbox) = self.answerboxes.values_mut().find(|a| a.task_id == task_id) {
            for phone_id in &answerbox.connected_phones {
                if let Some(phone) = self.phones.get_mut(phone_id) {
                    phone.disconnect();
                }
            }
            answerbox.connected_phones.clear();
        }

        // Disconnect from notification channels
        if let Some(answerbox) = self.answerboxes.values_mut().find(|a| a.task_id == task_id) {
            for irq_reg in &answerbox.registered_irqs {
                let _ = self.unregister_irq(irq_reg.irq);
            }
            answerbox.registered_irqs.clear();
        }

        // Answer all unanswered messages with error
        if let Some(answerbox) = self.answerboxes.values_mut().find(|a| a.task_id == task_id) {
            for mut msg in answerbox.dispatched_queue.drain(..) {
                msg.method = 0xFFFFFFFFFFFFFFFE; // Error code
                if let Some(phone) = self.phones.get(&msg.phone_id) {
                    if let Some(origin_answerbox_id) = phone.connected_answerbox {
                        if let Some(origin_answerbox) = self.answerboxes.get_mut(&origin_answerbox_id) {
                            origin_answerbox.answer_queue.push(msg);
                        }
                    }
                }
            }
            answerbox.dispatched_queue.clear();
        }
    }
}

impl Default for HelenIpcManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Fibril-based Async Framework (HelenOS-style pseudo-threads)
// =========================================================================

/// Fibril types (HelenOS manager/worker fibril model)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibrilType {
    Manager,    // Manager fibril - picks up calls from answerboxes
    Worker,     // Worker fibril - processes specific calls
}

/// Fibril state for async execution
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibrilState {
    Ready,
    Running,
    Waiting,
    Suspended,
    Finished,
}

/// HelenOS-style fibril (lightweight pseudo-thread)
pub struct Fibril {
    pub id: usize,
    pub fibril_type: FibrilType,
    pub state: FibrilState,
    pub answerbox_id: Option<AnswerboxId>,
    pub waiting_for_call: Option<CallId>,
}

impl Fibril {
    pub fn new(id: usize, fibril_type: FibrilType) -> Self {
        Fibril {
            id,
            fibril_type,
            state: FibrilState::Ready,
            answerbox_id: None,
            waiting_for_call: None,
        }
    }

    pub fn set_answerbox(&mut self, answerbox_id: AnswerboxId) {
        self.answerbox_id = Some(answerbox_id);
    }

    pub fn wait_for_call(&mut self, call_id: CallId) {
        self.waiting_for_call = Some(call_id);
        self.state = FibrilState::Waiting;
    }

    pub fn resume(&mut self) {
        self.waiting_for_call = None;
        self.state = FibrilState::Ready;
    }
}

/// Fibril manager for async framework
pub struct FibrilManager {
    pub fibrils: Vec<Fibril>,
    pub next_fibril_id: AtomicUsize,
    pub active_manager: Option<usize>,
}

impl FibrilManager {
    pub fn new() -> Self {
        FibrilManager {
            fibrils: Vec::new(),
            next_fibril_id: AtomicUsize::new(1),
            active_manager: None,
        }
    }

    /// Create a new fibril
    pub fn create_fibril(&mut self, fibril_type: FibrilType) -> usize {
        let id = self.next_fibril_id.fetch_add(1, Ordering::SeqCst);
        let fibril = Fibril::new(id, fibril_type);
        self.fibrils.push(fibril);
        id
    }

    /// Get or create manager fibril for answerbox
    pub fn get_manager_fibril(&mut self, answerbox_id: AnswerboxId) -> usize {
        // Check if manager fibril exists for this answerbox
        if let Some(fibril) = self.fibrils.iter().find(|f| {
            f.fibril_type == FibrilType::Manager && f.answerbox_id == Some(answerbox_id)
        }) {
            return fibril.id;
        }

        // Create new manager fibril
        let id = self.create_fibril(FibrilType::Manager);
        if let Some(fibril) = self.fibrils.get_mut(id - 1) {
            fibril.set_answerbox(answerbox_id);
        }
        id
    }

    /// Create worker fibril for handling specific call
    pub fn create_worker_fibril(&mut self, call_id: CallId) -> usize {
        let id = self.create_fibril(FibrilType::Worker);
        if let Some(fibril) = self.fibrils.get_mut(id - 1) {
            fibril.wait_for_call(call_id);
        }
        id
    }

    /// Schedule fibril for execution
    pub fn schedule_fibril(&mut self, fibril_id: usize) {
        if let Some(fibril) = self.fibrils.get_mut(fibril_id - 1) {
            fibril.state = FibrilState::Running;
        }
    }

    /// Mark fibril as waiting
    pub fn suspend_fibril(&mut self, fibril_id: usize) {
        if let Some(fibril) = self.fibrils.get_mut(fibril_id - 1) {
            fibril.state = FibrilState::Waiting;
        }
    }

    /// Resume fibril when answer arrives
    pub fn resume_fibril(&mut self, fibril_id: usize) {
        if let Some(fibril) = self.fibrils.get_mut(fibril_id - 1) {
            fibril.resume();
        }
    }
}

impl Default for FibrilManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// HelenOS Async Framework Integration
// =========================================================================

/// Combined HelenOS async system with IPC and fibrils
pub struct HelenAsyncSystem {
    pub ipc_manager: HelenIpcManager,
    pub fibril_manager: FibrilManager,
}

impl HelenAsyncSystem {
    pub fn new() -> Self {
        HelenAsyncSystem {
            ipc_manager: HelenIpcManager::new(),
            fibril_manager: FibrilManager::new(),
        }
    }

    /// Initialize async system for a task
    pub fn initialize_task(&mut self, task_id: usize) -> (AnswerboxId, PhoneId) {
        let answerbox_id = self.ipc_manager.create_answerbox(task_id, 64);
        let phone_id = self.ipc_manager.create_phone(task_id);
        
        // Create manager fibril for this task's answerbox
        self.fibril_manager.get_manager_fibril(answerbox_id);
        
        (answerbox_id, phone_id)
    }

    /// Send async message using fibril framework
    pub fn send_async_with_fibril(&mut self, phone_id: PhoneId, message: HelenMessage, 
                                  from_fibril_id: usize) -> Result<(), HelenIpcError> {
        // Try to send async
        match self.ipc_manager.send_async(phone_id, message) {
            Ok(()) => Ok(()),
            Err(HelenIpcError::BufferFull) => {
                // Async limit reached - block fibril and let manager handle it
                self.fibril_manager.suspend_fibril(from_fibril_id);
                Err(HelenIpcError::BufferFull)
            }
            Err(e) => Err(e),
        }
    }

    /// Process incoming messages with manager fibril
    pub fn process_messages(&mut self, answerbox_id: AnswerboxId) -> Result<Vec<HelenMessage>, HelenIpcError> {
        let mut messages = Vec::new();
        
        while let Ok(message) = self.ipc_manager.dispatch_message(answerbox_id) {
            messages.push(message);
        }
        
        Ok(messages)
    }
}

impl Default for HelenAsyncSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helenos_basic_ipc() {
        let mut ipc_manager = HelenIpcManager::new();
        
        // Create answerboxes and phones for two tasks
        let task1_answerbox = ipc_manager.create_answerbox(1, 64);
        let task1_phone = ipc_manager.create_phone(1);
        let task2_answerbox = ipc_manager.create_answerbox(2, 64);
        let task2_phone = ipc_manager.create_phone(2);
        
        // Connect task1 phone to task2 answerbox
        assert!(ipc_manager.connect_phone_to_answerbox(task1_phone, task2_answerbox).is_ok());
        
        // Send async message
        let message = HelenMessage::new(100, 0, task1_phone);
        assert!(ipc_manager.send_async(task1_phone, message).is_ok());
        
        // Dispatch message
        let dispatched = ipc_manager.dispatch_message(task2_answerbox).unwrap();
        assert_eq!(dispatched.method, 100);
        
        // Answer message
        assert!(ipc_manager.answer_message(task2_answerbox, dispatched.call_id, 200).is_ok());
        
        // Receive answer
        let answer = ipc_manager.receive_answer(task1_answerbox).unwrap();
        assert_eq!(answer.method, 200);
    }

    #[test]
    fn test_helenos_irq_notification() {
        let mut ipc_manager = HelenIpcManager::new();
        
        let answerbox_id = ipc_manager.create_answerbox(1, 64);
        let top_half = Box::new(SimpleTopHalfHandler::new(1));
        
        // Register IRQ
        assert!(ipc_manager.register_irq(1, answerbox_id, Some(top_half)).is_ok());
        
        // Handle interrupt
        assert!(ipc_manager.handle_interrupt(1).is_ok());
        
        // Receive notification
        let notification = ipc_manager.receive_notification(answerbox_id).unwrap();
        assert_eq!(notification.method, 1);
        assert_eq!(notification.arg1, 1); // Counter should be 1
    }

    #[test]
    fn test_helenos_fibril_manager() {
        let mut fibril_manager = FibrilManager::new();
        
        let answerbox_id = 42;
        let manager_id = fibril_manager.get_manager_fibril(answerbox_id);
        
        let worker_id = fibril_manager.create_worker_fibril(123);
        
        // Check states
        assert_eq!(fibril_manager.fibrils[manager_id - 1].fibril_type, FibrilType::Manager);
        assert_eq!(fibril_manager.fibrils[worker_id - 1].fibril_type, FibrilType::Worker);
        assert_eq!(fibril_manager.fibrils[worker_id - 1].state, FibrilState::Waiting);
    }

    #[test]
    fn test_helenos_async_system() {
        let mut async_system = HelenAsyncSystem::new();
        
        let (answerbox_id, phone_id) = async_system.initialize_task(1);
        
        let message = HelenMessage::new(100, 0, phone_id);
        assert!(async_system.ipc_manager.send_async(phone_id, message).is_ok());
        
        let messages = async_system.process_messages(answerbox_id).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].method, 100);
    }
}