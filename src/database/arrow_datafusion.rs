// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Apache Arrow zero-copy memory model + DataFusion query plans
// Principles: ZERO-COPY, CACHE CONSCIOUS DESIGN, LOCALITY OF REFERENCE

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

/// Apache Arrow data types
#[derive(Debug, Clone, PartialEq)]
pub enum ArrowDataType {
    Null,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Utf8,
    LargeUtf8,
    Binary,
    LargeBinary,
    Date32,
    Date64,
    Timestamp {
        tz: Option<String>,
    },
    Duration,
    Interval,
    List(Box<ArrowDataType>),
    Struct(Vec<ArrowField>),
    Dictionary {
        key: Box<ArrowDataType>,
        value: Box<ArrowDataType>,
    },
}

impl ArrowDataType {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Self::Int8
                | Self::Int16
                | Self::Int32
                | Self::Int64
                | Self::UInt8
                | Self::UInt16
                | Self::UInt32
                | Self::UInt64
                | Self::Float32
                | Self::Float64
        )
    }
    pub fn byte_width(&self) -> usize {
        match self {
            Self::Bool | Self::Int8 | Self::UInt8 => 1,
            Self::Int16 | Self::UInt16 | Self::Float16 => 2,
            Self::Int32 | Self::UInt32 | Self::Float32 | Self::Date32 => 4,
            Self::Int64 | Self::UInt64 | Self::Float64 | Self::Date64 | Self::Timestamp { .. } => 8,
            _ => 0,
        }
    }
}

/// Arrow field definition
#[derive(Debug, Clone, PartialEq)]
pub struct ArrowField {
    pub name: String,
    pub data_type: ArrowDataType,
    pub nullable: bool,
    pub metadata: BTreeMap<String, String>,
}

impl ArrowField {
    pub fn new(name: &str, dt: ArrowDataType, nullable: bool) -> Self {
        Self {
            name: name.into(),
            data_type: dt,
            nullable,
            metadata: BTreeMap::new(),
        }
    }
}

/// Arrow schema — ordered collection of fields
#[derive(Debug, Clone)]
pub struct ArrowSchema {
    pub fields: Vec<ArrowField>,
    pub metadata: BTreeMap<String, String>,
}

impl ArrowSchema {
    pub fn new(fields: Vec<ArrowField>) -> Self {
        Self {
            fields,
            metadata: BTreeMap::new(),
        }
    }
    pub fn field(&self, name: &str) -> Option<&ArrowField> {
        self.fields.iter().find(|f| f.name == name)
    }
    pub fn field_index(&self, name: &str) -> Option<usize> {
        self.fields.iter().position(|f| f.name == name)
    }
    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }
}

/// Arrow array — typed columnar buffer
#[derive(Debug, Clone)]
pub struct ArrowArray {
    pub data_type: ArrowDataType,
    pub length: usize,
    pub null_count: usize,
    pub validity_bitmap: Vec<u8>, // bit-packed null bitmap
    pub data_buffer: Vec<u8>,     // raw value bytes
    pub offsets_buffer: Vec<i32>, // for variable-length types
    pub child_arrays: Vec<ArrowArray>,
}

impl ArrowArray {
    pub fn new_empty(dt: ArrowDataType) -> Self {
        Self {
            data_type: dt,
            length: 0,
            null_count: 0,
            validity_bitmap: Vec::new(),
            data_buffer: Vec::new(),
            offsets_buffer: Vec::new(),
            child_arrays: Vec::new(),
        }
    }

