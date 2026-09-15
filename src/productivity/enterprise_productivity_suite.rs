// SigmaOS Enterprise Productivity & Office Suite
// Clean-room Rust implementations inspired by Google Workspace (Docs, Sheets, Slides, Looker Studio), Zoho, Microsoft 365, Salesforce, Odoo, and Bitrix24.
// Zero-dependency, #![no_std] compliant native Rust implementation.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. GOOGLE DOCS REALTIME COLLABORATIVE EDITOR ENGINE (CRDT / OT)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentCollaborator {
    pub user_id: String,
    pub user_name: String,
    pub cursor_position: usize,
    pub active_color: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentComment {
    pub comment_id: u64,
    pub author: String,
    pub content: String,
    pub resolved: bool,
}

pub struct GoogleDocsRealtimeCollaborativeEditorEngine {
    pub document_title: String,
    pub text_buffer: String,
    pub collaborators: BTreeMap<String, DocumentCollaborator>,
    pub comments: Vec<DocumentComment>,
    pub revision_history: Vec<String>,
}

impl GoogleDocsRealtimeCollaborativeEditorEngine {
    pub fn new(title: &str) -> Self {
        Self {
            document_title: title.to_string(),
            text_buffer: String::new(),
            collaborators: BTreeMap::new(),
            comments: Vec::new(),
            revision_history: Vec::new(),
        }
    }

    pub fn join_session(&mut self, user_id: &str, name: &str, color: &str) {
        self.collaborators.insert(
            user_id.to_string(),
            DocumentCollaborator {
                user_id: user_id.to_string(),
                user_name: name.to_string(),
                cursor_position: 0,
                active_color: color.to_string(),
            },
        );
    }

    pub fn insert_text(&mut self, user_id: &str, position: usize, text: &str) -> bool {
        if position > self.text_buffer.len() {
            return false;
        }

        self.text_buffer.insert_str(position, text);
        self.revision_history.push(self.text_buffer.clone());

        if let Some(collab) = self.collaborators.get_mut(user_id) {
            collab.cursor_position = position + text.len();
        }

        true
    }

    pub fn add_comment(&mut self, author: &str, content: &str) -> u64 {
        let comment_id = (self.comments.len() + 1) as u64;
        self.comments.push(DocumentComment {
            comment_id,
            author: author.to_string(),
            content: content.to_string(),
            resolved: false,
        });
        comment_id
    }

    pub fn resolve_comment(&mut self, comment_id: u64) -> bool {
        if let Some(comment) = self.comments.iter_mut().find(|c| c.comment_id == comment_id) {
            comment.resolved = true;
            true
        } else {
            false
        }
    }
}

// =========================================================================
// 2. GOOGLE SHEETS / EXCEL FORMULA CALCULATION ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum CellValue {
    Text(String),
    Number(f64),
    Formula(String),
    Error(String),
}

pub struct GoogleSheetsFormulaCalcEngine {
    pub cells: BTreeMap<String, CellValue>,
}

impl GoogleSheetsFormulaCalcEngine {
    pub fn new() -> Self {
        Self {
            cells: BTreeMap::new(),
        }
    }

    pub fn set_cell_value(&mut self, cell_ref: &str, value: CellValue) {
        self.cells.insert(cell_ref.to_string(), value);
    }

    pub fn evaluate_sum(&self, range: &[&str]) -> f64 {
        let mut sum = 0.0;
        for cell_ref in range {
            if let Some(CellValue::Number(num)) = self.cells.get(*cell_ref) {
                sum += num;
            }
        }
        sum
    }

    pub fn evaluate_average(&self, range: &[&str]) -> f64 {
        let sum = self.evaluate_sum(range);
        let count = range
            .iter()
            .filter(|cell_ref| matches!(self.cells.get(**cell_ref), Some(CellValue::Number(_))))
            .count();

        if count == 0 {
            0.0
        } else {
            sum / (count as f64)
        }
    }
}

impl Default for GoogleSheetsFormulaCalcEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. GOOGLE SLIDES / POWERPOINT PRESENTATION ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlideShape {
    pub shape_type: String,
    pub x: u32,
    pub y: u32,
    pub text_content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationSlide {
    pub slide_id: u32,
    pub title: String,
    pub shapes: Vec<SlideShape>,
    pub speaker_notes: String,
}

pub struct GoogleSlidesPresentationEngine {
    pub deck_title: String,
    pub slides: Vec<PresentationSlide>,
    pub active_slide_index: usize,
}

impl GoogleSlidesPresentationEngine {
    pub fn new(deck_title: &str) -> Self {
        Self {
            deck_title: deck_title.to_string(),
            slides: Vec::new(),
            active_slide_index: 0,
        }
    }

    pub fn add_slide(&mut self, title: &str, notes: &str) -> u32 {
        let slide_id = (self.slides.len() + 1) as u32;
        self.slides.push(PresentationSlide {
            slide_id,
            title: title.to_string(),
            shapes: Vec::new(),
            speaker_notes: notes.to_string(),
        });
        slide_id
    }

    pub fn add_shape_to_slide(&mut self, slide_id: u32, shape_type: &str, x: u32, y: u32, text: &str) -> bool {
        if let Some(slide) = self.slides.iter_mut().find(|s| s.slide_id == slide_id) {
            slide.shapes.push(SlideShape {
                shape_type: shape_type.to_string(),
                x,
                y,
                text_content: text.to_string(),
            });
            true
        } else {
            false
        }
    }
}

// =========================================================================
// 4. GOOGLE LOOKER STUDIO / POWER BI DATA ANALYTICS & DASHBOARD ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct DataPoint {
    pub dimension: String,
    pub metric_value: f64,
}

