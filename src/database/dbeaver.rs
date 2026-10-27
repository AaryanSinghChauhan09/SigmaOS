// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - DBeaver universal SQL workbench
// Principles: UNIFORM ACCESS, INTERFACE SEGREGATION, OPEN STANDARD
use alloc::{collections::BTreeMap, format, string::String, vec::Vec};
#[derive(Debug, Clone, PartialEq)]
pub enum DbDriver {
    PostgreSql,
    DuckDb,
    Sqlite,
    MySql,
    MariaDb,
    MsSqlServer,
    Oracle,
    Clickhouse,
    Redshift,
}
impl DbDriver {
    pub fn default_port(&self) -> u16 {
        match self {
            Self::PostgreSql => 5432,
            Self::MySql | Self::MariaDb => 3306,
            Self::MsSqlServer => 1433,
            Self::Oracle => 1521,
            Self::Clickhouse => 9000,
            Self::Redshift => 5439,
            _ => 0,
        }
    }
}
#[derive(Debug, Clone)]
pub struct DbConnection {
    pub id: u32,
    pub name: String,
    pub driver: DbDriver,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub connected: bool,
}
impl DbConnection {
    pub fn new(
        id: u32,
        name: &str,
        driver: DbDriver,
        host: &str,
        database: &str,
        user: &str,
    ) -> Self {
        let port = driver.default_port();
        Self {
            id,
            name: name.into(),
            driver,
            host: host.into(),
            port,
            database: database.into(),
            user: user.into(),
            connected: false,
        }
    }
}
pub struct DbConnectionManager {
    pub connections: Vec<DbConnection>,
    pub next_id: u32,
}
impl DbConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            next_id: 1,
        }
    }
    pub fn add_connection(
        &mut self,
        name: &str,
        driver: DbDriver,
        host: &str,
        db: &str,
        user: &str,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.connections
            .push(DbConnection::new(id, name, driver, host, db, user));
        id
    }
    pub fn connect(&mut self, id: u32) -> Result<(), &'static str> {
        self.connections
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or("not found")
            .map(|c| c.connected = true)
    }
    pub fn disconnect(&mut self, id: u32) {
        if let Some(c) = self.connections.iter_mut().find(|c| c.id == id) {
            c.connected = false;
        }
    }
    pub fn test_connection(&self, id: u32) -> bool {
        self.connections
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.connected)
            .unwrap_or(false)
    }
    pub fn list(&self) -> Vec<&DbConnection> {
        self.connections.iter().collect()
    }
}
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
pub struct SqlEditor {
    pub query: String,
    pub last_result: Option<QueryResult>,
}
impl SqlEditor {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            last_result: None,
        }
    }
    pub fn set_query(&mut self, q: &str) {
        self.query = q.into();
    }
    pub fn execute(&mut self, _conn_id: u32) -> Result<&QueryResult, &'static str> {
        self.last_result = Some(QueryResult {
            columns: alloc::vec!["result".into()],
            rows: alloc::vec![alloc::vec!["ok".into()]],
        });
        Ok(self.last_result.as_ref().unwrap())
    }
    pub fn get_results(&self) -> Option<&QueryResult> {
        self.last_result.as_ref()
    }
}
#[derive(Debug, Clone)]
pub struct ErTable {
    pub name: String,
    pub columns: Vec<(String, String)>,
}
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub from_table: String,
    pub from_col: String,
    pub to_table: String,
    pub to_col: String,
}
pub struct ErDiagram {
    pub tables: Vec<ErTable>,
    pub foreign_keys: Vec<ForeignKey>,
}
impl ErDiagram {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            foreign_keys: Vec::new(),
        }
    }
    pub fn add_table(&mut self, name: &str, cols: Vec<(&str, &str)>) {
        self.tables.push(ErTable {
            name: name.into(),
            columns: cols
                .iter()
                .map(|(n, t)| (n.to_string(), t.to_string()))
                .collect(),
        });
    }
    pub fn add_foreign_key(&mut self, ft: &str, fc: &str, tt: &str, tc: &str) {
        self.foreign_keys.push(ForeignKey {
            from_table: ft.into(),
            from_col: fc.into(),
            to_table: tt.into(),
            to_col: tc.into(),
        });
    }
    pub fn export_plantuml(&self) -> String {
        let mut s = String::from(
            "@startuml
",
        );
        for t in &self.tables {
            s.push_str(&format!(
                "entity {} {{
}}
",
                t.name
            ));
        }
        for fk in &self.foreign_keys {
            s.push_str(&format!(
                "{} ||--o{{ {} : fk
",
                fk.from_table, fk.to_table
            ));
        }
        s.push_str(
            "@enduml
",
        );
        s
    }
}
#[derive(Debug, Clone)]
pub enum ExportFormat {
    Csv,
    Json,
    SqlInsert,
    Excel,
}
pub struct DataExporter;
impl DataExporter {
    pub fn export(result: &QueryResult, fmt: ExportFormat) -> Vec<u8> {
        match fmt {
            ExportFormat::Csv => {
                let header = result.columns.join(",");
                let rows: Vec<String> = result.rows.iter().map(|r| r.join(",")).collect();
                let mut full = header;
                for r in rows {
                    full.push('\n');
                    full.push_str(&r);
                }
                full.into_bytes()
            }
            ExportFormat::Json => format!(
                "{{\"columns\":{:?},\"rows\":{:?}}}",
                result.columns, result.rows
            )
            .into_bytes(),
            ExportFormat::SqlInsert => {
                let cols = result.columns.join(", ");
                let mut full = String::new();
                for r in &result.rows {
                    full.push_str(&format!(
                        "INSERT INTO t ({}) VALUES ({});\n",
                        cols,
                        r.join(", ")
                    ));
                }
                full.into_bytes()
            }
            ExportFormat::Excel => b"PK".to_vec(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_connection_manager() {
        let mut m = DbConnectionManager::new();
        let id = m.add_connection("local", DbDriver::PostgreSql, "localhost", "mydb", "admin");
        m.connect(id).unwrap();
        assert!(m.test_connection(id));
        m.disconnect(id);
        assert!(!m.test_connection(id));
    }
    #[test]
    fn test_sql_editor() {
        let mut e = SqlEditor::new();
        e.set_query("SELECT 1");
        e.execute(1).unwrap();
        assert!(e.get_results().is_some());
    }
    #[test]
    fn test_er_diagram() {
        let mut d = ErDiagram::new();
        d.add_table("users", alloc::vec![("id", "int"), ("name", "varchar")]);
        d.add_table("orders", alloc::vec![("id", "int"), ("user_id", "int")]);
        d.add_foreign_key("orders", "user_id", "users", "id");
        let pu = d.export_plantuml();
        assert!(pu.contains("@startuml"));
    }
    #[test]
    fn test_csv_export() {
        let r = QueryResult {
            columns: alloc::vec!["a".into(), "b".into()],
            rows: alloc::vec![alloc::vec!["1".into(), "2".into()]],
        };
        let csv = DataExporter::export(&r, ExportFormat::Csv);
        assert!(String::from_utf8(csv).unwrap().contains("a,b"));
    }
}
