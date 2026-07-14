#![no_std]
#![no_main]

/// OOP-based GPS Location for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 341
/// Implements GPS positioning and tracking

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LocationID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LocationError { Success = 0, NoFix = 1, NotFound = 2 }

pub trait Location {
    fn latitude(&self) -> f64;
    fn longitude(&self) -> f64;
    fn altitude(&self) -> f64;
    fn accuracy(&self) -> f64;
    fn timestamp(&self) -> u64;
}

#[repr(C)]
pub struct SimpleLocation {
    pub latitude: AtomicUsize,
    pub longitude: AtomicUsize,
    pub altitude: AtomicUsize,
    pub accuracy: AtomicUsize,
    pub timestamp: AtomicUsize,
}

impl SimpleLocation {
    pub fn new() -> Self {
        SimpleLocation {
            latitude: AtomicUsize::new(0),
            longitude: AtomicUsize::new(0),
            altitude: AtomicUsize::new(0),
            accuracy: AtomicUsize::new(0),
            timestamp: AtomicUsize::new(1000000),
        }
    }
}

impl Location for SimpleLocation {
    fn latitude(&self) -> f64 { (self.latitude.load(Ordering::SeqCst) as f64) / 1000000.0 }
    fn longitude(&self) -> f64 { (self.longitude.load(Ordering::SeqCst) as f64) / 1000000.0 }
    fn altitude(&self) -> f64 { (self.altitude.load(Ordering::SeqCst) as f64) / 100.0 }
    fn accuracy(&self) -> f64 { (self.accuracy.load(Ordering::SeqCst) as f64) / 100.0 }
    fn timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
}

pub trait GPS {
    fn id(&self) -> LocationID;
    fn has_fix(&self) -> bool;
    fn get_location(&self) -> &dyn Location;
    fn update_location(&mut self, lat: f64, lon: f64, alt: f64);
}

#[repr(C)]
pub struct SimpleGPS {
    pub id: LocationID,
    pub has_fix: AtomicUsize,
    pub location: SimpleLocation,
}

impl SimpleGPS {
    pub fn new(id: LocationID) -> Self {
        SimpleGPS {
            id,
            has_fix: AtomicUsize::new(0),
            location: SimpleLocation::new(),
        }
    }
}

impl GPS for SimpleGPS {
    fn id(&self) -> LocationID { self.id }
    fn has_fix(&self) -> bool { self.has_fix.load(Ordering::SeqCst) == 1 }
    fn get_location(&self) -> &dyn Location { &self.location }
    
    fn update_location(&mut self, lat: f64, lon: f64, alt: f64) {
        self.location.latitude.store((lat * 1000000.0) as usize, Ordering::SeqCst);
        self.location.longitude.store((lon * 1000000.0) as usize, Ordering::SeqCst);
        self.location.altitude.store((alt * 100.0) as usize, Ordering::SeqCst);
        self.has_fix.store(1, Ordering::SeqCst);
    }
}

pub trait LocationTracker {
    fn add_gps(&mut self, gps: Box<dyn GPS>) -> Result<LocationID, LocationError>;
    fn remove_gps(&mut self, id: LocationID) -> Result<(), LocationError>;
    fn get_gps(&self, id: LocationID) -> Option<&dyn GPS>;
    fn track_route(&mut self, gps_id: LocationID) -> Result<(), LocationError>;
}

#[repr(C)]
pub struct SimpleLocationTracker {
    pub gps_devices: Vec<Option<Box<dyn GPS>>>,
    pub routes: Vec<(LocationID, Vec<(f64, f64)>)>,
    pub next_id: AtomicUsize,
}

impl SimpleLocationTracker {
    pub fn new() -> Self {
        SimpleLocationTracker {
            gps_devices: Vec::new(),
            routes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LocationTracker for SimpleLocationTracker {
    fn add_gps(&mut self, gps: Box<dyn GPS>) -> Result<LocationID, LocationError> {
        let id = gps.id();
        self.gps_devices.push(Some(gps));
        Ok(id)
    }
    
    fn remove_gps(&mut self, id: LocationID) -> Result<(), LocationError> {
        for gps_option in &mut self.gps_devices {
            if let Some(ref gps) = *gps_option {
                if gps.id() == id {
                    return Ok(());
                }
            }
        }
        Err(LocationError::NotFound)
    }
    
    fn get_gps(&self, id: LocationID) -> Option<&dyn GPS> {
        for gps_option in &self.gps_devices {
            if let Some(ref gps) = *gps_option {
                if gps.id() == id { return Some(gps.as_ref()); }
            }
        }
        None
    }
    
    fn track_route(&mut self, gps_id: LocationID) -> Result<(), LocationError> {
        self.routes.push((gps_id, Vec::new()));
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
