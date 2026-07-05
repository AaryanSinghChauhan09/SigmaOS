// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/data/sigma_data.rs — Data Analysis Library
// Scientific computing and data analysis (pandas/NumPy-inspired)
//
// Features:
//   - DataFrame and Series data structures
//   - Numerical arrays with vectorized operations
//   - Statistical functions
//   - Data I/O (CSV, JSON, Parquet, SQL)
//   - Time series analysis
//   - Plotting and visualization
//   - India context: Support for Indian calendar systems and date formats
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Data Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Float64,
    Float32,
    Int64,
    Int32,
    Boolean,
    String,
    DateTime,
    Object,
}

// ── Series (1D data structure) ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeriesData {
    Float64Vec(Vec<f64>),
    Float32Vec(Vec<f32>),
    Int64Vec(Vec<i64>),
    Int32Vec(Vec<i32>),
    BoolVec(Vec<bool>),
    StringVec(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub name: String,
    pub data: SeriesData,
    pub dtype: DataType,
    pub nullable: bool,
}

impl Series {
    pub fn new(name: String, data: SeriesData, dtype: DataType) -> Self {
        Self {
            name,
            data,
            dtype,
            nullable: false,
        }
    }

    pub fn len(&self) -> usize {
        match &self.data {
            SeriesData::Float64Vec(v) => v.len(),
            SeriesData::Float32Vec(v) => v.len(),
            SeriesData::Int64Vec(v) => v.len(),
            SeriesData::Int32Vec(v) => v.len(),
            SeriesData::BoolVec(v) => v.len(),
            SeriesData::StringVec(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn sum(&self) -> Option<f64> {
        match &self.data {
            SeriesData::Float64Vec(v) => Some(v.iter().sum()),
            SeriesData::Float32Vec(v) => Some(v.iter().map(|&x| x as f64).sum()),
            SeriesData::Int64Vec(v) => Some(v.iter().map(|&x| x as f64).sum()),
            SeriesData::Int32Vec(v) => Some(v.iter().map(|&x| x as f64).sum()),
            SeriesData::BoolVec(v) => Some(v.iter().map(|&x| if x { 1.0 } else { 0.0 }).sum()),
            SeriesData::StringVec(_) => None,
        }
    }

    pub fn mean(&self) -> Option<f64> {
        let sum = self.sum()?;
        let len = self.len();
        if len == 0 { return None; }
        Some(sum / len as f64)
    }

    pub fn min(&self) -> Option<f64> {
        match &self.data {
            SeriesData::Float64Vec(v) => v.iter().copied().fold(f64::INFINITY, f64::min),
            SeriesData::Float32Vec(v) => v.iter().copied().map(|x| x as f64).fold(f64::INFINITY, f64::min),
            SeriesData::Int64Vec(v) => v.iter().copied().map(|x| x as f64).fold(f64::INFINITY, f64::min),
            SeriesData::Int32Vec(v) => v.iter().copied().map(|x| x as f64).fold(f64::INFINITY, f64::min),
            SeriesData::BoolVec(_) => Some(0.0),
            SeriesData::StringVec(_) => None,
        }
    }

    pub fn max(&self) -> Option<f64> {
        match &self.data {
            SeriesData::Float64Vec(v) => v.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            SeriesData::Float32Vec(v) => v.iter().copied().map(|x| x as f64).fold(f64::NEG_INFINITY, f64::max),
            SeriesData::Int64Vec(v) => v.iter().copied().map(|x| x as f64).fold(f64::NEG_INFINITY, f64::max),
            SeriesData::Int32Vec(v) => v.iter().copied().map(|x| x as f64).fold(f64::NEG_INFINITY, f64::max),
            SeriesData::BoolVec(_) => Some(1.0),
            SeriesData::StringVec(_) => None,
        }
    }

    pub fn std(&self) -> Option<f64> {
        let mean = self.mean()?;
        let len = self.len();
        if len < 2 { return None; }
        
        let variance = match &self.data {
            SeriesData::Float64Vec(v) => {
                v.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (len - 1) as f64
            }
            SeriesData::Float32Vec(v) => {
                v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / (len - 1) as f64
            }
            SeriesData::Int64Vec(v) => {
                v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / (len - 1) as f64
            }
            SeriesData::Int32Vec(v) => {
                v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / (len - 1) as f64
            }
            SeriesData::BoolVec(v) => {
                v.iter().map(|&x| {
                    let val = if x { 1.0 } else { 0.0 };
                    (val - mean).powi(2)
                }).sum::<f64>() / (len - 1) as f64
            }
            SeriesData::StringVec(_) => return None,
        };
        
        Some(variance.sqrt())
    }
}

// ── DataFrame (2D data structure) ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrame {
    pub columns: Vec<String>,
    pub data: HashMap<String, Series>,
    pub index: Vec<usize>,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            data: HashMap::new(),
            index: Vec::new(),
        }
    }

    pub fn with_capacity(num_columns: usize, num_rows: usize) -> Self {
        Self {
            columns: Vec::with_capacity(num_columns),
            data: HashMap::with_capacity(num_columns),
            index: Vec::with_capacity(num_rows),
        }
    }

