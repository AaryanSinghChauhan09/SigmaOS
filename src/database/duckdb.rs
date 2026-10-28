// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - DuckDB vectorized columnar OLAP engine
// Principles: ZERO-COPY, CACHE CONSCIOUS DESIGN, LOCALITY OF REFERENCE, MODULARITY

use alloc::{vec::Vec, string::String, collections::BTreeMap, format};

/// Column data types (mirrors DuckDB's type system)
#[derive(Debug, Clone, PartialEq)]
pub enum DuckType {
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    Float32, Float64,
    Varchar, Boolean, Blob,
    Date, Timestamp, Interval,
    List(alloc::boxed::Box<DuckType>),
    Struct(Vec<(String, DuckType)>),
    Null,
}

/// Runtime values
#[derive(Debug, Clone, PartialEq)]
pub enum DuckValue {
    Null,
    Int64(i64),
    UInt64(u64),
    Float64(f64),
    Varchar(String),
    Bool(bool),
    Blob(Vec<u8>),
    Date(i32),       // days since epoch
    Timestamp(i64),  // microseconds since epoch
}

impl DuckValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Null => "NULL", Self::Int64(_) => "BIGINT", Self::UInt64(_) => "UBIGINT",
            Self::Float64(_) => "DOUBLE", Self::Varchar(_) => "VARCHAR", Self::Bool(_) => "BOOLEAN",
            Self::Blob(_) => "BLOB", Self::Date(_) => "DATE", Self::Timestamp(_) => "TIMESTAMP",
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self { Self::Int64(n) => Some(*n), Self::UInt64(n) => Some(*n as i64), Self::Float64(f) => Some(*f as i64), _ => None }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self { Self::Float64(f) => Some(*f), Self::Int64(n) => Some(*n as f64), _ => None }
    }
}

/// A columnar data vector (arrow-style)
#[derive(Debug, Clone)]
pub struct DuckVector {
    pub name: String,
    pub dtype: DuckType,
    pub data: Vec<DuckValue>,
    pub validity: Vec<bool>,  // null bitmap
}

impl DuckVector {
    pub fn new(name: &str, dtype: DuckType) -> Self {
        Self { name: name.into(), dtype, data: Vec::new(), validity: Vec::new() }
    }

    pub fn push(&mut self, val: DuckValue) {
        let valid = val != DuckValue::Null;
        self.validity.push(valid);
        self.data.push(val);
    }

    pub fn is_null(&self, idx: usize) -> bool { !self.validity.get(idx).copied().unwrap_or(false) }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }
}

/// Record batch — columnar in-memory table
#[derive(Debug, Clone)]
pub struct DuckRecordBatch {
    pub schema: Vec<(String, DuckType)>,
    pub columns: Vec<DuckVector>,
    pub row_count: usize,
}

impl DuckRecordBatch {
    pub fn new(schema: Vec<(String, DuckType)>) -> Self {
        let columns = schema.iter()
            .map(|(n, t)| DuckVector::new(n, t.clone()))
            .collect();
        Self { schema, columns, row_count: 0 }
    }

    pub fn append_row(&mut self, values: Vec<DuckValue>) -> Result<(), &'static str> {
        if values.len() != self.columns.len() { return Err("column count mismatch"); }
        for (col, val) in self.columns.iter_mut().zip(values) { col.push(val); }
        self.row_count += 1;
        Ok(())
    }

    pub fn get_column(&self, name: &str) -> Option<&DuckVector> {
        self.columns.iter().find(|c| c.name == name)
    }

    pub fn slice(&self, offset: usize, length: usize) -> DuckRecordBatch {
        let mut out = DuckRecordBatch::new(self.schema.clone());
        for _ in offset..(offset + length).min(self.row_count) {
            // stub: real impl copies rows
        }
        out
    }
}

/// Aggregation functions
#[derive(Debug, Clone)]
pub enum AggFunc { Sum, Count, CountDistinct, Avg, Min, Max, First, Last }