    pub fn from_i64_slice(data: &[i64]) -> Self {
        let mut buf = Vec::with_capacity(data.len() * 8);
        for &v in data {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        Self {
            data_type: ArrowDataType::Int64,
            length: data.len(),
            null_count: 0,
            validity_bitmap: Vec::new(),
            data_buffer: buf,
            offsets_buffer: Vec::new(),
            child_arrays: Vec::new(),
        }
    }

    pub fn from_f64_slice(data: &[f64]) -> Self {
        let mut buf = Vec::with_capacity(data.len() * 8);
        for &v in data {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        Self {
            data_type: ArrowDataType::Float64,
            length: data.len(),
            null_count: 0,
            validity_bitmap: Vec::new(),
            data_buffer: buf,
            offsets_buffer: Vec::new(),
            child_arrays: Vec::new(),
        }
    }

    pub fn is_null(&self, idx: usize) -> bool {
        if self.validity_bitmap.is_empty() {
            return false;
        }
        let byte = self.validity_bitmap.get(idx / 8).copied().unwrap_or(0xFF);
        (byte >> (idx % 8)) & 1 == 0
    }

    pub fn get_i64(&self, idx: usize) -> Option<i64> {
        if self.data_type != ArrowDataType::Int64 || idx >= self.length {
            return None;
        }
        let start = idx * 8;
        self.data_buffer
            .get(start..start + 8)
            .map(|b| i64::from_le_bytes(b.try_into().unwrap_or([0; 8])))
    }

    pub fn null_ratio(&self) -> f64 {
        if self.length == 0 {
            0.0
        } else {
            self.null_count as f64 / self.length as f64
        }
    }
}

/// Arrow record batch — multiple arrays with shared schema
#[derive(Debug, Clone)]
pub struct ArrowRecordBatch {
    pub schema: ArrowSchema,
    pub columns: Vec<ArrowArray>,
    pub num_rows: usize,
}

impl ArrowRecordBatch {
    pub fn new(schema: ArrowSchema, columns: Vec<ArrowArray>) -> Result<Self, &'static str> {
        if schema.num_fields() != columns.len() {
            return Err("schema/columns length mismatch");
        }
        let num_rows = columns.first().map(|c| c.length).unwrap_or(0);
        if columns.iter().any(|c| c.length != num_rows) {
            return Err("column length mismatch");
        }
        Ok(Self {
            schema,
            columns,
            num_rows,
        })
    }

    pub fn column(&self, name: &str) -> Option<&ArrowArray> {
        let idx = self.schema.field_index(name)?;
        self.columns.get(idx)
    }

    pub fn slice(&self, offset: usize, length: usize) -> Self {
        // Stub: real impl would slice each column buffer
        Self {
            schema: self.schema.clone(),
            columns: self.columns.clone(),
            num_rows: length.min(self.num_rows.saturating_sub(offset)),
        }
    }
}

/// IPC buffer for zero-copy inter-process Arrow transfer
#[derive(Debug)]
pub struct IpcBuffer {
    pub data: Vec<u8>,
    pub schema_offset: usize,
    pub body_offset: usize,
}

impl IpcBuffer {
    pub fn serialize(batch: &ArrowRecordBatch) -> Self {
        let schema_bytes =
            alloc::format!("schema:fields={}", batch.schema.num_fields()).into_bytes();
        let body_bytes: Vec<u8> = batch
            .columns
            .iter()
            .flat_map(|c| c.data_buffer.iter().copied())
            .collect();
        let schema_offset = 0;
        let body_offset = schema_bytes.len();
        let mut data = schema_bytes;
        data.extend_from_slice(&body_bytes);
        Self {
            data,
            schema_offset,
            body_offset,
        }
    }

    pub fn schema_bytes(&self) -> &[u8] {
        &self.data[self.schema_offset..self.body_offset]
    }
    pub fn body_bytes(&self) -> &[u8] {
        &self.data[self.body_offset..]
    }
    pub fn total_size(&self) -> usize {
        self.data.len()
    }
}

// ── DataFusion Query Plans ────────────────────────────────────────────────────

/// Logical query plan node
#[derive(Debug, Clone)]
pub enum LogicalPlan {
    Scan {
        table: String,
        projection: Vec<String>,
    },
    Project {
        input: Box<LogicalPlan>,
        exprs: Vec<String>,
    },
    Filter {
        input: Box<LogicalPlan>,
        predicate: String,
    },
    Aggregate {
        input: Box<LogicalPlan>,
        group_by: Vec<String>,
        aggs: Vec<String>,
    },
    Join {
        left: Box<LogicalPlan>,
        right: Box<LogicalPlan>,
        condition: String,
        join_type: JoinType,
    },
    Sort {
        input: Box<LogicalPlan>,
        exprs: Vec<(String, bool)>,
    },
    Limit {
        input: Box<LogicalPlan>,
        n: usize,
    },
    Union {
        plans: Vec<LogicalPlan>,
    },
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinType {
    Inner,
    LeftOuter,
    RightOuter,
    Full,
    Cross,
    LeftSemi,
    LeftAnti,
}

/// Physical query plan node
#[derive(Debug, Clone)]
pub enum PhysicalPlan {
    TableScan {
        table: String,
        projected_cols: Vec<String>,
    },
    HashProbe {
        build: Box<PhysicalPlan>,
        probe: Box<PhysicalPlan>,
        keys: Vec<String>,
    },
    VectorizedFilter {
        input: Box<PhysicalPlan>,
        expr: String,
    },
    HashAggregate {
        input: Box<PhysicalPlan>,
        keys: Vec<String>,
        aggs: Vec<String>,
    },
    MergeSort {
        inputs: Vec<PhysicalPlan>,
        sort_keys: Vec<String>,
    },
    ProjectionExec {
        input: Box<PhysicalPlan>,
        exprs: Vec<String>,
    },
    Limit {
        input: Box<PhysicalPlan>,
        n: usize,
    },
}

/// DataFusion execution context
pub struct DataFusionContext {
    pub tables: BTreeMap<String, ArrowSchema>,
    pub catalog: BTreeMap<String, String>, // alias → table name
}

impl DataFusionContext {
    pub fn new() -> Self {
        Self {
            tables: BTreeMap::new(),
            catalog: BTreeMap::new(),
        }
    }

