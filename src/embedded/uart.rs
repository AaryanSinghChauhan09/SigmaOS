#![no_std]
#![no_main]

/// OOP-based UART for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1136
/// Implements UART serial communication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type UARTID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Parity { None = 0, Even = 1, Odd = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum UARTError { Success = 0, NotFound = 1 }

pub trait UARTPort {
    fn id(&self) -> UARTID;
    fn baud_rate(&self) -> u32;
    fn data_bits(&self) -> u8;
}

#[repr(C)]
pub struct SimpleUARTPort {
    pub id: UARTID,
    pub baud_rate: AtomicUsize,
    pub data_bits: AtomicUsize,
}

impl SimpleUARTPort {
    pub fn new(id: UARTID, baud_rate: u32, data_bits: u8) -> Self {
        SimpleUARTPort {
            id,
            baud_rate: AtomicUsize::new(baud_rate as usize),
            data_bits: AtomicUsize::new(data_bits as usize),
        }
    }
}

impl UARTPort for SimpleUARTPort {
    fn id(&self) -> UARTID { self.id }
    fn baud_rate(&self) -> u32 { self.baud_rate.load(Ordering::SeqCst) as u32 }
    fn data_bits(&self) -> u8 { self.data_bits.load(Ordering::SeqCst) as u8 }
}

pub trait UARTController {
    fn configure(&mut self, uart_id: UARTID, baud_rate: u32, data_bits: u8, parity: Parity) -> Result<(), UARTError>;
    fn write(&self, uart_id: UARTID, data: &[u8]) -> Result<(), UARTError>;
    fn read(&self, uart_id: UARTID, buffer: &mut [u8]) -> Result<usize, UARTError>;
}

#[repr(C)]
pub struct SimpleUARTController {
    pub ports: Vec<Option<Box<dyn UARTPort>>>,
    pub next_id: AtomicUsize,
}

impl SimpleUARTController {
    pub fn new() -> Self {
        SimpleUARTController {
            ports: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl UARTController for SimpleUARTController {
    fn configure(&mut self, uart_id: UARTID, baud_rate: u32, data_bits: u8, _parity: Parity) -> Result<(), UARTError> {
        let port = SimpleUARTPort::new(uart_id, baud_rate, data_bits);
        self.ports.push(Some(Box::new(port)));
        Ok(())
    }
    
    fn write(&self, uart_id: UARTID, _data: &[u8]) -> Result<(), UARTError> {
        if self.get_port(uart_id).is_some() {
            Ok(())
        } else {
            Err(UARTError::NotFound)
        }
    }
    
    fn read(&self, uart_id: UARTID, buffer: &mut [u8]) -> Result<usize, UARTError> {
        if self.get_port(uart_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(buffer.len())
        } else {
            Err(UARTError::NotFound)
        }
    }
    
    fn get_port(&self, id: UARTID) -> Option<&dyn UARTPort> {
        for port_option in &self.ports {
            if let Some(ref port) = *port_option {
                if port.id() == id { return Some(port.as_ref()); }
            }
        }
        None
    }
}

pub trait Console {
    def init(&mut self, uart_id: UARTID) -> Result<(), UARTError>;
    def puts(&self, s: &[u8]);
    def gets(&self, buffer: &mut [u8]) -> usize;
}

#[repr(C)]
pub struct SimpleConsole {
    pub controller: SimpleUARTController,
    pub active_uart: AtomicUsize,
}

impl SimpleConsole {
    pub fn new(controller: SimpleUARTController) -> Self {
        SimpleConsole {
            controller,
            active_uart: AtomicUsize::new(0),
        }
    }
}

impl Console for SimpleConsole {
    fn init(&mut self, uart_id: UARTID) -> Result<(), UARTError> {
        self.active_uart.store(uart_id, Ordering::SeqCst);
        Ok(())
    }
    
    fn puts(&self, s: &[u8]) {
        let uart_id = self.active_uart.load(Ordering::SeqCst);
        let _ = self.controller.write(uart_id, s);
    }
    
    fn gets(&self, buffer: &mut [u8]) -> usize {
        let uart_id = self.active_uart.load(Ordering::SeqCst);
        match self.controller.read(uart_id, buffer) {
            Ok(len) => len,
            Err(_) => 0,
        }
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