/// Comparison predicates for filter pushdown
#[derive(Debug, Clone)]
pub enum FilterOp { Eq, Ne, Lt, Lte, Gt, Gte, IsNull, IsNotNull, Like }

#[derive(Debug, Clone)]
pub struct FilterExpr {
    pub column: String,
    pub op: FilterOp,
    pub value: DuckValue,
}

impl FilterExpr {
    pub fn matches(&self, val: &DuckValue) -> bool {
        match &self.op {
            FilterOp::IsNull => *val == DuckValue::Null,
            FilterOp::IsNotNull => *val != DuckValue::Null,
            FilterOp::Eq => val == &self.value,
            FilterOp::Ne => val != &self.value,
            FilterOp::Gt => val.as_i64().zip(self.value.as_i64()).map(|(a,b)| a > b).unwrap_or(false),
            FilterOp::Gte => val.as_i64().zip(self.value.as_i64()).map(|(a,b)| a >= b).unwrap_or(false),
            FilterOp::Lt => val.as_i64().zip(self.value.as_i64()).map(|(a,b)| a < b).unwrap_or(false),
            FilterOp::Lte => val.as_i64().zip(self.value.as_i64()).map(|(a,b)| a <= b).unwrap_or(false),
            FilterOp::Like => {
                if let (DuckValue::Varchar(s), DuckValue::Varchar(p)) = (val, &self.value) {
                    s.contains(p.as_str())
                } else { false }
            }
        }
    }
}

/// Window function specification
#[derive(Debug, Clone)]
pub struct WindowSpec {
    pub partition_by: Vec<String>,
    pub order_by: Vec<(String, bool)>,  // (column, ascending)
    pub func: AggFunc,
    pub target_col: String,
}

/// DuckDB in-process OLAP engine
pub struct DuckDbEngine {
    pub tables: BTreeMap<String, DuckRecordBatch>,
    pub views: BTreeMap<String, String>,  // name → SQL definition
}

impl DuckDbEngine {
    pub fn new() -> Self {
        Self { tables: BTreeMap::new(), views: BTreeMap::new() }
    }

