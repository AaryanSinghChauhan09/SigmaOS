#![no_std]
#![no_main]

/// OOP-based Audio Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 71
/// Implements audio device management and playback

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AudioDeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AudioType { Playback = 0, Capture = 1, Duplex = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AudioError { Success = 0, NotFound = 1, InitFailed = 2, PlaybackFailed = 3 }

pub trait AudioDevice {
    fn id(&self) -> AudioDeviceID;
    fn name(&self) -> &[u8];
    fn audio_type(&self) -> AudioType;
    fn sample_rate(&self) -> u32;
    fn initialize(&mut self) -> Result<(), AudioError>;
}

#[repr(C)]
pub struct SimpleAudioDevice {
    pub id: AudioDeviceID,
    pub name: [u8; 64],
    pub audio_type: AtomicUsize,
    pub sample_rate: AtomicUsize,
}

impl SimpleAudioDevice {
    pub fn new(id: AudioDeviceID, name: &[u8], audio_type: AudioType, sample_rate: u32) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleAudioDevice {
            id,
            name: name_array,
            audio_type: AtomicUsize::new(audio_type as usize),
            sample_rate: AtomicUsize::new(sample_rate as usize),
        }
    }
}

impl AudioDevice for SimpleAudioDevice {
    fn id(&self) -> AudioDeviceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn audio_type(&self) -> AudioType { unsafe { core::mem::transmute(self.audio_type.load(Ordering::SeqCst)) } }
    fn sample_rate(&self) -> u32 { self.sample_rate.load(Ordering::SeqCst) as u32 }
    
    fn initialize(&mut self) -> Result<(), AudioError> {
        Ok(())
    }
}

pub trait AudioManager {
    fn register_device(&mut self, device: Box<dyn AudioDevice>) -> Result<AudioDeviceID, AudioError>;
    fn get_default_playback(&self) -> Option<&dyn AudioDevice>;
    fn get_default_capture(&self) -> Option<&dyn AudioDevice>;
    fn list_devices(&self) -> Vec<AudioDeviceID>;
}

#[repr(C)]
pub struct SimpleAudioManager {
    pub devices: Vec<Option<Box<dyn AudioDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAudioManager {
    pub fn new() -> Self {
        SimpleAudioManager {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AudioManager for SimpleAudioManager {
    fn register_device(&mut self, device: Box<dyn AudioDevice>) -> Result<AudioDeviceID, AudioError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }
    
    fn get_default_playback(&self) -> Option<&dyn AudioDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.audio_type() == AudioType::Playback || device.audio_type() == AudioType::Duplex {
                    return Some(device.as_ref());
                }
            }
        }
        None
    }
    
    fn get_default_capture(&self) -> Option<&dyn AudioDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.audio_type() == AudioType::Capture || device.audio_type() == AudioType::Duplex {
                    return Some(device.as_ref());
                }
            }
        }
        None
    }
    
    fn list_devices(&self) -> Vec<AudioDeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                ids.push(device.id());
            }
        }
        ids
    }
}

pub trait AudioMixer {
    fn set_volume(&mut self, device_id: AudioDeviceID, volume: u8) -> Result<(), AudioError>;
    fn get_volume(&self, device_id: AudioDeviceID) -> u8;
    fn mute(&mut self, device_id: AudioDeviceID, muted: bool) -> Result<(), AudioError>;
}

#[repr(C)]
pub struct SimpleAudioMixer {
    pub volumes: Vec<(AudioDeviceID, AtomicUsize, AtomicUsize)>,
}

impl SimpleAudioMixer {
    pub fn new() -> Self {
        SimpleAudioMixer {
            volumes: Vec::new(),
        }
    }
}

impl AudioMixer for SimpleAudioMixer {
    fn set_volume(&mut self, device_id: AudioDeviceID, volume: u8) -> Result<(), AudioError> {
        for i in 0..self.volumes.len() {
            if self.volumes[i].0 == device_id {
                self.volumes[i].1.store(volume as usize, Ordering::SeqCst);
                return Ok(());
            }
        }
        self.volumes.push((device_id, AtomicUsize::new(volume as usize), AtomicUsize::new(0)));
        Ok(())
    }
    
    fn get_volume(&self, device_id: AudioDeviceID) -> u8 {
        for &(id, ref volume, _) in &self.volumes {
            if id == device_id {
                return volume.load(Ordering::SeqCst) as u8;
            }
        }
        100
    }
    
    fn mute(&mut self, device_id: AudioDeviceID, muted: bool) -> Result<(), AudioError> {
        for i in 0..self.volumes.len() {
            if self.volumes[i].0 == device_id {
                self.volumes[i].2.store(if muted { 1 } else { 0 }, Ordering::SeqCst);
                return Ok(());
            }
        }
        Err(AudioError::NotFound)
    }
}

pub trait AudioStream {
    fn create_stream(&mut self, device_id: AudioDeviceID, channels: u8, format: u32) -> Result<usize, AudioError>;
    fn write_samples(&mut self, stream_id: usize, samples: &[u8]) -> Result<(), AudioError>;
    fn read_samples(&mut self, stream_id: usize, buffer: &mut [u8]) -> Result<usize, AudioError>;
}

#[repr(C)]
pub struct SimpleAudioStream {
    pub streams: Vec<(usize, AudioDeviceID, u8, u32)>,
    pub next_id: AtomicUsize,
}

impl SimpleAudioStream {
    pub fn new() -> Self {
        SimpleAudioStream {
            streams: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AudioStream for SimpleAudioStream {
    fn create_stream(&mut self, device_id: AudioDeviceID, channels: u8, format: u32) -> Result<usize, AudioError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.streams.push((id, device_id, channels, format));
        Ok(id)
    }
    
    fn write_samples(&mut self, _stream_id: usize, _samples: &[u8]) -> Result<(), AudioError> {
        Ok(())
    }
    
    fn read_samples(&mut self, _stream_id: usize, _buffer: &mut [u8]) -> Result<usize, AudioError> {
        Ok(0)
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