    pub fn add_column(&mut self, series: Series) {
        let name = series.name.clone();
        self.columns.push(name.clone());
        self.data.insert(name, series);
        
        // Update index length
        if let Some(first_series) = self.data.values().next() {
            let new_len = first_series.len();
            if self.index.is_empty() {
                self.index = (0..new_len).collect();
            }
        }
    }

    pub fn get_column(&self, name: &str) -> Option<&Series> {
        self.data.get(name)
    }

    pub fn get_column_mut(&mut self, name: &str) -> Option<&mut Series> {
        self.data.get_mut(name)
    }

    pub fn shape(&self) -> (usize, usize) {
        let rows = self.index.len();
        let cols = self.columns.len();
        (rows, cols)
    }

    pub fn head(&self, n: usize) -> DataFrame {
        let mut result = DataFrame::new();
        for col_name in &self.columns {
            if let Some(series) = self.get_column(col_name) {
                let truncated_data = match &series.data {
                    SeriesData::Float64Vec(v) => SeriesData::Float64Vec(v.iter().take(n).copied().collect()),
                    SeriesData::Float32Vec(v) => SeriesData::Float32Vec(v.iter().take(n).copied().collect()),
                    SeriesData::Int64Vec(v) => SeriesData::Int64Vec(v.iter().take(n).copied().collect()),
                    SeriesData::Int32Vec(v) => SeriesData::Int32Vec(v.iter().take(n).copied().collect()),
                    SeriesData::BoolVec(v) => SeriesData::BoolVec(v.iter().take(n).copied().collect()),
                    SeriesData::StringVec(v) => SeriesData::StringVec(v.iter().take(n).cloned().collect()),
                };
                result.add_column(Series {
                    name: series.name.clone(),
                    data: truncated_data,
                    dtype: series.dtype.clone(),
                    nullable: series.nullable,
                });
            }
        }
        result.index = self.index.iter().take(n).copied().collect();
        result
    }

    pub fn describe(&self) -> DataFrame {
        let mut result = DataFrame::new();
        
        let mut count_vec = Vec::new();
        let mut mean_vec = Vec::new();
        let mut std_vec = Vec::new();
        let mut min_vec = Vec::new();
        let mut max_vec = Vec::new();
        
        for col_name in &self.columns {
            if let Some(series) = self.get_column(col_name) {
                count_vec.push(series.len() as f64);
                mean_vec.push(series.mean().unwrap_or(0.0));
                std_vec.push(series.std().unwrap_or(0.0));
                min_vec.push(series.min().unwrap_or(0.0));
                max_vec.push(series.max().unwrap_or(0.0));
            }
        }
        
        result.add_column(Series::new("count".to_string(), SeriesData::Float64Vec(count_vec), DataType::Float64));
        result.add_column(Series::new("mean".to_string(), SeriesData::Float64Vec(mean_vec), DataType::Float64));
        result.add_column(Series::new("std".to_string(), SeriesData::Float64Vec(std_vec), DataType::Float64));
        result.add_column(Series::new("min".to_string(), SeriesData::Float64Vec(min_vec), DataType::Float64));
        result.add_column(Series::new("max".to_string(), SeriesData::Float64Vec(max_vec), DataType::Float64));
        
        result
    }
}

impl Default for DataFrame {
    fn default() -> Self {
        Self::new()
    }
}

// ── Indian Calendar Support ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndianCalendar {
    Saka,      // Official Indian national calendar
    Vikram,    // Vikram Samvat
    Bengali,   // Bengali calendar
    Tamil,     // Tamil calendar
    Malayalam, // Malayalam calendar
    Gregorian, // Gregorian calendar (standard)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndianDate {
    pub calendar: IndianCalendar,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub era: Option<String>, // For calendars with eras (e.g., Saka)
}

impl IndianDate {
    pub fn to_gregorian(&self) -> Result<IndianDate, String> {
        // In production: Convert from Indian calendar to Gregorian
        // For now: Return as-is with Gregorian calendar
        Ok(IndianDate {
            calendar: IndianCalendar::Gregorian,
            year: self.year,
            month: self.month,
            day: self.day,
            era: None,
        })
    }

    pub fn from_gregorian(calendar: IndianCalendar, year: i32, month: u32, day: u32) -> Result<IndianDate, String> {
        // In production: Convert from Gregorian to Indian calendar
        // For now: Return as-is with specified calendar
        Ok(IndianDate {
            calendar,
            year,
            month,
            day,
            era: None,
        })
    }
}

