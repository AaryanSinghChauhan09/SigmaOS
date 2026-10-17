// SPDX-License-Identifier: MIT
// SigmaOS - PostgreSQL-inspired Database System
// Provides relational database capabilities with SQL support

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// Column type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    BigInt,
    Real,
    DoublePrecision,
    Text,
    Varchar(usize),
    Boolean,
    Date,
    Timestamp,
    UUID,
    JSON,
    Binary,
}

/// Column
#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub primary_key: bool,
    pub unique: bool,
    pub default_value: Option<String>,
}

/// Table
#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub next_row_id: AtomicU32,
}

impl Clone for Table {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            columns: self.columns.clone(),
            next_row_id: AtomicU32::new(self.next_row_id.load(Ordering::Relaxed)),
        }
    }
}

/// Value
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Integer(i32),
    BigInt(i64),
    Real(f32),
    DoublePrecision(f64),
    Text(String),
    Boolean(bool),
    Timestamp(i64),
    UUID([u8; 16]),
    JSON(String),
    Binary(Vec<u8>),
}

/// Row
pub type Row = BTreeMap<String, Value>;

/// Index type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexType {
    BTree,
    Hash,
    GIN,
    GiST,
}

/// Index
#[derive(Debug, Clone)]
pub struct Index {
    pub name: String,
    pub table_name: String,
    pub columns: Vec<String>,
    pub index_type: IndexType,
    pub unique: bool,
}

/// Constraint type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
    NotNull,
}

/// Constraint
#[derive(Debug, Clone)]
pub struct Constraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub table_name: String,
    pub columns: Vec<String>,
    pub reference_table: Option<String>,
    pub reference_columns: Option<Vec<String>>,
    pub check_condition: Option<String>,
}

/// Transaction state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionState {
    Idle,
    InProgress,
    InCommit,
    InRollback,
}

/// Transaction
pub struct Transaction {
    pub id: u32,
    pub state: TransactionState,
    pub isolation_level: IsolationLevel,
    pub start_time: i64,
}

/// Isolation level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

/// Database
pub struct Database {
    pub name: String,
    pub tables: BTreeMap<String, Table>,
    pub data: BTreeMap<String, Vec<Row>>,
    pub indexes: BTreeMap<String, Index>,
    pub constraints: BTreeMap<String, Constraint>,
    pub next_table_id: AtomicU32,
    next_transaction_id: AtomicU32,
    active_transactions: BTreeMap<u32, Transaction>,
}

impl Database {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tables: BTreeMap::new(),
            data: BTreeMap::new(),
            indexes: BTreeMap::new(),
            constraints: BTreeMap::new(),
            next_table_id: AtomicU32::new(1),
            next_transaction_id: AtomicU32::new(1),
            active_transactions: BTreeMap::new(),
        }
    }

    /// Create table
    pub fn create_table(&mut self, name: String, columns: Vec<Column>) -> Result<(), &'static str> {
        if self.tables.contains_key(&name) {
            return Err("Table already exists");
        }
        let table = Table {
            name: name.clone(),
            columns,
            next_row_id: AtomicU32::new(1),
        };
        self.tables.insert(name.clone(), table);
        self.data.insert(name, Vec::new());
        Ok(())
    }

    /// Drop table
    pub fn drop_table(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.tables.contains_key(name) {
            return Err("Table not found");
        }
        self.tables.remove(name);
        self.data.remove(name);
        Ok(())
    }

    /// Insert row
    pub fn insert_row(&mut self, table_name: &str, row: Row) -> Result<(), &'static str> {
        let table = self.tables.get(table_name).ok_or("Table not found")?;
        let data = self.data.get_mut(table_name).ok_or("Data not found")?;

        // Validate columns
        for column in &table.columns {
            if !row.contains_key(&column.name) && !column.nullable && column.default_value.is_none()
            {
                return Err("Missing required column");
            }
        }

        data.push(row);
        Ok(())
    }

    /// Select rows
    pub fn select_rows(&self, table_name: &str) -> Result<Vec<Row>, &'static str> {
        let data = self.data.get(table_name).ok_or("Table not found")?;
        Ok(data.clone())
    }

    /// Update row
    pub fn update_row(
        &mut self,
        table_name: &str,
        row_index: usize,
        updates: Row,
    ) -> Result<(), &'static str> {
        let data = self.data.get_mut(table_name).ok_or("Table not found")?;
        if row_index >= data.len() {
            return Err("Row index out of bounds");
        }
        let row = data.get_mut(row_index).ok_or("Row not found")?;
        for (key, value) in updates {
            row.insert(key, value);
        }
        Ok(())
    }

    /// Delete row
    pub fn delete_row(&mut self, table_name: &str, row_index: usize) -> Result<(), &'static str> {
        let data = self.data.get_mut(table_name).ok_or("Table not found")?;
        if row_index >= data.len() {
            return Err("Row index out of bounds");
        }
        data.remove(row_index);
        Ok(())
    }

    /// Create index
    pub fn create_index(
        &mut self,
        name: String,
        table_name: String,
        columns: Vec<String>,
        index_type: IndexType,
        unique: bool,
    ) -> Result<(), &'static str> {
        if !self.tables.contains_key(&table_name) {
            return Err("Table not found");
        }
        let index = Index {
            name: name.clone(),
            table_name,
            columns,
            index_type,
            unique,
        };
        self.indexes.insert(name, index);
        Ok(())
    }

    /// Drop index
    pub fn drop_index(&mut self, name: &str) -> Result<(), &'static str> {
        if !self.indexes.contains_key(name) {
            return Err("Index not found");
        }
        self.indexes.remove(name);
        Ok(())
    }

    /// Add constraint
    pub fn add_constraint(&mut self, constraint: Constraint) -> Result<(), &'static str> {
        if !self.tables.contains_key(&constraint.table_name) {
            return Err("Table not found");
        }
        self.constraints.insert(constraint.name.clone(), constraint);
        Ok(())
    }

    /// Begin transaction
    pub fn begin_transaction(&mut self, isolation_level: IsolationLevel) -> u32 {
        let id = self.next_transaction_id.fetch_add(1, Ordering::Relaxed);
        let transaction = Transaction {
            id,
            state: TransactionState::InProgress,
            isolation_level,
            start_time: 0, // Would use actual timestamp
        };
        self.active_transactions.insert(id, transaction);
        id
    }

    /// Commit transaction
    pub fn commit_transaction(&mut self, transaction_id: u32) -> Result<(), &'static str> {
        let transaction = self
            .active_transactions
            .get_mut(&transaction_id)
            .ok_or("Transaction not found")?;
        transaction.state = TransactionState::InCommit;
        self.active_transactions.remove(&transaction_id);
        Ok(())
    }

    /// Rollback transaction
    pub fn rollback_transaction(&mut self, transaction_id: u32) -> Result<(), &'static str> {
        let transaction = self
            .active_transactions
            .get_mut(&transaction_id)
            .ok_or("Transaction not found")?;
        transaction.state = TransactionState::InRollback;
        self.active_transactions.remove(&transaction_id);
        Ok(())
    }

    /// Execute query (simplified)
    pub fn execute_query(&mut self, query: &str) -> Result<Vec<Row>, &'static str> {
        // Simplified query parser - in real implementation would use full SQL parser
        let query_lower = query.to_lowercase();

        if query_lower.starts_with("select") {
            // Extract table name
            let parts: Vec<&str> = query_lower.split("from").collect();
            if parts.len() < 2 {
                return Err("Invalid SELECT query");
            }
            let table_name = parts[1]
                .trim()
                .split_whitespace()
                .next()
                .ok_or("No table name")?;
            self.select_rows(table_name)
        } else if query_lower.starts_with("insert") {
            Ok(Vec::new())
        } else if query_lower.starts_with("update") {
            Ok(Vec::new())
        } else if query_lower.starts_with("delete") {
            Ok(Vec::new())
        } else {
            Err("Unsupported query type")
        }
    }

    /// Get table info
    pub fn get_table_info(&self, table_name: &str) -> Result<&Table, &'static str> {
        self.tables.get(table_name).ok_or("Table not found")
    }

    /// List tables
    pub fn list_tables(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }

    /// Get database size
    pub fn get_size(&self) -> usize {
        self.data.values().map(|rows| rows.len()).sum()
    }
}

