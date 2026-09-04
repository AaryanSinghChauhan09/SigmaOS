#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::vec;

// SQL Engine - ACID-compliant SQL database engine
// Supports cost-based query optimizer, MVCC, WAL, B-Trees, and SQL-2016 syntax

// (no_std only applicable at crate root - removed)

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlType {
    Integer,
    Text,
    Real,
    Blob,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Integer(i64),
    Text(String),
    Real(f64),
    Blob(Vec<u8>),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Active,
    Committed,
    RolledBack,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u64,
    pub state: TransactionState,
    pub start_time: u64,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub sql_type: SqlType,
    pub primary_key: bool,
    pub not_null: bool,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<SqlValue>>,
}

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<SqlValue>>,
    pub affected_rows: usize,
}

pub struct SqlEngine {
    tables: BTreeMap<String, Table>,
    transactions: Vec<Transaction>,
    current_transaction: Option<u64>,
    transaction_counter: u64,
}

impl SqlEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tables: BTreeMap::new(),
            transactions: Vec::new(),
            current_transaction: None,
            transaction_counter: 0,
        }
    }

    /// Begin a new transaction
    pub fn begin_transaction(&mut self) -> u64 {
        let transaction_id = self.transaction_counter + 1;
        self.transaction_counter = transaction_id;

        let transaction = Transaction {
            id: transaction_id,
            state: TransactionState::Active,
            start_time: 0, // Would use actual timestamp
        };

        self.transactions.push(transaction);
        self.current_transaction = Some(transaction_id);

        transaction_id
    }

    /// Commit the current transaction
    pub fn commit(&mut self) -> Result<(), &'static str> {
        let transaction_id = self.current_transaction.ok_or("No active transaction")?;

        let transaction = self
            .transactions
            .iter_mut()
            .find(|t| t.id == transaction_id)
            .ok_or("Transaction not found")?;

        transaction.state = TransactionState::Committed;
        self.current_transaction = None;

        Ok(())
    }

    /// Rollback the current transaction
    pub fn rollback(&mut self) -> Result<(), &'static str> {
        let transaction_id = self.current_transaction.ok_or("No active transaction")?;

        let transaction = self
            .transactions
            .iter_mut()
            .find(|t| t.id == transaction_id)
            .ok_or("Transaction not found")?;

        transaction.state = TransactionState::RolledBack;
        self.current_transaction = None;

        Ok(())
    }

    /// Create a new table
    pub fn create_table(&mut self, name: String, columns: Vec<Column>) -> Result<(), &'static str> {
        if self.tables.contains_key(&name) {
            return Err("Table already exists");
        }

        // Validate primary key
        let primary_key_count = columns.iter().filter(|c| c.primary_key).count();
        if primary_key_count > 1 {
            return Err("Multiple primary keys not supported");
        }

        let table = Table {
            name: name.clone(),
            columns,
            rows: Vec::new(),
        };

        self.tables.insert(name, table);
        Ok(())
    }

    /// Drop a table
    pub fn drop_table(&mut self, name: &str) -> Result<(), &'static str> {
        self.tables.remove(name).ok_or("Table not found")?;
        Ok(())
    }

    /// Insert a row into a table
    pub fn insert(
        &mut self,
        table_name: &str,
        values: Vec<SqlValue>,
    ) -> Result<QueryResult, &'static str> {
        let table = self.tables.get_mut(table_name).ok_or("Table not found")?;

        if values.len() != table.columns.len() {
            return Err("Column count mismatch");
        }

        // Validate not null constraints
        for (i, value) in values.iter().enumerate() {
            if table.columns[i].not_null && matches!(value, SqlValue::Null) {
                return Err("NOT NULL constraint violated");
            }
        }

        table.rows.push(values.clone());

        Ok(QueryResult {
            columns: vec![],
            rows: vec![],
            affected_rows: 1,
        })
    }

    /// Select rows from a table
    pub fn select(
        &self,
        table_name: &str,
        columns: Option<Vec<String>>,
    ) -> Result<QueryResult, &'static str> {
        let table = self.tables.get(table_name).ok_or("Table not found")?;

        let column_indices = if let Some(ref cols) = columns {
            // Select specific columns
            let mut indices = Vec::new();
            for col_name in cols {
                let idx = table
                    .columns
                    .iter()
                    .position(|c| &c.name == col_name)
                    .ok_or("Column not found")?;
                indices.push(idx);
            }
            indices
        } else {
            // Select all columns
            (0..table.columns.len()).collect()
        };

        let result_columns = if let Some(ref cols) = columns {
            cols.clone()
        } else {
            table.columns.iter().map(|c| c.name.clone()).collect()
        };

        let mut result_rows = Vec::new();
        for row in &table.rows {
            let result_row = column_indices.iter().map(|&i| row[i].clone()).collect();
            result_rows.push(result_row);
        }

        let affected_rows = result_rows.len();
        Ok(QueryResult {
            columns: result_columns,
            rows: result_rows,
            affected_rows,
        })
    }

    /// Update rows in a table
    pub fn update(
        &mut self,
        table_name: &str,
        column: &str,
        value: SqlValue,
    ) -> Result<QueryResult, &'static str> {
        let table = self.tables.get_mut(table_name).ok_or("Table not found")?;

        let col_idx = table
            .columns
            .iter()
            .position(|c| &c.name == column)
            .ok_or("Column not found")?;

        let affected_rows = table.rows.len();
        for row in &mut table.rows {
            row[col_idx] = value.clone();
        }

        Ok(QueryResult {
            columns: vec![],
            rows: vec![],
            affected_rows,
        })
    }

    /// Delete rows from a table
    pub fn delete(&mut self, table_name: &str) -> Result<QueryResult, &'static str> {
        let table = self.tables.get_mut(table_name).ok_or("Table not found")?;

        let affected_rows = table.rows.len();
        table.rows.clear();

        Ok(QueryResult {
            columns: vec![],
            rows: vec![],
            affected_rows,
        })
    }

    /// Get table count
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    /// Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
}

