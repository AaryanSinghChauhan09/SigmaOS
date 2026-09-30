// SPDX-License-Identifier: MIT
// SigmaOS Enterprise Productivity Suite
// Inspired by Google Workspace (Looker Studio, Slides, Docs), Zoho CRM, Salesforce, Odoo, and Bitrix24.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. Google Looker Studio / PowerBI Parity Analytics Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyticsChartWidget {
    pub widget_id: String,
    pub title: String,
    pub chart_type: String, // "bar", "line", "pie", "scorecard"
    pub data_source: String,
    pub metrics: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SigmaLookerAnalyticsEngine {
    pub dashboard_title: String,
    pub widgets: BTreeMap<String, AnalyticsChartWidget>,
    pub data_records: Vec<BTreeMap<String, String>>,
}

impl SigmaLookerAnalyticsEngine {
    pub fn new(title: &str) -> Self {
        Self {
            dashboard_title: title.to_string(),
            widgets: BTreeMap::new(),
            data_records: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: AnalyticsChartWidget) {
        self.widgets.insert(widget.widget_id.clone(), widget);
    }

    pub fn ingest_record(&mut self, record: BTreeMap<String, String>) {
        self.data_records.push(record);
    }

    pub fn compute_metric_sum(&self, field_name: &str) -> f64 {
        let mut total = 0.0;
        for rec in &self.data_records {
            if let Some(val_str) = rec.get(field_name) {
                if let Ok(val) = val_str.parse::<f64>() {
                    total += val;
                }
            }
        }
        total
    }

    pub fn export_dashboard_summary(&self) -> String {
        format!(
            "Dashboard: {} | Widgets: {} | Records Ingested: {}",
            self.dashboard_title,
            self.widgets.len(),
            self.data_records.len()
        )
    }
}

// =========================================================================
// 2. Google Slides / PowerPoint Parity Presentation Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationSlide {
    pub slide_number: u32,
    pub title: String,
    pub body_markdown: String,
    pub speaker_notes: String,
}

#[derive(Debug, Clone)]
pub struct SigmaSlidesPresenterEngine {
    pub presentation_title: String,
    pub slides: Vec<PresentationSlide>,
    pub current_slide_idx: usize,
    pub is_presenter_mode: bool,
}

impl SigmaSlidesPresenterEngine {
    pub fn new(title: &str) -> Self {
        Self {
            presentation_title: title.to_string(),
            slides: Vec::new(),
            current_slide_idx: 0,
            is_presenter_mode: false,
        }
    }

    pub fn add_slide(&mut self, title: &str, body_markdown: &str, notes: &str) -> u32 {
        let slide_number = (self.slides.len() as u32) + 1;
        self.slides.push(PresentationSlide {
            slide_number,
            title: title.to_string(),
            body_markdown: body_markdown.to_string(),
            speaker_notes: notes.to_string(),
        });
        slide_number
    }

    pub fn start_presentation(&mut self) {
        self.current_slide_idx = 0;
        self.is_presenter_mode = true;
    }

    pub fn next_slide(&mut self) -> Option<&PresentationSlide> {
        if self.current_slide_idx + 1 < self.slides.len() {
            self.current_slide_idx += 1;
            Some(&self.slides[self.current_slide_idx])
        } else {
            self.get_current_slide()
        }
    }

    pub fn get_current_slide(&self) -> Option<&PresentationSlide> {
        self.slides.get(self.current_slide_idx)
    }
}

// =========================================================================
// 3. Google Docs / MS Word Parity Real-time Collaboration Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveCollaboratorCursor {
    pub user_id: String,
    pub user_name: String,
    pub cursor_position: usize,
    pub selection_length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSuggestion {
    pub suggestion_id: String,
    pub author: String,
    pub original_text: String,
    pub suggested_text: String,
    pub is_accepted: bool,
}

#[derive(Debug, Clone)]
pub struct SigmaDocsEnterpriseCollaborationEngine {
    pub document_id: String,
    pub content: String,
    pub cursors: BTreeMap<String, LiveCollaboratorCursor>,
    pub suggestions: Vec<DocumentSuggestion>,
}