    pub fn create_table(&mut self, name: &str, schema: Vec<(String, DuckType)>) -> Result<(), &'static str> {
        if self.tables.contains_key(name) { return Err("table already exists"); }
        self.tables.insert(name.into(), DuckRecordBatch::new(schema));
        Ok(())
    }

    pub fn drop_table(&mut self, name: &str) -> Result<(), &'static str> {
        self.tables.remove(name).ok_or("table not found").map(|_| ())
    }

    pub fn insert_into(&mut self, table: &str, row: Vec<DuckValue>) -> Result<(), &'static str> {
        self.tables.get_mut(table).ok_or("table not found")?.append_row(row)
    }

    pub fn select_all(&self, table: &str) -> Result<&DuckRecordBatch, &'static str> {
        self.tables.get(table).ok_or("table not found")
    }

    /// Filter a record batch using predicate pushdown
    pub fn filter(&self, batch: &DuckRecordBatch, exprs: &[FilterExpr]) -> DuckRecordBatch {
        let mut out = DuckRecordBatch::new(batch.schema.clone());
        for row_idx in 0..batch.row_count {
            let passes = exprs.iter().all(|expr| {
                batch.get_column(&expr.column)
                    .and_then(|col| col.data.get(row_idx))
                    .map(|v| expr.matches(v))
                    .unwrap_or(false)
            });
            if passes {
                let row: Vec<DuckValue> = batch.columns.iter()
                    .map(|c| c.data.get(row_idx).cloned().unwrap_or(DuckValue::Null))
                    .collect();
                let _ = out.append_row(row);
            }
        }
        out
    }

    /// Aggregate a column
    pub fn aggregate(&self, table: &str, agg: AggFunc, col: &str) -> Option<DuckValue> {
        let batch = self.tables.get(table)?;
        let column = batch.get_column(col)?;
        match agg {
            AggFunc::Count => Some(DuckValue::Int64(column.len() as i64)),
            AggFunc::Sum => {
                let sum: f64 = column.data.iter().filter_map(|v| v.as_f64()).sum();
                Some(DuckValue::Float64(sum))
            }
            AggFunc::Avg => {
                let vals: Vec<f64> = column.data.iter().filter_map(|v| v.as_f64()).collect();
                if vals.is_empty() { None } else { Some(DuckValue::Float64(vals.iter().sum::<f64>() / vals.len() as f64)) }
            }
            AggFunc::Min => column.data.iter().filter_map(|v| v.as_i64()).min().map(DuckValue::Int64),
            AggFunc::Max => column.data.iter().filter_map(|v| v.as_i64()).max().map(DuckValue::Int64),
            _ => Some(DuckValue::Null),
        }
    }

    /// Scan from a Parquet file path (stub — returns empty batch)
    pub fn scan_parquet(&self, _path: &str, schema: Vec<(String, DuckType)>) -> DuckRecordBatch {
        DuckRecordBatch::new(schema)
    }

    /// Scan from a CSV file (stub)
    pub fn scan_csv(&self, _path: &str, schema: Vec<(String, DuckType)>, _delimiter: char) -> DuckRecordBatch {
        DuckRecordBatch::new(schema)
    }

    pub fn create_view(&mut self, name: &str, sql: &str) {
        self.views.insert(name.into(), sql.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_db() -> DuckDbEngine {
        let mut db = DuckDbEngine::new();
        db.create_table("users", alloc::vec![
            ("id".into(), DuckType::Int64),
            ("name".into(), DuckType::Varchar),
            ("age".into(), DuckType::Int64),
        ]).unwrap();
        db.insert_into("users", alloc::vec![DuckValue::Int64(1), DuckValue::Varchar("Alice".into()), DuckValue::Int64(30)]).unwrap();
        db.insert_into("users", alloc::vec![DuckValue::Int64(2), DuckValue::Varchar("Bob".into()), DuckValue::Int64(25)]).unwrap();
        db.insert_into("users", alloc::vec![DuckValue::Int64(3), DuckValue::Varchar("Carol".into()), DuckValue::Int64(35)]).unwrap();
        db
    }

    #[test]
    fn test_create_insert_select() {
        let db = make_db();
        let batch = db.select_all("users").unwrap();
        assert_eq!(batch.row_count, 3);
    }

    #[test]
    fn test_filter() {
        let db = make_db();
        let batch = db.select_all("users").unwrap();
        let filtered = db.filter(batch, &[FilterExpr { column: "age".into(), op: FilterOp::Gt, value: DuckValue::Int64(28) }]);
        assert_eq!(filtered.row_count, 2); // Alice(30) and Carol(35)
    }

    #[test]
    fn test_aggregate_count() {
        let db = make_db();
        let count = db.aggregate("users", AggFunc::Count, "id").unwrap();
        assert_eq!(count, DuckValue::Int64(3));
    }

    #[test]
    fn test_aggregate_sum() {
        let db = make_db();
        let sum = db.aggregate("users", AggFunc::Sum, "age").unwrap();
        assert_eq!(sum, DuckValue::Float64(90.0));
    }

    #[test]
    fn test_duplicate_table_fails() {
        let mut db = DuckDbEngine::new();
        db.create_table("t", alloc::vec![("x".into(), DuckType::Int64)]).unwrap();
        assert!(db.create_table("t", alloc::vec![("x".into(), DuckType::Int64)]).is_err());
    }

    #[test]
    fn test_filter_like() {
        let db = make_db();
        let batch = db.select_all("users").unwrap();
        let filtered = db.filter(batch, &[FilterExpr { column: "name".into(), op: FilterOp::Like, value: DuckValue::Varchar("li".into()) }]);
        assert_eq!(filtered.row_count, 1); // only "Alice" contains "li"
    }
}