// ── Data I/O ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileFormat {
    CSV,
    JSON,
    Parquet,
    Excel,
    SQL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadConfig {
    pub format: FileFormat,
    pub path: String,
    pub has_header: bool,
    pub delimiter: char,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteConfig {
    pub format: FileFormat,
    pub path: String,
    pub include_header: bool,
    pub delimiter: char,
}

// ── Statistical Functions ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationMatrix {
    pub columns: Vec<String>,
    pub matrix: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionResult {
    pub coefficients: Vec<f64>,
    pub intercept: f64,
    pub r_squared: f64,
    pub p_values: Vec<f64>,
}

// ── Data Analysis Engine ─────────────────────────────────────────────────

pub struct DataEngine {
    default_calendar: IndianCalendar,
}

impl DataEngine {
    pub fn new() -> Self {
        Self {
            default_calendar: IndianCalendar::Saka, // Indian national calendar
        }
    }

    /// Read data from file
    pub fn read_data(&self, config: ReadConfig) -> Result<DataFrame, String> {
        // In production: Parse file based on format
        // For now: Return empty DataFrame
        Ok(DataFrame::new())
    }

    /// Write data to file
    pub fn write_data(&self, df: &DataFrame, config: WriteConfig) -> Result<(), String> {
        // In production: Write file based on format
        Ok(())
    }

    /// Compute correlation matrix
    pub fn correlation(&self, df: &DataFrame) -> Result<CorrelationMatrix, String> {
        // In production: Compute Pearson correlation between numeric columns
        // For now: Return identity matrix
        let cols = df.columns.clone();
        let n = cols.len();
        let mut matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            matrix[i][i] = 1.0;
        }
        Ok(CorrelationMatrix {
            columns: cols,
            matrix,
        })
    }

    /// Perform linear regression
    pub fn linear_regression(&self, df: &DataFrame, target: &str, features: &[String]) -> Result<RegressionResult, String> {
        // In production: Fit linear regression model
        // For now: Return mock result
        Ok(RegressionResult {
            coefficients: vec![0.0; features.len()],
            intercept: 0.0,
            r_squared: 0.0,
            p_values: vec![1.0; features.len()],
        })
    }

    /// Convert date to Indian calendar
    pub fn convert_to_indian_calendar(&self, date: IndianDate, calendar: IndianCalendar) -> Result<IndianDate, String> {
        match date.calendar {
            IndianCalendar::Gregorian => IndianDate::from_gregorian(calendar, date.year, date.month, date.day),
            _ => {
                // Convert to Gregorian first, then to target calendar
                let gregorian = date.to_gregorian()?;
                IndianDate::from_gregorian(calendar, gregorian.year, gregorian.month, gregorian.day)
            }
        }
    }

    /// Set default calendar
    pub fn set_default_calendar(&mut self, calendar: IndianCalendar) {
        self.default_calendar = calendar;
    }
}

impl Default for DataEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn data_engine_create() -> *mut DataEngine {
    Box::into_raw(Box::new(DataEngine::new()))
}

#[no_mangle]
pub extern "C" fn data_engine_destroy(engine: *mut DataEngine) {
    unsafe {
        if !engine.is_null() {
            let _ = Box::from_raw(engine);
        }
    }
}

#[no_mangle]
pub extern "C" fn dataframe_create() -> *mut DataFrame {
    Box::into_raw(Box::new(DataFrame::new()))
}

#[no_mangle]
pub extern "C" fn dataframe_destroy(df: *mut DataFrame) {
    unsafe {
        if !df.is_null() {
            let _ = Box::from_raw(df);
        }
    }
}

#[no_mangle]
pub extern "C" fn dataframe_add_column(df: *mut DataFrame,
                                      name: *const u8, name_len: usize,
                                      data_type: i32,
                                      values: *const f64,
                                      values_len: usize) -> i32 {
    unsafe {
        if df.is_null() || name.is_null() || values.is_null() { return -1; }
        let name = String::from_utf8_unchecked(
            std::slice::from_raw_parts(name, name_len));
        let values_slice = std::slice::from_raw_parts(values, values_len);
        
        let series = match data_type {
            0 => Series::new(name.clone(), SeriesData::Float64Vec(values_slice.to_vec()), DataType::Float64),
            1 => Series::new(name.clone(), SeriesData::Float32Vec(values_slice.iter().map(|&x| x as f32).collect()), DataType::Float32),
            2 => Series::new(name.clone(), SeriesData::Int64Vec(values_slice.iter().map(|&x| x as i64).collect()), DataType::Int64),
            3 => Series::new(name.clone(), SeriesData::Int32Vec(values_slice.iter().map(|&x| x as i32).collect()), DataType::Int32),
            _ => return -1,
        };
        
        (*df).add_column(series);
        0
    }
}

#[no_mangle]
pub extern "C" fn dataframe_shape(df: *const DataFrame, rows: *mut usize, cols: *mut usize) -> i32 {
    unsafe {
        if df.is_null() || rows.is_null() || cols.is_null() { return -1; }
        let (r, c) = (*df).shape();
        *rows = r;
        *cols = c;
        0
    }
}

#[no_mangle]
pub extern "C" fn dataframe_describe(df: *const DataFrame, out_json: *mut u8, out_len: usize) -> i32 {
    unsafe {
        if df.is_null() { return -1; }
        let result = (*df).describe();
        let json = serde_json::to_string(&result).unwrap_or_default();
        let bytes = json.as_bytes();
        let copy_len = std::cmp::min(bytes.len(), out_len);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_json, copy_len);
        copy_len as i32
    }
}