impl SigmaDocsEnterpriseCollaborationEngine {
    pub fn new(doc_id: &str, initial_content: &str) -> Self {
        Self {
            document_id: doc_id.to_string(),
            content: initial_content.to_string(),
            cursors: BTreeMap::new(),
            suggestions: Vec::new(),
        }
    }

    pub fn update_cursor(&mut self, cursor: LiveCollaboratorCursor) {
        self.cursors.insert(cursor.user_id.clone(), cursor);
    }

    pub fn propose_suggestion(&mut self, author: &str, orig: &str, replacement: &str) -> String {
        let sug_id = format!("sug-{}", self.suggestions.len() + 1);
        self.suggestions.push(DocumentSuggestion {
            suggestion_id: sug_id.clone(),
            author: author.to_string(),
            original_text: orig.to_string(),
            suggested_text: replacement.to_string(),
            is_accepted: false,
        });
        sug_id
    }

    pub fn accept_suggestion(&mut self, sug_id: &str) -> bool {
        if let Some(sug) = self
            .suggestions
            .iter_mut()
            .find(|s| s.suggestion_id == sug_id)
        {
            if !sug.is_accepted {
                sug.is_accepted = true;
                self.content = self
                    .content
                    .replace(&sug.original_text, &sug.suggested_text);
                return true;
            }
        }
        false
    }
}

