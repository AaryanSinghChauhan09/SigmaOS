//! SigmaGrid - Grid Layout System
//!
//! Provides comprehensive grid layout management inspired by Grid Craft.
//! Supports grid systems, responsive layouts, and alignment.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Grid unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridUnit {
    Pixel,
    Percent,
    Fraction,
    Auto,
;

/// Grid alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridAlignment {
    Start,
    Center,
    End,
    Stretch,
;

/// Grid item
#[derive(Debug, Clone)]
pub struct GridItem {
    pub id: String,
    pub row_start: u32,
    pub row_end: u32,
    pub column_start: u32,
    pub column_end: u32,
    pub row_span: u32,
    pub column_span: u32,
;

impl GridItem {
    pub fn new(id: String, row: u32, column: u32) -> Self {
        Self {
            id,
            row_start: row,
            row_end: row + 1,
            column_start: column,
            column_end: column + 1,
            row_span: 1,
            column_span: 1,
        }
    }

    pub fn set_row_span(&mut self, span: u32) {
        self.row_span = span;
        self.row_end = self.row_start + span;
    }

    pub fn set_column_span(&mut self, span: u32) {
        self.column_span = span;
        self.column_end = self.column_start + span;
    }

    pub fn set_position(&mut self, row_start: u32, column_start: u32) {
        self.row_start = row_start;
        self.column_start = column_start;
        self.row_end = row_start + self.row_span;
        self.column_end = column_start + self.column_span;
    }
;

/// Grid row
#[derive(Debug, Clone)]
pub struct GridRow {
    pub id: String,
    pub height: f32,
    pub min_height: f32,
    pub max_height: f32,
;

impl GridRow {
    pub fn new(id: String, height: f32) -> Self {
        Self {
            id,
            height,
            min_height: 0.0,
            max_height: f32::MAX,
        }
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn set_min_height(&mut self, min_height: f32) {
        self.min_height = min_height;
    }

    pub fn set_max_height(&mut self, max_height: f32) {
        self.max_height = max_height;
    }
;

/// Grid column
#[derive(Debug, Clone)]
pub struct GridColumn {
    pub id: String,
    pub width: f32,
    pub min_width: f32,
    pub max_width: f32,
    pub unit: GridUnit,
;

impl GridColumn {
    pub fn new(id: String, width: f32, unit: GridUnit) -> Self {
        Self {
            id,
            width,
            min_width: 0.0,
            max_width: f32::MAX,
            unit,
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_min_width(&mut self, min_width: f32) {
        self.min_width = min_width;
    }

    pub fn set_max_width(&mut self, max_width: f32) {
        self.max_width = max_width;
    }

    pub fn set_unit(&mut self, unit: GridUnit) {
        self.unit = unit;
    }
;

/// Grid area
#[derive(Debug, Clone)]
pub struct GridArea {
    pub id: String,
    pub name: String,
    pub rows: u32,
    pub columns: u32,
    pub gap_x: f32,
    pub gap_y: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub items: BTreeMap<String, GridItem>,
    pub grid_rows: BTreeMap<String, GridRow>,
    pub grid_columns: BTreeMap<String, GridColumn>,
;

impl GridArea {
    pub fn new(id: String, name: String, rows: u32, columns: u32) -> Self {
        Self {
            id,
            name,
            rows,
            columns,
            gap_x: 0.0,
            gap_y: 0.0,
            padding_x: 0.0,
            padding_y: 0.0,
            items: BTreeMap::new(),
            grid_rows: BTreeMap::new(),
            grid_columns: BTreeMap::new(),
        }
    }

    pub fn add_item(&mut self, item: GridItem) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn remove_item(&mut self, id: &str) -> bool {
        self.items.remove(id).is_some()
    }

    pub fn get_item(&self, id: &str) -> Option<&GridItem> {
        self.items.get(id)
    }

    pub fn get_item_mut(&mut self, id: &str) -> Option<&mut GridItem> {
        self.items.get_mut(id)
    }

    pub fn add_row(&mut self, row: GridRow) {
        self.grid_rows.insert(row.id.clone(), row);
    }

    pub fn remove_row(&mut self, id: &str) -> bool {
        self.grid_rows.remove(id).is_some()
    }

    pub fn add_column(&mut self, column: GridColumn) {
        self.grid_columns.insert(column.id.clone(), column);
    }

    pub fn remove_column(&mut self, id: &str) -> bool {
        self.grid_columns.remove(id).is_some()
    }

    pub fn set_gap(&mut self, gap_x: f32, gap_y: f32) {
        self.gap_x = gap_x;
        self.gap_y = gap_y;
    }

    pub fn set_padding(&mut self, padding_x: f32, padding_y: f32) {
        self.padding_x = padding_x;
        self.padding_y = padding_y;
    }

    pub fn resize(&mut self, rows: u32, columns: u32) {
        self.rows = rows;
        self.columns = columns;
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn row_count(&self) -> usize {
        self.grid_rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.grid_columns.len()
    }
;

/// SigmaGrid - Grid layout system manager
#[derive(Debug)]
pub struct SigmaGrid {
    grid_areas: BTreeMap<String, GridArea>,
    current_area: Option<String>,
;

impl SigmaGrid {
    pub fn new() -> Self {
        Self {
            grid_areas: BTreeMap::new(),
            current_area: None,
        }
    }

    pub fn add_grid_area(&mut self, area: GridArea) {
        self.grid_areas.insert(area.id.clone(), area);
    }

    pub fn remove_grid_area(&mut self, id: &str) -> bool {
        if self.current_area.as_ref() == Some(&id.to_string()) {
            self.current_area = None;
        }
        self.grid_areas.remove(id).is_some()
    }

    pub fn get_grid_area(&self, id: &str) -> Option<&GridArea> {
        self.grid_areas.get(id)
    }

    pub fn get_grid_area_mut(&mut self, id: &str) -> Option<&mut GridArea> {
        self.grid_areas.get_mut(id)
    }

    pub fn set_current_area(&mut self, id: &str) -> bool {
        if self.grid_areas.contains_key(id) {
            self.current_area = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_area(&self) -> Option<&GridArea> {
        self.current_area
            .as_ref()
            .and_then(|id| self.grid_areas.get(id))
    }

    pub fn get_current_area_mut(&mut self) -> Option<&mut GridArea> {
        self.current_area
            .as_ref()
            .and_then(|id| self.grid_areas.get_mut(id.as_str()))
    }

    pub fn list_grid_areas(&self) -> Vec<&GridArea> {
        self.grid_areas.values().collect()
    }

    pub fn grid_area_count(&self) -> usize {
        self.grid_areas.len()
    }
;

impl Default for SigmaGrid {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaGrid instance
static mut SIGMAGRID_INSTANCE: Option<SigmaGrid> = None;

/// Initialize the global SigmaGrid instance
pub fn init_sigmagrid() {
    unsafe {
        SIGMAGRID_INSTANCE = Some(SigmaGrid::new());
    }
;

/// Get the global SigmaGrid instance
pub fn get_sigmagrid() -> Option<&'static SigmaGrid> {
    unsafe { SIGMAGRID_INSTANCE.as_ref() }
;

/// Get the global SigmaGrid instance (mutable)
pub fn get_sigmagrid_mut() -> Option<&'static mut SigmaGrid> {
    unsafe { SIGMAGRID_INSTANCE.as_mut() }
;

/// Convenience function to add a grid area
pub fn add_grid_area(area: GridArea) -> bool {
    if let Some(grid) = get_sigmagrid_mut() {
        grid.add_grid_area(area);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_item_creation() {
        let item = GridItem::new("item1".to_string(), 0, 0);
        assert_eq!(item.id, "item1");
        assert_eq!(item.row_start, 0);
        assert_eq!(item.column_start, 0);
        assert_eq!(item.row_span, 1);
        assert_eq!(item.column_span, 1);
    }

    #[test]
    fn test_grid_item_modification() {
        let mut item = GridItem::new("item1".to_string(), 0, 0);
        item.set_row_span(2);
        item.set_column_span(3);
        assert_eq!(item.row_span, 2);
        assert_eq!(item.column_span, 3);
        assert_eq!(item.row_end, 2);
        assert_eq!(item.column_end, 3);
    }

    #[test]
    fn test_grid_row_creation() {
        let row = GridRow::new("row1".to_string(), 100.0);
        assert_eq!(row.id, "row1");
        assert_eq!(row.height, 100.0);
    }

    #[test]
    fn test_grid_column_creation() {
        let column = GridColumn::new("col1".to_string(), 50.0, GridUnit::Pixel);
        assert_eq!(column.id, "col1");
        assert_eq!(column.width, 50.0);
        assert_eq!(column.unit, GridUnit::Pixel);
    }

    #[test]
    fn test_grid_area_creation() {
        let area = GridArea::new("area1".to_string(), "Main Grid".to_string(), 12, 12);
        assert_eq!(area.id, "area1");
        assert_eq!(area.rows, 12);
        assert_eq!(area.columns, 12);
    }

    #[test]
    fn test_grid_area_management() {
        let mut area = GridArea::new("area1".to_string(), "Test".to_string(), 12, 12);
        let item = GridItem::new("item1".to_string(), 0, 0);
        area.add_item(item);
        assert_eq!(area.item_count(), 1);
        assert!(area.remove_item("item1"));
        assert_eq!(area.item_count(), 0);
    }

    #[test]
    fn test_sigmagrid_creation() {
        let grid = SigmaGrid::new();
        assert_eq!(grid.grid_area_count(), 0);
    }

    #[test]
    fn test_sigmagrid_management() {
        let mut grid = SigmaGrid::new();
        let area = GridArea::new("area1".to_string(), "Test".to_string(), 12, 12);
        grid.add_grid_area(area);
        assert_eq!(grid.grid_area_count(), 1);
    }
;