    pub fn register_table(&mut self, name: &str, schema: ArrowSchema) {
        self.tables.insert(name.into(), schema);
    }

    pub fn register_alias(&mut self, alias: &str, table: &str) {
        self.catalog.insert(alias.into(), table.into());
    }

    /// Parse SQL string into a logical plan (stub)
    pub fn sql(&self, sql: &str) -> Result<LogicalPlan, &'static str> {
        let upper = sql.to_ascii_uppercase();
        if upper.starts_with("SELECT") {
            let table = sql
                .split_whitespace()
                .skip_while(|w| !w.eq_ignore_ascii_case("FROM"))
                .nth(1)
                .unwrap_or("unknown");
            Ok(LogicalPlan::Scan {
                table: table.into(),
                projection: alloc::vec!["*".into()],
            })
        } else {
            Err("only SELECT supported in stub")
        }
    }

    /// Optimize a logical plan (stub — returns as-is)
    pub fn optimize(&self, plan: LogicalPlan) -> LogicalPlan {
        plan
    }

    /// Convert logical plan to physical plan (stub)
    pub fn plan_to_physical(&self, plan: &LogicalPlan) -> PhysicalPlan {
        match plan {
            LogicalPlan::Scan { table, projection } => PhysicalPlan::TableScan {
                table: table.clone(),
                projected_cols: projection.clone(),
            },
            LogicalPlan::Filter { input, predicate } => {
                let phys_input = self.plan_to_physical(input);
                PhysicalPlan::VectorizedFilter {
                    input: Box::new(phys_input),
                    expr: predicate.clone(),
                }
            }
            _ => PhysicalPlan::TableScan {
                table: "empty".into(),
                projected_cols: Vec::new(),
            },
        }
    }

    /// Execute a physical plan (stub — returns empty batch)
    pub fn execute(&self, plan: &PhysicalPlan) -> Vec<ArrowRecordBatch> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_array_i64() {
        let arr = ArrowArray::from_i64_slice(&[10, 20, 30]);
        assert_eq!(arr.length, 3);
        assert_eq!(arr.get_i64(1), Some(20));
    }

    #[test]
    fn test_record_batch_schema_mismatch() {
        let schema = ArrowSchema::new(alloc::vec![ArrowField::new(
            "x",
            ArrowDataType::Int64,
            false
        )]);
        let result = ArrowRecordBatch::new(schema, alloc::vec![]); // 1 field, 0 columns
        assert!(result.is_err());
    }

    #[test]
    fn test_ipc_buffer() {
        let schema = ArrowSchema::new(alloc::vec![ArrowField::new(
            "v",
            ArrowDataType::Float64,
            false
        )]);
        let arr = ArrowArray::from_f64_slice(&[1.0, 2.0]);
        let batch = ArrowRecordBatch::new(schema, alloc::vec![arr]).unwrap();
        let buf = IpcBuffer::serialize(&batch);
        assert!(!buf.schema_bytes().is_empty());
        assert!(!buf.body_bytes().is_empty()); // 2 × 8 bytes
        assert_eq!(buf.body_bytes().len(), 16);
    }

    #[test]
    fn test_datafusion_sql_parse() {
        let mut ctx = DataFusionContext::new();
        ctx.register_table(
            "orders",
            ArrowSchema::new(alloc::vec![
                ArrowField::new("id", ArrowDataType::Int64, false),
                ArrowField::new("amount", ArrowDataType::Float64, true),
            ]),
        );
        let plan = ctx.sql("SELECT id, amount FROM orders").unwrap();
        assert!(matches!(plan, LogicalPlan::Scan { .. }));
    }

    #[test]
    fn test_arrow_data_type_helpers() {
        assert!(ArrowDataType::Int64.is_numeric());
        assert!(!ArrowDataType::Utf8.is_numeric());
        assert_eq!(ArrowDataType::Float64.byte_width(), 8);
    }
}