pub struct GoogleLookerStudioDataBiEngine {
    pub dashboard_title: String,
    pub datasets: BTreeMap<String, Vec<DataPoint>>,
}

impl GoogleLookerStudioDataBiEngine {
    pub fn new(title: &str) -> Self {
        Self {
            dashboard_title: title.to_string(),
            datasets: BTreeMap::new(),
        }
    }

    pub fn add_data_point(&mut self, dataset_name: &str, dimension: &str, metric: f64) {
        let dataset = self.datasets.entry(dataset_name.to_string()).or_insert_with(Vec::new);
        dataset.push(DataPoint {
            dimension: dimension.to_string(),
            metric_value: metric,
        });
    }

    pub fn calculate_dataset_total(&self, dataset_name: &str) -> f64 {
        if let Some(points) = self.datasets.get(dataset_name) {
            points.iter().map(|p| p.metric_value).sum()
        } else {
            0.0
        }
    }
}

// =========================================================================
// 5. ZOHO / SALESFORCE / ODOO / BITRIX24 CRM & ERP ENGINE
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrmLeadStage {
    New,
    Contacted,
    Qualified,
    Won,
    Lost,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrmDealLead {
    pub lead_id: u64,
    pub company_name: String,
    pub contact_email: String,
    pub deal_value: f64,
    pub stage: CrmLeadStage,
}

pub struct ZohoMicrosoftSalesforceCrmErpEngine {
    pub organization_name: String,
    pub leads: BTreeMap<u64, CrmDealLead>,
    pub next_lead_id: u64,
}

impl ZohoMicrosoftSalesforceCrmErpEngine {
    pub fn new(org_name: &str) -> Self {
        Self {
            organization_name: org_name.to_string(),
            leads: BTreeMap::new(),
            next_lead_id: 1,
        }
    }

    pub fn create_lead(&mut self, company: &str, email: &str, value: f64) -> u64 {
        let id = self.next_lead_id;
        self.next_lead_id += 1;

        self.leads.insert(
            id,
            CrmDealLead {
                lead_id: id,
                company_name: company.to_string(),
                contact_email: email.to_string(),
                deal_value: value,
                stage: CrmLeadStage::New,
            },
        );
        id
    }

    pub fn update_stage(&mut self, lead_id: u64, stage: CrmLeadStage) -> bool {
        if let Some(lead) = self.leads.get_mut(&lead_id) {
            lead.stage = stage;
            true
        } else {
            false
        }
    }

    pub fn total_pipeline_value(&self) -> f64 {
        self.leads.values().map(|l| l.deal_value).sum()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_docs_realtime_editor() {
        let mut docs = GoogleDocsRealtimeCollaborativeEditorEngine::new("Project Roadmap");
        docs.join_session("user1", "Alice", "#FF0000");

        assert!(docs.insert_text("user1", 0, "SigmaOS v1.0 Release"));
        assert_eq!(docs.text_buffer, "SigmaOS v1.0 Release");

        let cid = docs.add_comment("Alice", "Looks ready for release!");
        assert_eq!(cid, 1);
        assert!(docs.resolve_comment(cid));
    }

    #[test]
    fn test_google_sheets_formula_calc() {
        let mut sheets = GoogleSheetsFormulaCalcEngine::new();
        sheets.set_cell_value("A1", CellValue::Number(100.0));
        sheets.set_cell_value("A2", CellValue::Number(200.0));
        sheets.set_cell_value("A3", CellValue::Number(300.0));

        let range = vec!["A1", "A2", "A3"];
        assert_eq!(sheets.evaluate_sum(&range), 600.0);
        assert_eq!(sheets.evaluate_average(&range), 200.0);
    }

    #[test]
    fn test_google_slides_presentation() {
        let mut slides = GoogleSlidesPresentationEngine::new("Keynote 2026");
        let slide_id = slides.add_slide("Introduction", "Welcome speech notes");
        assert_eq!(slide_id, 1);

        assert!(slides.add_shape_to_slide(slide_id, "TextBox", 100, 200, "SigmaOS OS Engine"));
        assert_eq!(slides.slides[0].shapes.len(), 1);
    }

    #[test]
    fn test_google_looker_studio_bi() {
        let mut looker = GoogleLookerStudioDataBiEngine::new("Q1 Sales Report");
        looker.add_data_point("Revenue", "North America", 150000.0);
        looker.add_data_point("Revenue", "Europe", 120000.0);

        assert_eq!(looker.calculate_dataset_total("Revenue"), 270000.0);
    }

    #[test]
    fn test_crm_erp_engine() {
        let mut crm = ZohoMicrosoftSalesforceCrmErpEngine::new("Acme Corp");
        let lead1 = crm.create_lead("Enterprise Systems", "contact@enterprise.com", 50000.0);
        let _lead2 = crm.create_lead("Global Tech", "info@globaltech.com", 75000.0);

        assert_eq!(crm.total_pipeline_value(), 125000.0);
        assert!(crm.update_stage(lead1, CrmLeadStage::Won));
        assert_eq!(crm.leads.get(&lead1).unwrap().stage, CrmLeadStage::Won);
    }
}
