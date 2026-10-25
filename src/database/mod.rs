// SPDX-License-Identifier: MIT
// SigmaOS - Database Module
// Database systems inspired by PostgreSQL, MySQL, SQLite

pub mod postgres;

pub use postgres::{
    get_database, init_database, Column, ColumnType, Constraint, ConstraintType, Database, Index,
    IndexType, IsolationLevel, Row, Table, Transaction, TransactionState, Value,
};

// ─── Extended Modules ──────────────────────────────────────
pub mod duckdb;
pub use duckdb::*;
pub mod arrow_datafusion;
pub use arrow_datafusion::*;
pub mod dbeaver;
pub use dbeaver::*;
pub mod superset;
pub use superset::*;
pub mod metabase;
pub use metabase::*;
