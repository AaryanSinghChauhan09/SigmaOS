#![no_std]
#![no_main]

/// OOP-based Smartcard Reader for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 572
/// Implements smartcard communication and authentication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CardID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CardState { Empty = 0, Present = 1, Active = 2, Error = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CardError { Success = 0, NotFound = 1, ReadFailed = 2 }

pub trait Smartcard {
    fn id(&self) -> CardID;
    fn atr(&self) -> &[u8];
    fn state(&self) -> CardState;
    fn set_state(&mut self, state: CardState);
}

#[repr(C)]
pub struct SimpleSmartcard {
    pub id: CardID,
    pub atr: [u8; 32],
    pub state: AtomicUsize,
}

impl SimpleSmartcard {
    pub fn new(id: CardID, atr: &[u8]) -> Self {
        let mut atr_array = [0u8; 32];
        let atr_len = atr.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(atr.as_ptr(), atr_array.as_mut_ptr(), atr_len);
        }
        SimpleSmartcard {
            id,
            atr: atr_array,
            state: AtomicUsize::new(CardState::Empty as usize),
        }
    }
}

impl Smartcard for SimpleSmartcard {
    fn id(&self) -> CardID { self.id }
    fn atr(&self) -> &[u8] {
        let len = self.atr.iter().position(|&b| b == 0).unwrap_or(32);
        &self.atr[..len]
    }
    fn state(&self) -> CardState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    
    fn set_state(&mut self, state: CardState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait SmartcardReader {
    fn detect_card(&mut self) -> Result<CardID, CardError>;
    def read_apdu(&self, card_id: CardID, apdu: &[u8]) -> Result<Vec<u8>, CardError>;
    def write_apdu(&self, card_id: CardID, apdu: &[u8]) -> Result<(), CardError>;
}

#[repr(C)]
pub struct SimpleSmartcardReader {
    pub cards: Vec<Option<Box<dyn Smartcard>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSmartcardReader {
    pub fn new() -> Self {
        SimpleSmartcardReader {
            cards: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SmartcardReader for SimpleSmartcardReader {
    fn detect_card(&mut self) -> Result<CardID, CardError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let card = SimpleSmartcard::new(id, b"3B9F95801C670D");
        card.set_state(CardState::Present);
        self.cards.push(Some(Box::new(card)));
        Ok(id)
    }
    
    fn read_apdu(&self, card_id: CardID, _apdu: &[u8]) -> Result<Vec<u8>, CardError> {
        for card_option in &self.cards {
            if let Some(ref card) = *card_option {
                if card.id() == card_id {
                    let mut response = Vec::new();
                    response.push(0x90);
                    response.push(0x00);
                    return Ok(response);
                }
            }
        }
        Err(CardError::NotFound)
    }
    
    fn write_apdu(&self, card_id: CardID, _apdu: &[u8]) -> Result<(), CardError> {
        for card_option in &self.cards {
            if let Some(ref card) *card_option {
                if card.id() == card_id {
                    return Ok(());
                }
            }
        }
        Err(CardError::NotFound)
    }
}

pub trait PKCS11 {
    fn initialize(&mut self) -> Result<(), CardError>;
    fn login(&mut self, pin: &[u8]) -> Result<(), CardError>;
    fn logout(&mut self) -> Result<(), CardError>;
}

#[repr(C)]
pub struct SimplePKCS11 {
    pub logged_in: AtomicUsize,
}

impl SimplePKCS11 {
    pub fn new() -> Self {
        SimplePKCS11 {
            logged_in: AtomicUsize::new(0),
        }
    }
}

impl PKCS11 for SimplePKCS11 {
    fn initialize(&mut self) -> Result<(), CardError> {
        Ok(())
    }
    
    fn login(&mut self, _pin: &[u8]) -> Result<(), CardError> {
        self.logged_in.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn logout(&mut self) -> Result<(), CardError> {
        self.logged_in.store(0, Ordering::SeqCst);
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
