// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Apache Superset BI visualization
// Principles: MODULARITY, OPEN STANDARD, SEPARATION OF CONCERNS
use alloc::{string::String, vec::Vec, collections::BTreeMap};
#[derive(Debug,Clone,PartialEq)] pub enum ChartType { Bar, Line, Pie, Scatter, Heatmap, Sankey, FunnelChart, TreeMap, Histogram, Table, BigNumber, Gauge, WordCloud, Sunburst, BoxPlot }
impl ChartType { pub fn as_str(&self) -> &'static str { match self { Self::Bar=>"bar", Self::Line=>"line", Self::Pie=>"pie", Self::Scatter=>"scatter", Self::Heatmap=>"heatmap", Self::Sankey=>"sankey", Self::FunnelChart=>"funnel_chart", Self::TreeMap=>"treemap", Self::Histogram=>"histogram", Self::Table=>"table", Self::BigNumber=>"big_number", Self::Gauge=>"gauge", Self::WordCloud=>"word_cloud", Self::Sunburst=>"sunburst", Self::BoxPlot=>"box_plot" } } }
#[derive(Debug,Clone)] pub struct DatasetConfig { pub id: u32, pub name: String, pub sql_query: Option<String>, pub table_name: Option<String>, pub datasource_type: String }
#[derive(Debug,Clone)] pub struct ChartSlice { pub id: u32, pub name: String, pub chart_type: ChartType, pub dataset_id: u32, pub config: BTreeMap<String,String> }
impl ChartSlice {
    pub fn new(id: u32, name: &str, ct: ChartType, dataset_id: u32) -> Self { Self { id, name: name.into(), chart_type: ct, dataset_id, config: BTreeMap::new() } }
    pub fn set_config(&mut self, k: &str, v: &str) { self.config.insert(k.into(), v.into()); }
}
#[derive(Debug,Clone)] pub struct DashboardPosition { pub col: u8, pub row: u8, pub width: u8, pub height: u8 }
#[derive(Debug,Clone)] pub struct DashboardSliceRef { pub slice_id: u32, pub position: DashboardPosition }
#[derive(Debug,Clone)] pub struct Dashboard { pub id: u32, pub title: String, pub slices: Vec<DashboardSliceRef>, pub filters: Vec<String> }
impl Dashboard {
    pub fn new(id: u32, title: &str) -> Self { Self { id, title: title.into(), slices: Vec::new(), filters: Vec::new() } }
    pub fn add_slice(&mut self, slice_id: u32, col: u8, row: u8, w: u8, h: u8) { self.slices.push(DashboardSliceRef { slice_id, position: DashboardPosition { col, row, width: w, height: h } }); }
    pub fn add_filter(&mut self, f: &str) { self.filters.push(f.into()); }
}
pub struct SupersetCatalog { pub datasets: Vec<DatasetConfig>, pub charts: Vec<ChartSlice>, pub dashboards: Vec<Dashboard>, pub next_id: u32 }
impl SupersetCatalog {
    pub fn new() -> Self { Self { datasets: Vec::new(), charts: Vec::new(), dashboards: Vec::new(), next_id: 1 } }
    pub fn register_datasource(&mut self, name: &str, sql: Option<&str>, table: Option<&str>) -> u32 { let id = self.next_id; self.next_id += 1; self.datasets.push(DatasetConfig { id, name: name.into(), sql_query: sql.map(|s| s.into()), table_name: table.map(|s| s.into()), datasource_type: String::from("table") }); id }
    pub fn add_chart(&mut self, name: &str, ct: ChartType, dataset_id: u32) -> u32 { let id = self.next_id; self.next_id += 1; self.charts.push(ChartSlice::new(id, name, ct, dataset_id)); id }
    pub fn create_dashboard(&mut self, title: &str) -> u32 { let id = self.next_id; self.next_id += 1; self.dashboards.push(Dashboard::new(id, title)); id }
    pub fn add_chart_to_dashboard(&mut self, dash_id: u32, chart_id: u32, col: u8, row: u8, w: u8, h: u8) -> Result<(),&'static str> { self.dashboards.iter_mut().find(|d| d.id == dash_id).ok_or("dashboard not found").map(|d| d.add_slice(chart_id, col, row, w, h)) }
    pub fn export_dashboard_json(&self, dash_id: u32) -> Option<alloc::string::String> { self.dashboards.iter().find(|d| d.id == dash_id).map(|d| alloc::format!("{{\"title\":\"{}\",\"slices\":{}}}", d.title, d.slices.len())) }
}
pub struct SqlLabQuery { pub sql: String, pub datasource_id: u32, pub running: bool }
impl SqlLabQuery {
    pub fn new(sql: &str, ds: u32) -> Self { Self { sql: sql.into(), datasource_id: ds, running: false } }
    pub fn execute_async(&mut self) { self.running = true; }
    pub fn cancel(&mut self) { self.running = false; }
    pub fn get_results(&self) -> Vec<BTreeMap<String,String>> { Vec::new() }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_catalog() { let mut cat = SupersetCatalog::new(); let ds = cat.register_datasource("sales", None, Some("sales_table")); let chart = cat.add_chart("Monthly Sales", ChartType::Bar, ds); let dash = cat.create_dashboard("Sales Overview"); cat.add_chart_to_dashboard(dash, chart, 0, 0, 6, 4).unwrap(); let json = cat.export_dashboard_json(dash).unwrap(); assert!(json.contains("Sales Overview")); }
    #[test] fn test_chart_types() { assert_eq!(ChartType::Pie.as_str(), "pie"); assert_eq!(ChartType::Sankey.as_str(), "sankey"); }
}
