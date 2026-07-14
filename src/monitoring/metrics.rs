#![no_std]
#![no_main]

/// OOP-based Metrics Collection for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 151
/// Implements system metrics collection and monitoring

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MetricID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MetricType { Counter = 0, Gauge = 1, Histogram = 2, Summary = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MetricError { Success = 0, NotFound = 1, InvalidType = 2 }

pub trait Metric {
    fn id(&self) -> MetricID;
    fn name(&self) -> &[u8];
    fn metric_type(&self) -> MetricType;
    fn value(&self) -> f64;
    fn set_value(&mut self, value: f64);
}

#[repr(C)]
pub struct SimpleMetric {
    pub id: MetricID,
    pub name: [u8; 64],
    pub metric_type: AtomicUsize,
    pub value: AtomicUsize,
}

impl SimpleMetric {
    pub fn new(id: MetricID, name: &[u8], metric_type: MetricType, value: f64) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleMetric {
            id,
            name: name_array,
            metric_type: AtomicUsize::new(metric_type as usize),
            value: AtomicUsize::new((value * 10000.0) as usize),
        }
    }
}

impl Metric for SimpleMetric {
    fn id(&self) -> MetricID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn metric_type(&self) -> MetricType { unsafe { core::mem::transmute(self.metric_type.load(Ordering::SeqCst)) } }
    fn value(&self) -> f64 { (self.value.load(Ordering::SeqCst) as f64) / 10000.0 }
    
    fn set_value(&mut self, value: f64) {
        self.value.store((value * 10000.0) as usize, Ordering::SeqCst);
    }
}

pub trait MetricsCollector {
    fn register_metric(&mut self, metric: Box<dyn Metric>) -> Result<MetricID, MetricError>;
    fn unregister_metric(&mut self, id: MetricID) -> Result<(), MetricError>;
    fn get_metric(&self, id: MetricID) -> Option<&dyn Metric>;
    fn increment(&mut self, id: MetricID, delta: f64) -> Result<(), MetricError>;
    fn set(&mut self, id: MetricID, value: f64) -> Result<(), MetricError>;
}

#[repr(C)]
pub struct SimpleMetricsCollector {
    pub metrics: Vec<Option<Box<dyn Metric>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMetricsCollector {
    pub fn new() -> Self {
        SimpleMetricsCollector {
            metrics: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MetricsCollector for SimpleMetricsCollector {
    fn register_metric(&mut self, metric: Box<dyn Metric>) -> Result<MetricID, MetricError> {
        let id = metric.id();
        self.metrics.push(Some(metric));
        Ok(id)
    }
    
    fn unregister_metric(&mut self, id: MetricID) -> Result<(), MetricError> {
        for metric_option in &mut self.metrics {
            if let Some(ref metric) = *metric_option {
                if metric.id() == id {
                    return Ok(());
                }
            }
        }
        Err(MetricError::NotFound)
    }
    
    fn get_metric(&self, id: MetricID) -> Option<&dyn Metric> {
        for metric_option in &self.metrics {
            if let Some(ref metric) = *metric_option {
                if metric.id() == id { return Some(metric.as_ref()); }
            }
        }
        None
    }
    
    fn increment(&mut self, id: MetricID, delta: f64) -> Result<(), MetricError> {
        for metric_option in &mut self.metrics {
            if let Some(ref mut metric) = *metric_option {
                if metric.id() == id {
                    let current = metric.value();
                    metric.set_value(current + delta);
                    return Ok(());
                }
            }
        }
        Err(MetricError::NotFound)
    }
    
    fn set(&mut self, id: MetricID, value: f64) -> Result<(), MetricError> {
        for metric_option in &mut self.metrics {
            if let Some(ref mut metric) = *metric_option {
                if metric.id() == id {
                    metric.set_value(value);
                    return Ok(());
                }
            }
        }
        Err(MetricError::NotFound)
    }
}

pub trait MetricsExporter {
    fn export(&self) -> Vec<&[u8]>;
    fn export_prometheus(&self) -> Vec<u8>;
}

#[repr(C)]
pub struct SimpleMetricsExporter {
    pub collector: SimpleMetricsCollector,
}

impl SimpleMetricsExporter {
    pub fn new(collector: SimpleMetricsCollector) -> Self {
        SimpleMetricsExporter { collector }
    }
}

impl MetricsExporter for SimpleMetricsExporter {
    fn export(&self) -> Vec<&[u8]> {
        let mut lines = Vec::new();
        for metric_option in &self.collector.metrics {
            if let Some(ref metric) = *metric_option {
                lines.push(metric.name());
            }
        }
        lines
    }
    
    fn export_prometheus(&self) -> Vec<u8> {
        let mut output = Vec::new();
        for metric_option in &self.collector.metrics {
            if let Some(ref metric) = *metric_option {
                let name = metric.name();
                let value = metric.value();
                
                for &byte in name { output.push(byte); }
                output.push(b' ');
                
                let value_str = format_simple(value);
                for &byte in &value_str { output.push(byte); }
                output.push(b'\n');
            }
        }
        output
    }
}

fn format_simple(value: f64) -> Vec<u8> {
    let int_part = value as i32;
    let frac_part = ((value - int_part as f64) * 1000.0) as i32;
    
    let mut result = Vec::new();
    
    if int_part < 0 {
        result.push(b'-');
    }
    
    let mut n = (int_part as i32).abs();
    if n == 0 {
        result.push(b'0');
    } else {
        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % 10) as u8 + b'0');
            n /= 10;
        }
        while !digits.is_empty() {
            result.push(digits.pop().unwrap());
        }
    }
    
    if frac_part != 0 {
        result.push(b'.');
        let frac_abs = frac_part.abs();
        if frac_abs < 100 { result.push(b'0'); }
        if frac_abs < 10 { result.push(b'0'); }
        let mut n = frac_abs;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push((n % 10) as u8 + b'0');
            n /= 10;
        }
        while !digits.is_empty() {
            result.push(digits.pop().unwrap());
        }
    }
    
    result
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
    fn is_empty(&self) -> bool { self.len == 0 }
    fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            unsafe { Some(core::ptr::read(self.data.add(self.len))) }
        } else {
            None
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
