extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

/// Datatype enumeration for tabular data columns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColumnType {
    Integer,
    Float,
    String,
    Boolean,
    Null,
}

/// Dynamic value type stored in DataFrame cells
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

impl DataValue {
    pub fn column_type(&self) -> ColumnType {
        match self {
            DataValue::Integer(_) => ColumnType::Integer,
            DataValue::Float(_) => ColumnType::Float,
            DataValue::Text(_) => ColumnType::String,
            DataValue::Boolean(_) => ColumnType::Boolean,
            DataValue::Null => ColumnType::Null,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataValue::Integer(v) => v.to_string(),
            DataValue::Float(v) => format!("{:.4}", v),
            DataValue::Text(s) => s.clone(),
            DataValue::Boolean(b) => b.to_string(),
            DataValue::Null => "NULL".to_string(),
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DataValue::Integer(i) => Some(*i as f64),
            DataValue::Float(f) => Some(*f),
            _ => None,
        }
    }
}

/// Column schema definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnSchema {
    pub name: String,
    pub col_type: ColumnType,
}

/// Data Schema describing DataFrame headers and types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataSchema {
    pub columns: Vec<ColumnSchema>,
}

impl DataSchema {
    pub fn new(columns: Vec<ColumnSchema>) -> Self {
        Self { columns }
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.name == name)
    }
}

/// Data Record representing one row in a DataFrame
#[derive(Debug, Clone, PartialEq)]
pub struct DataRecord {
    pub values: Vec<DataValue>,
}

impl DataRecord {
    pub fn new(values: Vec<DataValue>) -> Self {
        Self { values }
    }

    pub fn get(&self, index: usize) -> Option<&DataValue> {
        self.values.get(index)
    }
}

/// Aggregation operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregateOp {
    Count,
    Sum,
    Mean,
    Min,
    Max,
}

/// Join operation type (Inner, Left, Outer)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    Inner,
    Left,
}

/// DataFrame: Core high-performance in-memory tabular structure inspired by DuckDB and Polars
#[derive(Debug, Clone, PartialEq)]
pub struct DataFrame {
    pub schema: DataSchema,
    pub records: Vec<DataRecord>,
}

impl DataFrame {
    pub fn new(schema: DataSchema, records: Vec<DataRecord>) -> Self {
        Self { schema, records }
    }

    pub fn row_count(&self) -> usize {
        self.records.len()
    }

    pub fn column_count(&self) -> usize {
        self.schema.columns.len()
    }

    /// Auto-infer schema from a CSV formatted string (Miller / DuckDB inspired)
    pub fn from_csv(csv_text: &str, delimiter: char) -> Result<Self, &'static str> {
        let mut lines = csv_text.lines();
        let header_line = lines.next().ok_or("CSV text is empty")?;

        let col_names: Vec<String> = header_line
            .split(delimiter)
            .map(|s| s.trim().to_string())
            .collect();

