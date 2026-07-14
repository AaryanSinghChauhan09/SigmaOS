#![no_std]
#![no_main]

/// OOP-based SD Card for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1186
/// Implements SD card interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CardID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CardType { SDSC = 0, SDHC = 1, SDXC = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SDCardError { Success = 0, NotFound = 1, ReadFailed = 2 }

pub trait SDCard {
    fn id(&self) -> CardID;
    fn card_type(&self) -> CardType;
    fn capacity(&self) -> u64;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSDCard {
    pub id: CardID,
    pub card_type: AtomicUsize,
    pub capacity: AtomicUsize,
    pub initialized: AtomicUsize,
}

impl SimpleSDCard {
    pub fn new(id: CardID, card_type: CardType, capacity: u64) -> Self {
        SimpleSDCard {
            id,
            card_type: AtomicUsize::new(card_type as usize),
            capacity: AtomicUsize::new(capacity as usize),
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SDCard for SimpleSDCard {
    fn id(&self) -> CardID { self.id }
    fn card_type(&self) -> CardType { unsafe { core::mem::transmute(self.card_type.load(Ordering::SeqCst)) } }
    fn capacity(&self) -> u64 { self.capacity.load(Ordering::SeqCst) as u64 }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SDCardController {
    fn init(&mut self) -> Result<(), SDCardError>;
    fn read_block(&self, block: u32, buffer: &mut [u8]) -> Result<(), SDCardError>;
    fn write_block(&self, block: u32, data: &[u8]) -> Result<(), SDCardError>;
}

#[repr(C)]
pub struct SimpleSDCardController {
    pub card: Option<Box<dyn SDCard>>,
}

impl SimpleSDCardController {
    pub fn new() -> Self {
        SimpleSDCardController {
            card: None,
        }
    }
}

impl SDCardController for SimpleSDCardController {
    fn init(&mut self) -> Result<(), SDCardError> {
        let card = SimpleSDCard::new(1, CardType::SDHC, 32 * 1024 * 1024 * 1024);
        card.initialized.store(1, Ordering::SeqCst);
        self.card = Some(Box::new(card));
        Ok(())
    }
    
    fn read_block(&self, _block: u32, buffer: &mut [u8]) -> Result<(), SDCardError> {
        if self.card.is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(SDCardError::NotFound)
        }
    }
    
    fn write_block(&self, _block: u32, _data: &[u8]) -> Result<(), SDCardError> {
        if self.card.is_some() {
            Ok(())
        } else {
            Err(SDCardError::NotFound)
        }
    }
}

pub trait FileSystem {
    def mount(&mut self) -> Result<(), SDCardError>;
    def unmount(&mut self) -> Result<(), SDCardError>;
    def is_mounted(&self) -> bool;
}

#[repr(C)]
pub struct SimpleFileSystem {
    pub mounted: AtomicUsize,
}

impl SimpleFileSystem {
    pub fn new() -> Self {
        SimpleFileSystem {
            mounted: AtomicUsize::new(0),
        }
    }
}

impl FileSystem for SimpleFileSystem {
    fn mount(&mut self) -> Result<(), SDCardError> {
        self.mounted.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn unmount(&mut self) -> Result<(), SDCardError> {
        self.mounted.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn is_mounted(&self) -> bool { self.mounted.load(Ordering::SeqCst) == 1 }
}