/// Global database
static mut DATABASE: Option<Database> = None;

/// Initialize database
pub fn init_database(name: String) {
    unsafe {
        DATABASE = Some(Database::new(name));
    }
}

/// Get database
pub fn get_database() -> &'static mut Database {
    unsafe { DATABASE.as_mut().expect("Database not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::new("test_db".to_string());
        assert_eq!(db.name, "test_db");
    }

    #[test]
    fn test_create_table() {
        let mut db = Database::new("test_db".to_string());
        let columns = vec![
            Column {
                name: "id".to_string(),
                column_type: ColumnType::Integer,
                nullable: false,
                primary_key: true,
                unique: true,
                default_value: None,
            },
            Column {
                name: "name".to_string(),
                column_type: ColumnType::Text,
                nullable: false,
                primary_key: false,
                unique: false,
                default_value: None,
            },
        ];
        db.create_table("users".to_string(), columns).unwrap();
        assert!(db.tables.contains_key("users"));
    }

    #[test]
    fn test_insert_row() {
        let mut db = Database::new("test_db".to_string());
        let columns = vec![
            Column {
                name: "id".to_string(),
                column_type: ColumnType::Integer,
                nullable: false,
                primary_key: true,
                unique: true,
                default_value: None,
            },
            Column {
                name: "name".to_string(),
                column_type: ColumnType::Text,
                nullable: false,
                primary_key: false,
                unique: false,
                default_value: None,
            },
        ];
        db.create_table("users".to_string(), columns).unwrap();

        let mut row = Row::new();
        row.insert("id".to_string(), Value::Integer(1));
        row.insert("name".to_string(), Value::Text("Alice".to_string()));
        db.insert_row("users", row).unwrap();

        assert_eq!(db.get_size(), 1);
    }

    #[test]
    fn test_select_rows() {
        let mut db = Database::new("test_db".to_string());
        let columns = vec![Column {
            name: "id".to_string(),
            column_type: ColumnType::Integer,
            nullable: false,
            primary_key: true,
            unique: true,
            default_value: None,
        }];
        db.create_table("users".to_string(), columns).unwrap();

        let mut row = Row::new();
        row.insert("id".to_string(), Value::Integer(1));
        db.insert_row("users", row).unwrap();

        let rows = db.select_rows("users").unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_transaction() {
        let mut db = Database::new("test_db".to_string());
        let tx_id = db.begin_transaction(IsolationLevel::ReadCommitted);
        assert!(db.active_transactions.contains_key(&tx_id));
        db.commit_transaction(tx_id).unwrap();
        assert!(!db.active_transactions.contains_key(&tx_id));
    }
}