        let mut raw_rows: Vec<Vec<String>> = Vec::new();
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let row: Vec<String> = trimmed.split(delimiter).map(|s| s.trim().to_string()).collect();
            if row.len() == col_names.len() {
                raw_rows.push(row);
            }
        }

        // Infer column types
        let mut col_schemas: Vec<ColumnSchema> = Vec::new();
        for (col_idx, name) in col_names.iter().enumerate() {
            let mut is_int = true;
            let mut is_float = true;
            let mut is_bool = true;

            for row in &raw_rows {
                if col_idx < row.len() {
                    let val = &row[col_idx];
                    if val.parse::<i64>().is_err() {
                        is_int = false;
                    }
                    if val.parse::<f64>().is_err() {
                        is_float = false;
                    }
                    if val != "true" && val != "false" {
                        is_bool = false;
                    }
                }
            }

            let col_type = if is_int {
                ColumnType::Integer
            } else if is_float {
                ColumnType::Float
            } else if is_bool {
                ColumnType::Boolean
            } else {
                ColumnType::String
            };

            col_schemas.push(ColumnSchema {
                name: name.clone(),
                col_type,
            });
        }

        let schema = DataSchema::new(col_schemas);
        let mut records: Vec<DataRecord> = Vec::new();

        for row in raw_rows {
            let mut vals: Vec<DataValue> = Vec::new();
            for (col_idx, str_val) in row.into_iter().enumerate() {
                let col_type = schema.columns[col_idx].col_type;
                let data_val = match col_type {
                    ColumnType::Integer => {
                        str_val.parse::<i64>().map(DataValue::Integer).unwrap_or(DataValue::Null)
                    }
                    ColumnType::Float => {
                        str_val.parse::<f64>().map(DataValue::Float).unwrap_or(DataValue::Null)
                    }
                    ColumnType::Boolean => {
                        str_val.parse::<bool>().map(DataValue::Boolean).unwrap_or(DataValue::Null)
                    }
                    _ => DataValue::Text(str_val),
                };
                vals.push(data_val);
            }
            records.push(DataRecord::new(vals));
        }

        Ok(DataFrame::new(schema, records))
    }

    /// Select specific columns (Projection)
    pub fn select(&self, col_names: &[&str]) -> Result<DataFrame, &'static str> {
        let mut indices: Vec<usize> = Vec::new();
        let mut new_col_schemas: Vec<ColumnSchema> = Vec::new();

        for name in col_names {
            let idx = self.schema.column_index(name).ok_or("Column not found")?;
            indices.push(idx);
            new_col_schemas.push(self.schema.columns[idx].clone());
        }

        let new_schema = DataSchema::new(new_col_schemas);
        let mut new_records: Vec<DataRecord> = Vec::new();

        for rec in &self.records {
            let mut new_vals: Vec<DataValue> = Vec::new();
            for &idx in &indices {
                new_vals.push(rec.values[idx].clone());
            }
            new_records.push(DataRecord::new(new_vals));
        }

        Ok(DataFrame::new(new_schema, new_records))
    }

    /// Filter rows matching predicate closure
    pub fn filter<F>(&self, predicate: F) -> DataFrame
    where
        F: Fn(&DataRecord, &DataSchema) -> bool,
    {
        let filtered_records: Vec<DataRecord> = self
            .records
            .iter()
            .filter(|rec| predicate(rec, &self.schema))
            .cloned()
            .collect();

        DataFrame::new(self.schema.clone(), filtered_records)
    }

    /// Perform Aggregation by Group (Group By / Aggregation inspired by pandas / SQL)
    pub fn group_by_aggregate(
        &self,
        group_col: &str,
        agg_col: &str,
        op: AggregateOp,
    ) -> Result<DataFrame, &'static str> {
        let g_idx = self.schema.column_index(group_col).ok_or("Group column not found")?;
        let a_idx = self.schema.column_index(agg_col).ok_or("Agg column not found")?;

        let mut groups: HashMap<String, Vec<DataValue>> = HashMap::new();

        for rec in &self.records {
            let key = rec.values[g_idx].to_string();
            let val = rec.values[a_idx].clone();
            groups.entry(key).or_default().push(val);
        }

        let result_schema = DataSchema::new(vec![
            ColumnSchema {
                name: group_col.to_string(),
                col_type: self.schema.columns[g_idx].col_type,
            },
            ColumnSchema {
                name: format!("{:?}_{}", op, agg_col).to_lowercase(),
                col_type: ColumnType::Float,
            },
        ]);

        let mut new_records: Vec<DataRecord> = Vec::new();

        for (key, vals) in groups {
            let agg_result: f64 = match op {
                AggregateOp::Count => vals.len() as f64,
                AggregateOp::Sum => vals.iter().filter_map(|v| v.as_f64()).sum(),
                AggregateOp::Mean => {
                    let nums: Vec<f64> = vals.iter().filter_map(|v| v.as_f64()).collect();
                    if nums.is_empty() {
                        0.0
                    } else {
                        nums.iter().sum::<f64>() / nums.len() as f64
                    }
                }
                AggregateOp::Min => vals
                    .iter()
                    .filter_map(|v| v.as_f64())
                    .fold(f64::INFINITY, f64::min),
                AggregateOp::Max => vals
                    .iter()
                    .filter_map(|v| v.as_f64())
                    .fold(f64::NEG_INFINITY, f64::max),
            };

            new_records.push(DataRecord::new(vec![
                DataValue::Text(key),
                DataValue::Float(agg_result),
            ]));
        }

        Ok(DataFrame::new(result_schema, new_records))
    }

    /// Perform Inner / Left Relational Join between two DataFrames (SQL / Polars parity)
    pub fn join(
        &self,
        other: &DataFrame,
        on_col: &str,
        join_type: JoinType,
    ) -> Result<DataFrame, &'static str> {
        let l_idx = self.schema.column_index(on_col).ok_or("Join column not found in left DataFrame")?;
        let r_idx = other.schema.column_index(on_col).ok_or("Join column not found in right DataFrame")?;

        let mut combined_cols = self.schema.columns.clone();
        for col in &other.schema.columns {
            if col.name != on_col {
                combined_cols.push(col.clone());
            }
        }
        let joined_schema = DataSchema::new(combined_cols);

        let mut right_map: HashMap<String, Vec<&DataRecord>> = HashMap::new();
        for rec in &other.records {
            let key = rec.values[r_idx].to_string();
            right_map.entry(key).or_default().push(rec);
        }

        let mut joined_records: Vec<DataRecord> = Vec::new();

        for l_rec in &self.records {
            let key = l_rec.values[l_idx].to_string();
            if let Some(r_recs) = right_map.get(&key) {
                for r_rec in r_recs {
                    let mut vals = l_rec.values.clone();
                    for (idx, v) in r_rec.values.iter().enumerate() {
                        if idx != r_idx {
                            vals.push(v.clone());
                        }
                    }
                    joined_records.push(DataRecord::new(vals));
                }
            } else if join_type == JoinType::Left {
                let mut vals = l_rec.values.clone();
                for (idx, _) in other.schema.columns.iter().enumerate() {
                    if idx != r_idx {
                        vals.push(DataValue::Null);
                    }
                }
                joined_records.push(DataRecord::new(vals));
            }
        }

        Ok(DataFrame::new(joined_schema, joined_records))
    }

    /// Generates Markdown / Terminal ASCII table representation (VisiData / Miller inspired)
    pub fn to_table_string(&self) -> String {
        let mut out = String::new();

        // Print header
        let headers: Vec<&str> = self.schema.columns.iter().map(|c| c.name.as_str()).collect();
        out.push_str("| ");
        out.push_str(&headers.join(" | "));
        out.push_str(" |\n|");

        for _ in &self.schema.columns {
            out.push_str(" --- |");
        }
        out.push('\n');

        // Print rows
        for rec in &self.records {
            let row_str: Vec<String> = rec.values.iter().map(|v| v.to_string()).collect();
            out.push_str("| ");
            out.push_str(&row_str.join(" | "));
            out.push_str(" |\n");
        }

        out
    }

    /// Converts DataFrame to JSON lines format (jq / ndjson parity)
    pub fn to_ndjson(&self) -> String {
        let mut out = String::new();
        for rec in &self.records {
            out.push('{');
            let mut pairs: Vec<String> = Vec::new();
            for (idx, col) in self.schema.columns.iter().enumerate() {
                let val_str = match &rec.values[idx] {
                    DataValue::Integer(i) => i.to_string(),
                    DataValue::Float(f) => format!("{:.4}", f),
                    DataValue::Boolean(b) => b.to_string(),
                    DataValue::Text(s) => format!("\"{}\"", s),
                    DataValue::Null => "null".to_string(),
                };
                pairs.push(format!("\"{}\":{}", col.name, val_str));
            }
            out.push_str(&pairs.join(","));
            out.push_str("}\n");
        }
        out
    }
}

