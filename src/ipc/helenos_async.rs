#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]

use std::vec::Vec;
use std::boxed::Box;
use std::collections::BTreeMap;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

pub type PhoneId = usize;
pub type AnswerboxId = usize;
pub type CallId = usize;
pub type IrqNumber = u32;

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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HelenMessage {
    pub method: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub call_id: CallId,
    pub phone_id: PhoneId,
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

    pub fn with_args(
        method: u64,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        call_id: CallId,
        phone_id: PhoneId,
    ) -> Self {
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

#[repr(C)]
pub struct Answerbox {
    pub id: AnswerboxId,
    pub task_id: usize,

    pub incoming_queue: Vec<HelenMessage>,
    pub dispatched_queue: Vec<HelenMessage>,
    pub answer_queue: Vec<HelenMessage>,
    pub notification_queue: Vec<HelenMessage>,

    pub connected_phones: Vec<PhoneId>,

    pub max_async_messages: usize,
    pub current_async_count: AtomicUsize,

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

#[repr(C)]
pub struct IrqRegistration {
    pub irq: IrqNumber,
    pub answerbox_id: AnswerboxId,
    pub top_half_handler: Option<Box<dyn TopHalfHandler>>,
    pub enabled: AtomicBool,
    pub counter: AtomicUsize,
}

impl Clone for IrqRegistration {
    fn clone(&self) -> Self {
        Self {
            irq: self.irq,
            answerbox_id: self.answerbox_id,
            top_half_handler: None,
            enabled: AtomicBool::new(self.enabled.load(Ordering::SeqCst)),
            counter: AtomicUsize::new(self.counter.load(Ordering::SeqCst)),
        }
    }
}

impl std::fmt::Debug for IrqRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IrqRegistration")
            .field("irq", &self.irq)
            .field("answerbox_id", &self.answerbox_id)
            .field("has_handler", &self.top_half_handler.is_some())
            .field("enabled", &self.enabled.load(Ordering::SeqCst))
            .field("counter", &self.counter.load(Ordering::SeqCst))
            .finish()
    }
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

pub trait TopHalfHandler {
    fn handle(&mut self, irq: IrqNumber) -> (u64, u64, u64, u64, u64);
}

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
        (irq as u64, count as u64, 0, 0, 0)
    }
}

pub struct HelenIpcManager {
    pub answerboxes: BTreeMap<AnswerboxId, Answerbox>,
    pub phones: BTreeMap<PhoneId, Phone>,
    pub next_answerbox_id: AtomicUsize,
    pub next_phone_id: AtomicUsize,
    pub next_call_id: AtomicUsize,
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

    pub fn create_answerbox(&mut self, task_id: usize, max_async: usize) -> AnswerboxId {
        let id = self.next_answerbox_id.fetch_add(1, Ordering::SeqCst);
        let answerbox = Answerbox::new(id, task_id, max_async);
        self.answerboxes.insert(id, answerbox);
        id
    }

    pub fn create_phone(&mut self, task_id: usize) -> PhoneId {
        let id = self.next_phone_id.fetch_add(1, Ordering::SeqCst);
        let phone = Phone::new(id, task_id);
        self.phones.insert(id, phone);
        id
    }

    pub fn connect_phone_to_answerbox(
        &mut self,
        phone_id: PhoneId,
        answerbox_id: AnswerboxId,
    ) -> Result<(), HelenIpcError> {
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

    pub fn send_async(
        &mut self,
        phone_id: PhoneId,
        mut message: HelenMessage,
    ) -> Result<(), HelenIpcError> {
        let phone = self.phones.get(&phone_id).ok_or(HelenIpcError::PhoneNotFound)?;

        if !phone.is_connected() {
            return Err(HelenIpcError::NotConnected);
        }

        let answerbox_id = phone.connected_answerbox.ok_or(HelenIpcError::NotConnected)?;

        let answerbox = self.answerboxes.get(&answerbox_id).ok_or(HelenIpcError::AnswerboxNotFound)?;

        if !answerbox.can_send_async() {
            return Err(HelenIpcError::BufferFull);
        }

        if message.call_id == 0 {
            message.call_id = self.next_call_id.fetch_add(1, Ordering::SeqCst);
        }
        message.phone_id = phone_id;

        if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
            answerbox.incoming_queue.push(message);
            answerbox.increment_async();
            Ok(())
        } else {
            Err(HelenIpcError::AnswerboxNotFound)
        }
    }

    pub fn dispatch_message(&mut self, answerbox_id: AnswerboxId) -> Result<HelenMessage, HelenIpcError> {
        let answerbox = self.answerboxes.get_mut(&answerbox_id).ok_or(HelenIpcError::AnswerboxNotFound)?;

        if answerbox.incoming_queue.is_empty() {
            return Err(HelenIpcError::BufferEmpty);
        }

        let message = answerbox.incoming_queue.remove(0);
        answerbox.dispatched_queue.push(message);

        Ok(message)
    }