// =========================================================================
// 4. Zoho CRM / Salesforce Parity Sales Pipeline & Lead Engine
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeadStage {
    New,
    Contacted,
    Qualified,
    ProposalSent,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrmLead {
    pub lead_id: String,
    pub contact_name: String,
    pub company: String,
    pub estimated_value_usd: f64,
    pub stage: LeadStage,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct SovereignEnterpriseCrmErpEngine {
    pub leads: BTreeMap<String, CrmLead>,
}

impl SovereignEnterpriseCrmErpEngine {
    pub fn new() -> Self {
        Self {
            leads: BTreeMap::new(),
        }
    }

    pub fn add_lead(&mut self, lead: CrmLead) {
        self.leads.insert(lead.lead_id.clone(), lead);
    }

    pub fn update_stage(&mut self, lead_id: &str, new_stage: LeadStage) -> bool {
        if let Some(lead) = self.leads.get_mut(lead_id) {
            lead.stage = new_stage;
            true
        } else {
            false
        }
    }

    pub fn calculate_pipeline_forecast(&self) -> f64 {
        let mut total = 0.0;
        for lead in self.leads.values() {
            let probability = match lead.stage {
                LeadStage::New => 0.10,
                LeadStage::Contacted => 0.25,
                LeadStage::Qualified => 0.50,
                LeadStage::ProposalSent => 0.75,
                LeadStage::ClosedWon => 1.00,
                LeadStage::ClosedLost => 0.00,
            };
            total += lead.estimated_value_usd * probability;
        }
        total
    }
}

impl Default for SovereignEnterpriseCrmErpEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Odoo / Bitrix24 Parity ERP, Kanban & HR Clock-In Suite
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: u32,
    pub unit_price_cents: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiCurrencyInvoice {
    pub invoice_id: String,
    pub customer_name: String,
    pub currency_code: String, // "USD", "EUR", "INR"
    pub items: Vec<InvoiceItem>,
    pub is_paid: bool,
}

impl MultiCurrencyInvoice {
    pub fn calculate_total_cents(&self) -> u64 {
        self.items
            .iter()
            .map(|item| (item.quantity as u64) * item.unit_price_cents)
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HrAttendanceClock {
    pub employee_id: String,
    pub clock_in_sec: u64,
    pub clock_out_sec: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SovereignOdooBitrixSuite {
    pub invoices: BTreeMap<String, MultiCurrencyInvoice>,
    pub attendance_logs: Vec<HrAttendanceClock>,
}

impl SovereignOdooBitrixSuite {
    pub fn new() -> Self {
        Self {
            invoices: BTreeMap::new(),
            attendance_logs: Vec::new(),
        }
    }

    pub fn issue_invoice(&mut self, invoice: MultiCurrencyInvoice) {
        self.invoices.insert(invoice.invoice_id.clone(), invoice);
    }

    pub fn clock_in(&mut self, emp_id: &str, now_sec: u64) {
        self.attendance_logs.push(HrAttendanceClock {
            employee_id: emp_id.to_string(),
            clock_in_sec: now_sec,
            clock_out_sec: None,
        });
    }

    pub fn clock_out(&mut self, emp_id: &str, now_sec: u64) -> bool {
        if let Some(log) = self
            .attendance_logs
            .iter_mut()
            .rev()
            .find(|l| l.employee_id == emp_id && l.clock_out_sec.is_none())
        {
            log.clock_out_sec = Some(now_sec);
            true
        } else {
            false
        }
    }
}

impl Default for SovereignOdooBitrixSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looker_analytics_engine() {
        let mut engine = SigmaLookerAnalyticsEngine::new("Sales Dashboard");
        engine.add_widget(AnalyticsChartWidget {
            widget_id: "w1".to_string(),
            title: "Revenue".to_string(),
            chart_type: "bar".to_string(),
            data_source: "sales_db".to_string(),
            metrics: vec!["amount".to_string()],
        });

        let mut r1 = BTreeMap::new();
        r1.insert("amount".to_string(), "1500.50".to_string());
        let mut r2 = BTreeMap::new();
        r2.insert("amount".to_string(), "2499.50".to_string());

        engine.ingest_record(r1);
        engine.ingest_record(r2);

        assert_eq!(engine.compute_metric_sum("amount"), 4000.0);
        assert!(engine
            .export_dashboard_summary()
            .contains("Sales Dashboard"));
    }

    #[test]
    fn test_slides_presenter_engine() {
        let mut slides = SigmaSlidesPresenterEngine::new("SigmaOS Overview");
        let s1 = slides.add_slide("Intro", "# Welcome", "Key points for slide 1");
        assert_eq!(s1, 1);

        slides.add_slide(
            "Architecture",
            "Microkernel details",
            "Key points for slide 2",
        );

        slides.start_presentation();
        assert_eq!(slides.get_current_slide().unwrap().title, "Intro");

        slides.next_slide();
        assert_eq!(slides.get_current_slide().unwrap().title, "Architecture");
    }

    #[test]
    fn test_docs_collaboration_engine() {
        let mut docs = SigmaDocsEnterpriseCollaborationEngine::new("doc-001", "Hello World");
        docs.update_cursor(LiveCollaboratorCursor {
            user_id: "user-1".to_string(),
            user_name: "Alice".to_string(),
            cursor_position: 5,
            selection_length: 0,
        });

        let sug_id = docs.propose_suggestion("Bob", "World", "SigmaOS");
        assert!(docs.accept_suggestion(&sug_id));
        assert_eq!(docs.content, "Hello SigmaOS");
    }

    #[test]
    fn test_crm_erp_engine() {
        let mut crm = SovereignEnterpriseCrmErpEngine::new();
        crm.add_lead(CrmLead {
            lead_id: "lead-101".to_string(),
            contact_name: "John Doe".to_string(),
            company: "Acme Corp".to_string(),
            estimated_value_usd: 10000.0,
            stage: LeadStage::Qualified,
            score: 80,
        });

        assert_eq!(crm.calculate_pipeline_forecast(), 5000.0);

        crm.update_stage("lead-101", LeadStage::ClosedWon);
        assert_eq!(crm.calculate_pipeline_forecast(), 10000.0);
    }

    #[test]
    fn test_odoo_bitrix_suite() {
        let mut suite = SovereignOdooBitrixSuite::new();
        suite.issue_invoice(MultiCurrencyInvoice {
            invoice_id: "inv-001".to_string(),
            customer_name: "Enterprise Client".to_string(),
            currency_code: "USD".to_string(),
            items: vec![InvoiceItem {
                description: "License".to_string(),
                quantity: 2,
                unit_price_cents: 5000,
            }],
            is_paid: false,
        });

        let inv = suite.invoices.get("inv-001").unwrap();
        assert_eq!(inv.calculate_total_cents(), 10000);

        suite.clock_in("emp-1", 1000);
        assert!(suite.clock_out("emp-1", 5000));
    }
}