/// Unified Data Engine Facade for SigmaOS Userland and Shell Execution
pub struct SigmaDataEngine;

impl SigmaDataEngine {
    pub fn parse_csv(csv: &str) -> Result<DataFrame, &'static str> {
        DataFrame::from_csv(csv, ',')
    }

    pub fn parse_tsv(tsv: &str) -> Result<DataFrame, &'static str> {
        DataFrame::from_csv(tsv, '\t')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_parsing_and_schema_inference() {
        let csv_data = "id,name,salary,active\n1,Alice,85000.5,true\n2,Bob,62000.0,false\n3,Charlie,91000.75,true";
        let df = DataFrame::from_csv(csv_data, ',').unwrap();

        assert_eq!(df.row_count(), 3);
        assert_eq!(df.column_count(), 4);

        assert_eq!(df.schema.columns[0].col_type, ColumnType::Integer);
        assert_eq!(df.schema.columns[1].col_type, ColumnType::String);
        assert_eq!(df.schema.columns[2].col_type, ColumnType::Float);
        assert_eq!(df.schema.columns[3].col_type, ColumnType::Boolean);
    }

    #[test]
    fn test_dataframe_select_and_filter() {
        let csv_data = "id,name,salary\n1,Alice,85000\n2,Bob,62000\n3,Charlie,91000";
        let df = DataFrame::from_csv(csv_data, ',').unwrap();

        let projected = df.select(&["name", "salary"]).unwrap();
        assert_eq!(projected.column_count(), 2);

        let filtered = df.filter(|rec, schema| {
            let idx = schema.column_index("salary").unwrap();
            rec.values[idx].as_f64().unwrap_or(0.0) > 70000.0
        });

        assert_eq!(filtered.row_count(), 2);
    }

    #[test]
    fn test_dataframe_group_by_and_aggregation() {
        let csv_data = "department,salary\nEng,100\nEng,120\nSales,80\nSales,90";
        let df = DataFrame::from_csv(csv_data, ',').unwrap();

        let agg = df.group_by_aggregate("department", "salary", AggregateOp::Mean).unwrap();
        assert_eq!(agg.row_count(), 2);
    }

    #[test]
    fn test_dataframe_join_and_table_rendering() {
        let csv1 = "id,dept\n1,Eng\n2,Sales";
        let csv2 = "dept,budget\nEng,500000\nSales,250000";

        let df1 = DataFrame::from_csv(csv1, ',').unwrap();
        let df2 = DataFrame::from_csv(csv2, ',').unwrap();

        let joined = df1.join(&df2, "dept", JoinType::Inner).unwrap();
        assert_eq!(joined.row_count(), 2);
        assert_eq!(joined.column_count(), 3);

        let table = joined.to_table_string();
        assert!(table.contains("Eng"));
        assert!(table.contains("500000"));
    }
}
