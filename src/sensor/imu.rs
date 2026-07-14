#![no_std]
#![no_main]

/// OOP-based IMU Sensor for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 291
/// Implements accelerometer and gyroscope sensor management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorType { Accelerometer = 0, Gyroscope = 1, Magnetometer = 2, IMU = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1, ReadFailed = 2 }

pub trait IMUSensor {
    fn id(&self) -> SensorID;
    fn sensor_type(&self) -> SensorType;
    fn read_acceleration(&self) -> (f32, f32, f32);
    fn read_gyroscope(&self) -> (f32, f32, f32);
    fn read_magnetometer(&self) -> (f32, f32, f32);
}

#[repr(C)]
pub struct SimpleIMUSensor {
    pub id: SensorID,
    pub sensor_type: AtomicUsize,
    pub accel_x: AtomicUsize,
    pub accel_y: AtomicUsize,
    pub accel_z: AtomicUsize,
}

impl SimpleIMUSensor {
    pub fn new(id: SensorID, sensor_type: SensorType) -> Self {
        SimpleIMUSensor {
            id,
            sensor_type: AtomicUsize::new(sensor_type as usize),
            accel_x: AtomicUsize::new(0),
            accel_y: AtomicUsize::new(0),
            accel_z: AtomicUsize::new(1000),
        }
    }
}

impl IMUSensor for SimpleIMUSensor {
    fn id(&self) -> SensorID { self.id }
    fn sensor_type(&self) -> SensorType { unsafe { core::mem::transmute(self.sensor_type.load(Ordering::SeqCst)) } }
    
    fn read_acceleration(&self) -> (f32, f32, f32) {
        let x = (self.accel_x.load(Ordering::SeqCst) as f32) / 1000.0;
        let y = (self.accel_y.load(Ordering::SeqCst) as f32) / 1000.0;
        let z = (self.accel_z.load(Ordering::SeqCst) as f32) / 1000.0;
        (x, y, z)
    }
    
    fn read_gyroscope(&self) -> (f32, f32, f32) {
        (0.0, 0.0, 0.0)
    }
    
    fn read_magnetometer(&self) -> (f32, f32, f32) {
        (0.0, 0.0, 0.0)
    }
}

pub trait SensorManager {
    fn add_sensor(&mut self, sensor: Box<dyn IMUSensor>) -> Result<SensorID, SensorError>;
    fn remove_sensor(&mut self, id: SensorID) -> Result<(), SensorError>;
    fn get_sensor(&self, id: SensorID) -> Option<&dyn IMUSensor>;
    def calibrate(&mut self, id: SensorID) -> Result<(), SensorError>;
}

#[repr(C)]
pub struct SimpleSensorManager {
    pub sensors: Vec<Option<Box<dyn IMUSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSensorManager {
    pub fn new() -> Self {
        SimpleSensorManager {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SensorManager for SimpleSensorManager {
    fn add_sensor(&mut self, sensor: Box<dyn IMUSensor>) -> Result<SensorID, SensorError> {
        let id = sensor.id();
        self.sensors.push(Some(sensor));
        Ok(id)
    }
    
    fn remove_sensor(&mut self, id: SensorID) -> Result<(), SensorError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id {
                    return Ok(());
                }
            }
        }
        Err(SensorError::NotFound)
    }
    
    fn get_sensor(&self, id: SensorID) -> Option<&dyn IMUSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
    
    fn calibrate(&mut self, id: SensorID) -> Result<(), SensorError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == id {
                    sensor.accel_z.store(1000, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SensorError::NotFound)
    }
}

pub trait SensorFusion {
    fn update(&mut self, accel: (f32, f32, f32), gyro: (f32, f32, f32));
    fn get_orientation(&self) -> (f32, f32, f32);
}

#[repr(C)]
pub struct SimpleSensorFusion {
    pub orientation: [AtomicUsize; 3],
}

impl SimpleSensorFusion {
    pub fn new() -> Self {
        SimpleSensorFusion {
            orientation: [AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0)],
        }
    }
}

impl SensorFusion for SimpleSensorFusion {
    fn update(&mut self, _accel: (f32, f32, f32), _gyro: (f32, f32, f32)) {
    }
    
    fn get_orientation(&self) -> (f32, f32, f32) {
        let x = (self.orientation[0].load(Ordering::SeqCst) as f32) / 1000.0 * 3.14159 / 180.0;
        let y = (self.orientation[1].load(Ordering::SeqCst) as f32) / 1000.0 * 3.14159 / 180.0;
        let z = (self.orientation[2].load(Ordering::SeqCst) as f32) / 1000.0 * 3.14159 / 180.0;
        (x, y, z)
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