    pub fn answer_message(
        &mut self,
        answerbox_id: AnswerboxId,
        call_id: CallId,
        return_value: u64,
    ) -> Result<(), HelenIpcError> {
        let answerbox = self.answerboxes.get_mut(&answerbox_id).ok_or(HelenIpcError::AnswerboxNotFound)?;

        let msg_index = answerbox.dispatched_queue.iter().position(|m| m.call_id == call_id).ok_or(HelenIpcError::BufferEmpty)?;

        let mut message = answerbox.dispatched_queue.remove(msg_index);
        message.method = return_value;

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

    pub fn receive_answer(&mut self, answerbox_id: AnswerboxId) -> Result<HelenMessage, HelenIpcError> {
        let answerbox = self.answerboxes.get_mut(&answerbox_id).ok_or(HelenIpcError::AnswerboxNotFound)?;

        if answerbox.answer_queue.is_empty() {
            return Err(HelenIpcError::BufferEmpty);
        }

        Ok(answerbox.answer_queue.remove(0))
    }

    pub fn register_irq(
        &mut self,
        irq: IrqNumber,
        answerbox_id: AnswerboxId,
        top_half: Option<Box<dyn TopHalfHandler>>,
    ) -> Result<(), HelenIpcError> {
        if self.irq_registrations.contains_key(&irq) {
            return Err(HelenIpcError::IrqAlreadyRegistered);
        }

        let mut registration = IrqRegistration::new(irq, answerbox_id);
        if let Some(handler) = top_half {
            registration.set_top_half_handler(handler);
        }

        let registration_clone = registration.clone();
        self.irq_registrations.insert(irq, registration);

        if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
            answerbox.registered_irqs.push(registration_clone);
            Ok(())
        } else {
            Err(HelenIpcError::AnswerboxNotFound)
        }
    }

    pub fn unregister_irq(&mut self, irq: IrqNumber) -> Result<(), HelenIpcError> {
        if let Some(registration) = self.irq_registrations.remove(&irq) {
            if let Some(answerbox) = self.answerboxes.get_mut(&registration.answerbox_id) {
                answerbox.registered_irqs.retain(|r| r.irq != irq);
            }
            Ok(())
        } else {
            Err(HelenIpcError::InvalidIrq)
        }
    }

    pub fn handle_interrupt(&mut self, irq: IrqNumber) -> Result<(), HelenIpcError> {
        let registration = self.irq_registrations.get(&irq).ok_or(HelenIpcError::InvalidIrq)?;

        if !registration.enabled.load(Ordering::SeqCst) {
            return Err(HelenIpcError::PermissionDenied);
        }

        registration.increment_counter();

        let (method, arg1, arg2, arg3, arg4) = (irq as u64, registration.get_counter() as u64, 0, 0, 0);

        let call_id = self.next_call_id.fetch_add(1, Ordering::SeqCst);
        let notification = HelenMessage::with_args(method, arg1, arg2, arg3, arg4, call_id, 0);

        let answerbox_id = registration.answerbox_id;
        if let Some(answerbox) = self.answerboxes.get_mut(&answerbox_id) {
            answerbox.notification_queue.push(notification);
            Ok(())
        } else {
            Err(HelenIpcError::AnswerboxNotFound)
        }
    }

    pub fn receive_notification(&mut self, answerbox_id: AnswerboxId) -> Result<HelenMessage, HelenIpcError> {
        let answerbox = self.answerboxes.get_mut(&answerbox_id).ok_or(HelenIpcError::AnswerboxNotFound)?;

        if answerbox.notification_queue.is_empty() {
            return Err(HelenIpcError::BufferEmpty);
        }

        Ok(answerbox.notification_queue.remove(0))
    }
}

impl Default for HelenIpcManager {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibrilType {
    Manager,
    Worker,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FibrilState {
    Ready,
    Running,
    Waiting,
    Suspended,
    Finished,
}

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

    pub fn create_fibril(&mut self, fibril_type: FibrilType) -> usize {
        let id = self.next_fibril_id.fetch_add(1, Ordering::SeqCst);
        let fibril = Fibril::new(id, fibril_type);
        self.fibrils.push(fibril);
        id
    }

    pub fn get_manager_fibril(&mut self, answerbox_id: AnswerboxId) -> usize {
        if let Some(fibril) = self.fibrils.iter().find(|f| {
            f.fibril_type == FibrilType::Manager && f.answerbox_id == Some(answerbox_id)
        }) {
            return fibril.id;
        }

        let id = self.create_fibril(FibrilType::Manager);
        if let Some(fibril) = self.fibrils.get_mut(id - 1) {
            fibril.set_answerbox(answerbox_id);
        }
        id
    }

    pub fn create_worker_fibril(&mut self, call_id: CallId) -> usize {
        let id = self.create_fibril(FibrilType::Worker);
        if let Some(fibril) = self.fibrils.get_mut(id - 1) {
            fibril.wait_for_call(call_id);
        }
        id
    }

    pub fn schedule_fibril(&mut self, fibril_id: usize) {
        if let Some(fibril) = self.fibrils.get_mut(fibril_id - 1) {
            fibril.state = FibrilState::Running;
        }
    }

    pub fn suspend_fibril(&mut self, fibril_id: usize) {
        if let Some(fibril) = self.fibrils.get_mut(fibril_id - 1) {
            fibril.state = FibrilState::Waiting;
        }
    }

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

    pub fn initialize_task(&mut self, task_id: usize) -> (AnswerboxId, PhoneId) {
        let answerbox_id = self.ipc_manager.create_answerbox(task_id, 64);
        let phone_id = self.ipc_manager.create_phone(task_id);

        self.fibril_manager.get_manager_fibril(answerbox_id);

        (answerbox_id, phone_id)
    }

    pub fn send_async_with_fibril(
        &mut self,
        phone_id: PhoneId,
        message: HelenMessage,
        from_fibril_id: usize,
    ) -> Result<(), HelenIpcError> {
        match self.ipc_manager.send_async(phone_id, message) {
            Ok(()) => Ok(()),
            Err(HelenIpcError::BufferFull) => {
                self.fibril_manager.suspend_fibril(from_fibril_id);
                Err(HelenIpcError::BufferFull)
            }
            Err(e) => Err(e),
        }
    }

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