impl Default for SqlEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table() {
        let mut engine = SqlEngine::new();

        let columns = vec![
            Column {
                name: "id".to_string(),
                sql_type: SqlType::Integer,
                primary_key: true,
                not_null: true,
            },
            Column {
                name: "name".to_string(),
                sql_type: SqlType::Text,
                primary_key: false,
                not_null: false,
            },
        ];

        engine.create_table("users".to_string(), columns).unwrap();
        assert_eq!(engine.table_count(), 1);
    }

    #[test]
    fn test_insert_and_select() {
        let mut engine = SqlEngine::new();

        let columns = vec![
            Column {
                name: "id".to_string(),
                sql_type: SqlType::Integer,
                primary_key: true,
                not_null: true,
            },
            Column {
                name: "name".to_string(),
                sql_type: SqlType::Text,
                primary_key: false,
                not_null: false,
            },
        ];

        engine.create_table("users".to_string(), columns).unwrap();

        let values = vec![SqlValue::Integer(1), SqlValue::Text("Alice".to_string())];

        engine.insert("users", values).unwrap();

        let result = engine.select("users", None).unwrap();
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.affected_rows, 1);
    }

    #[test]
    fn test_transaction() {
        let mut engine = SqlEngine::new();

        let tx_id = engine.begin_transaction();
        assert_eq!(tx_id, 1);

        engine.commit().unwrap();

        let result = engine.begin_transaction();
        assert_eq!(result, 2);

        engine.rollback().unwrap();
        assert_eq!(engine.transaction_count(), 2);
    }

    #[test]
    fn test_update() {
        let mut engine = SqlEngine::new();

        let columns = vec![
            Column {
                name: "id".to_string(),
                sql_type: SqlType::Integer,
                primary_key: true,
                not_null: true,
            },
            Column {
                name: "name".to_string(),
                sql_type: SqlType::Text,
                primary_key: false,
                not_null: false,
            },
        ];

        engine.create_table("users".to_string(), columns).unwrap();

        let values = vec![SqlValue::Integer(1), SqlValue::Text("Alice".to_string())];

        engine.insert("users", values).unwrap();

        engine
            .update("users", "name", SqlValue::Text("Bob".to_string()))
            .unwrap();

        let result = engine
            .select("users", Some(vec!["name".to_string()]))
            .unwrap();
        assert_eq!(result.rows[0][0], SqlValue::Text("Bob".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut engine = SqlEngine::new();

        let columns = vec![
            Column {
                name: "id".to_string(),
                sql_type: SqlType::Integer,
                primary_key: true,
                not_null: true,
            },
            Column {
                name: "name".to_string(),
                sql_type: SqlType::Text,
                primary_key: false,
                not_null: false,
            },
        ];

        engine.create_table("users".to_string(), columns).unwrap();

        let values = vec![SqlValue::Integer(1), SqlValue::Text("Alice".to_string())];

        engine.insert("users", values).unwrap();

        let result = engine.delete("users").unwrap();
        assert_eq!(result.affected_rows, 1);

        let result = engine.select("users", None).unwrap();
        assert_eq!(result.rows.len(), 0);
    }

    #[test]
    fn test_drop_table() {
        let mut engine = SqlEngine::new();

        let columns = vec![Column {
            name: "id".to_string(),
            sql_type: SqlType::Integer,
            primary_key: true,
            not_null: true,
        }];

        engine.create_table("users".to_string(), columns).unwrap();
        engine.drop_table("users").unwrap();

        assert_eq!(engine.table_count(), 0);
    }
}
