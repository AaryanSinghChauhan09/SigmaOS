// SPDX-License-Identifier: MIT
// SigmaOS Sovereign Enterprise Productivity Suite
// High-performance, zero-dependency enterprise tools inspired by Google Workspace (Looker Studio, Slides, Docs),
// Zoho & Salesforce CRM/ERP, Microsoft 365, Odoo, and Bitrix24.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. Google Looker Studio Inspired Analytics & Reporting Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataSourceType {
    SqlDatabase,
    CsvDataset,
    RestApi,
    RealtimeStream,
}

#[derive(Debug, Clone)]
pub struct DataConnector {
    pub connector_id: String,
    pub name: String,
    pub source_type: DataSourceType,
    pub connection_uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidgetType {
    BarChart,
    PieChart,
    MetricScorecard,
    DataTable,
    LineSeries,
}

#[derive(Debug, Clone)]
pub struct DashboardWidget {
    pub widget_id: u32,
    pub title: String,
    pub widget_type: WidgetType,
    pub query: String,
    pub cached_metric: f64,
}

#[derive(Debug, Clone)]
pub struct LookerDashboard {
    pub dashboard_id: String,
    pub title: String,
    pub widgets: Vec<DashboardWidget>,
    pub auto_refresh_sec: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SigmaLookerAnalyticsEngine {
    pub connectors: BTreeMap<String, DataConnector>,
    pub dashboards: BTreeMap<String, LookerDashboard>,
}

impl SigmaLookerAnalyticsEngine {
    pub fn new() -> Self {
        Self {
            connectors: BTreeMap::new(),
            dashboards: BTreeMap::new(),
        }
    }

    pub fn register_connector(&mut self, id: &str, name: &str, source_type: DataSourceType, uri: &str) {
        self.connectors.insert(
            id.to_string(),
            DataConnector {
                connector_id: id.to_string(),
                name: name.to_string(),
                source_type,
                connection_uri: uri.to_string(),
            },
        );
    }

    pub fn create_dashboard(&mut self, dashboard_id: &str, title: &str, auto_refresh_sec: u32) {
        self.dashboards.insert(
            dashboard_id.to_string(),
            LookerDashboard {
                dashboard_id: dashboard_id.to_string(),
                title: title.to_string(),
                widgets: Vec::new(),
                auto_refresh_sec,
            },
        );
    }

    pub fn add_widget(&mut self, dashboard_id: &str, widget_id: u32, title: &str, widget_type: WidgetType, query: &str, metric: f64) -> Result<(), &'static str> {
        let db = self.dashboards.get_mut(dashboard_id).ok_or("Dashboard not found")?;
        db.widgets.push(DashboardWidget {
            widget_id,
            title: title.to_string(),
            widget_type,
            query: query.to_string(),
            cached_metric: metric,
        });
        Ok(())
    }

    pub fn generate_report_summary(&self, dashboard_id: &str) -> Result<String, &'static str> {
        let db = self.dashboards.get(dashboard_id).ok_or("Dashboard not found")?;
        let widget_count = db.widgets.len();
        let total_metric_sum: f64 = db.widgets.iter().map(|w| w.cached_metric).sum();
        Ok(format!(
            "LOOKER_REPORT: Dashboard '{}' ({} widgets) - Total Aggregate Metric Value: {:.2}",
            db.title, widget_count, total_metric_sum
        ))
    }
}

// ============================================================================
// 2. Google Slides / MS PowerPoint Inspired Presentation Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlideLayout {
    TitleSlide,
    TextAndImage,
    TwoColumns,
    BlankCanvas,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionEffect {
    None,
    Fade,
    SlideLeft,
    ZoomIn,
}

#[derive(Debug, Clone)]
pub struct PresentationSlide {
    pub slide_number: u32,
    pub title: String,
    pub layout: SlideLayout,
    pub body_content: Vec<String>,
    pub speaker_notes: String,
    pub transition: TransitionEffect,
}

#[derive(Debug, Clone)]
pub struct PresentationDeck {
    pub deck_id: String,
    pub title: String,
    pub author: String,
    pub theme_name: String,
    pub slides: Vec<PresentationSlide>,
}

#[derive(Debug, Clone, Default)]
pub struct SigmaSlidesPresenterEngine {
    pub decks: BTreeMap<String, PresentationDeck>,
}

impl SigmaSlidesPresenterEngine {
    pub fn new() -> Self {
        Self { decks: BTreeMap::new() }
    }

    pub fn create_deck(&mut self, deck_id: &str, title: &str, author: &str, theme: &str) {
        self.decks.insert(
            deck_id.to_string(),
            PresentationDeck {
                deck_id: deck_id.to_string(),
                title: title.to_string(),
                author: author.to_string(),
                theme_name: theme.to_string(),
                slides: Vec::new(),
            },
        );
    }

    pub fn add_slide(&mut self, deck_id: &str, slide_num: u32, title: &str, layout: SlideLayout, transition: TransitionEffect) -> Result<(), &'static str> {
        let deck = self.decks.get_mut(deck_id).ok_or("Presentation deck not found")?;
        deck.slides.push(PresentationSlide {
            slide_number: slide_num,
            title: title.to_string(),
            layout,
            body_content: Vec::new(),
            speaker_notes: String::new(),
            transition,
        });
        Ok(())
    }

    pub fn add_slide_text(&mut self, deck_id: &str, slide_num: u32, text: &str) -> Result<(), &'static str> {
        let deck = self.decks.get_mut(deck_id).ok_or("Presentation deck not found")?;
        if let Some(slide) = deck.slides.iter_mut().find(|s| s.slide_number == slide_num) {
            slide.body_content.push(text.to_string());
            Ok(())
        } else {
            Err("Slide number not found")
        }
    }

    pub fn set_speaker_notes(&mut self, deck_id: &str, slide_num: u32, notes: &str) -> Result<(), &'static str> {
        let deck = self.decks.get_mut(deck_id).ok_or("Presentation deck not found")?;
        if let Some(slide) = deck.slides.iter_mut().find(|s| s.slide_number == slide_num) {
            slide.speaker_notes = notes.to_string();
            Ok(())
        } else {
            Err("Slide number not found")
        }
    }

    pub fn export_deck_outline(&self, deck_id: &str) -> Result<String, &'static str> {
        let deck = self.decks.get(deck_id).ok_or("Presentation deck not found")?;
        Ok(format!(
            "SLIDES_PRESENTATION: '{}' by {} ({} slides, Theme: '{}')",
            deck.title, deck.author, deck.slides.len(), deck.theme_name
        ))
    }
}

// ============================================================================
// 3. Google Docs / Word Enterprise Collaboration Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Owner,
    Editor,
    Commenter,
    Viewer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct SuggestedEdit {
    pub edit_id: u32,
    pub author: String,
    pub original_text: String,
    pub proposed_text: String,
    pub status: EditStatus,
}

#[derive(Debug, Clone)]
pub struct CommentThread {
    pub thread_id: u32,
    pub author: String,
    pub comment_text: String,
    pub is_resolved: bool,
    pub replies: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EnterpriseDocumentSession {
    pub doc_id: String,
    pub title: String,
    pub users: BTreeMap<String, UserRole>,
    pub suggestions: Vec<SuggestedEdit>,
    pub comments: Vec<CommentThread>,
    pub next_id: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SigmaDocsEnterpriseCollaborationEngine {
    pub sessions: BTreeMap<String, EnterpriseDocumentSession>,
}

impl SigmaDocsEnterpriseCollaborationEngine {
    pub fn new() -> Self {
        Self { sessions: BTreeMap::new() }
    }

    pub fn create_document(&mut self, doc_id: &str, title: &str, owner: &str) {
        let mut users = BTreeMap::new();
        users.insert(owner.to_string(), UserRole::Owner);

        self.sessions.insert(
            doc_id.to_string(),
            EnterpriseDocumentSession {
                doc_id: doc_id.to_string(),
                title: title.to_string(),
                users,
                suggestions: Vec::new(),
                comments: Vec::new(),
                next_id: 1,
            },
        );
    }

    pub fn add_user_role(&mut self, doc_id: &str, username: &str, role: UserRole) -> Result<(), &'static str> {
        let session = self.sessions.get_mut(doc_id).ok_or("Document session not found")?;
        session.users.insert(username.to_string(), role);
        Ok(())
    }

    pub fn propose_suggested_edit(&mut self, doc_id: &str, author: &str, original: &str, proposed: &str) -> Result<u32, &'static str> {
        let session = self.sessions.get_mut(doc_id).ok_or("Document session not found")?;
        let id = session.next_id;
        session.next_id += 1;

        session.suggestions.push(SuggestedEdit {
            edit_id: id,
            author: author.to_string(),
            original_text: original.to_string(),
            proposed_text: proposed.to_string(),
            status: EditStatus::Pending,
        });
        Ok(id)
    }

    pub fn resolve_suggested_edit(&mut self, doc_id: &str, edit_id: u32, accept: bool) -> Result<(), &'static str> {
        let session = self.sessions.get_mut(doc_id).ok_or("Document session not found")?;
        if let Some(edit) = session.suggestions.iter_mut().find(|e| e.edit_id == edit_id) {
            edit.status = if accept { EditStatus::Accepted } else { EditStatus::Rejected };
            Ok(())
        } else {
            Err("Suggested edit not found")
        }
    }

    pub fn add_comment(&mut self, doc_id: &str, author: &str, comment: &str) -> Result<u32, &'static str> {
        let session = self.sessions.get_mut(doc_id).ok_or("Document session not found")?;
        let id = session.next_id;
        session.next_id += 1;

        session.comments.push(CommentThread {
            thread_id: id,
            author: author.to_string(),
            comment_text: comment.to_string(),
            is_resolved: false,
            replies: Vec::new(),
        });
        Ok(id)
    }

    pub fn resolve_comment(&mut self, doc_id: &str, thread_id: u32) -> Result<(), &'static str> {
        let session = self.sessions.get_mut(doc_id).ok_or("Document session not found")?;
        if let Some(ct) = session.comments.iter_mut().find(|c| c.thread_id == thread_id) {
            ct.is_resolved = true;
            Ok(())
        } else {
            Err("Comment thread not found")
        }
    }
}

// ============================================================================
// 4. Salesforce & Zoho Inspired Enterprise CRM / ERP Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealStage {
    Lead,
    Contacted,
    ProductDemo,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone)]
pub struct LeadRecord {
    pub lead_id: u32,
    pub company_name: String,
    pub contact_person: String,
    pub email: String,
    pub estimated_value_usd: f64,
    pub stage: DealStage,
}

#[derive(Debug, Clone)]
pub struct AccountRecord {
    pub account_id: u32,
    pub name: String,
    pub industry: String,
    pub annual_revenue_usd: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SovereignEnterpriseCrmErpEngine {
    pub leads: Vec<LeadRecord>,
    pub accounts: Vec<AccountRecord>,
    pub next_lead_id: u32,
    pub next_account_id: u32,
}

impl SovereignEnterpriseCrmErpEngine {
    pub fn new() -> Self {
        Self {
            leads: Vec::new(),
            accounts: Vec::new(),
            next_lead_id: 1,
            next_account_id: 1,
        }
    }

    pub fn add_lead(&mut self, company: &str, contact: &str, email: &str, val: f64) -> u32 {
        let id = self.next_lead_id;
        self.next_lead_id += 1;

        self.leads.push(LeadRecord {
            lead_id: id,
            company_name: company.to_string(),
            contact_person: contact.to_string(),
            email: email.to_string(),
            estimated_value_usd: val,
            stage: DealStage::Lead,
        });
        id
    }

    pub fn update_deal_stage(&mut self, lead_id: u32, new_stage: DealStage) -> Result<(), &'static str> {
        if let Some(lead) = self.leads.iter_mut().find(|l| l.lead_id == lead_id) {
            lead.stage = new_stage;
            Ok(())
        } else {
            Err("Lead record not found")
        }
    }

    pub fn convert_lead_to_account(&mut self, lead_id: u32, industry: &str) -> Result<u32, &'static str> {
        let lead = self.leads.iter().find(|l| l.lead_id == lead_id).cloned().ok_or("Lead not found")?;
        let acc_id = self.next_account_id;
        self.next_account_id += 1;

        self.accounts.push(AccountRecord {
            account_id: acc_id,
            name: lead.company_name,
            industry: industry.to_string(),
            annual_revenue_usd: lead.estimated_value_usd,
        });

        self.update_deal_stage(lead_id, DealStage::ClosedWon)?;
        Ok(acc_id)
    }

    pub fn get_pipeline_forecast_usd(&self) -> f64 {
        self.leads
            .iter()
            .filter(|l| l.stage != DealStage::ClosedLost)
            .map(|l| l.estimated_value_usd)
            .sum()
    }
}

// ============================================================================
// 5. Odoo & Bitrix24 Inspired Integrated Enterprise Suite
// ============================================================================

#[derive(Debug, Clone)]
pub struct EmployeeRecord {
    pub emp_id: u32,
    pub name: String,
    pub department: String,
    pub role: String,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct InvoiceItem {
    pub description: String,
    pub unit_price: f64,
    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct InvoiceRecord {
    pub invoice_id: u32,
    pub client_name: String,
    pub items: Vec<InvoiceItem>,
    pub is_paid: bool,
}

#[derive(Debug, Clone)]
pub struct IntranetPost {
    pub post_id: u32,
    pub author: String,
    pub content: String,
    pub likes_count: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SovereignOdooBitrixSuite {
    pub employees: Vec<EmployeeRecord>,
    pub invoices: Vec<InvoiceRecord>,
    pub intranet_feed: Vec<IntranetPost>,
    pub next_emp_id: u32,
    pub next_inv_id: u32,
    pub next_post_id: u32,
}

impl SovereignOdooBitrixSuite {
    pub fn new() -> Self {
        Self {
            employees: Vec::new(),
            invoices: Vec::new(),
            intranet_feed: Vec::new(),
            next_emp_id: 1,
            next_inv_id: 1,
            next_post_id: 1,
        }
    }

    pub fn hire_employee(&mut self, name: &str, dept: &str, role: &str) -> u32 {
        let id = self.next_emp_id;
        self.next_emp_id += 1;

        self.employees.push(EmployeeRecord {
            emp_id: id,
            name: name.to_string(),
            department: dept.to_string(),
            role: role.to_string(),
            is_active: true,
        });
        id
    }

    pub fn create_invoice(&mut self, client: &str, items: Vec<InvoiceItem>) -> u32 {
        let id = self.next_inv_id;
        self.next_inv_id += 1;

        self.invoices.push(InvoiceRecord {
            invoice_id: id,
            client_name: client.to_string(),
            items,
            is_paid: false,
        });
        id
    }

    pub fn mark_invoice_paid(&mut self, inv_id: u32) -> Result<(), &'static str> {
        if let Some(inv) = self.invoices.iter_mut().find(|i| i.invoice_id == inv_id) {
            inv.is_paid = true;
            Ok(())
        } else {
            Err("Invoice not found")
        }
    }

    pub fn post_to_intranet(&mut self, author: &str, content: &str) -> u32 {
        let id = self.next_post_id;
        self.next_post_id += 1;

        self.intranet_feed.push(IntranetPost {
            post_id: id,
            author: author.to_string(),
            content: content.to_string(),
            likes_count: 0,
        });
        id
    }

    pub fn get_enterprise_overview(&self) -> String {
        let total_invoice_val: f64 = self
            .invoices
            .iter()
            .map(|inv| inv.items.iter().map(|item| item.unit_price * item.quantity as f64).sum::<f64>())
            .sum();

        format!(
            "ODOO_BITRIX_OVERVIEW: Active Employees: {}, Outstanding Invoices: {}, Total Invoice Value: ${:.2}, Feed Posts: {}",
            self.employees.iter().filter(|e| e.is_active).count(),
            self.invoices.iter().filter(|i| !i.is_paid).count(),
            total_invoice_val,
            self.intranet_feed.len()
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looker_analytics_engine() {
        let mut looker = SigmaLookerAnalyticsEngine::new();
        looker.register_connector("c1", "Sales DB", DataSourceType::SqlDatabase, "postgres://localhost:5432/db");
        looker.create_dashboard("d1", "Q3 Performance", 300);

        assert!(looker.add_widget("d1", 1, "Revenue Trend", WidgetType::LineSeries, "SELECT rev FROM sales", 125000.0).is_ok());
        assert!(looker.add_widget("d1", 2, "New Users", WidgetType::MetricScorecard, "SELECT count(*) FROM users", 450.0).is_ok());

        let summary = looker.generate_report_summary("d1").unwrap();
        assert!(summary.contains("Q3 Performance"));
        assert!(summary.contains("125450.00"));
    }

    #[test]
    fn test_slides_presenter_engine() {
        let mut slides = SigmaSlidesPresenterEngine::new();
        slides.create_deck("deck1", "SigmaOS Architecture", "Engineering Team", "Modern Dark");

        assert!(slides.add_slide("deck1", 1, "Intro", SlideLayout::TitleSlide, TransitionEffect::Fade).is_ok());
        assert!(slides.add_slide_text("deck1", 1, "Welcome to Sovereign Operating System").is_ok());
        assert!(slides.set_speaker_notes("deck1", 1, "Emphasize zero-dependency Rust kernel").is_ok());

        let outline = slides.export_deck_outline("deck1").unwrap();
        assert!(outline.contains("SigmaOS Architecture"));
        assert!(outline.contains("1 slides"));
    }

    #[test]
    fn test_docs_collaboration_engine() {
        let mut docs = SigmaDocsEnterpriseCollaborationEngine::new();
        docs.create_document("doc1", "Q4 Strategy", "alice");
        assert!(docs.add_user_role("doc1", "bob", UserRole::Editor).is_ok());

        let edit_id = docs.propose_suggested_edit("doc1", "bob", "old text", "new text").unwrap();
        assert_eq!(edit_id, 1);
        assert!(docs.resolve_suggested_edit("doc1", edit_id, true).is_ok());

        let comment_id = docs.add_comment("doc1", "alice", "Great suggestion").unwrap();
        assert!(docs.resolve_comment("doc1", comment_id).is_ok());
    }

    #[test]
    fn test_crm_erp_engine() {
        let mut crm = SovereignEnterpriseCrmErpEngine::new();
        let lead_id = crm.add_lead("Acme Corp", "John Doe", "john@acme.com", 50000.0);
        assert_eq!(lead_id, 1);

        assert!(crm.update_deal_stage(lead_id, DealStage::ProductDemo).is_ok());
        assert_eq!(crm.get_pipeline_forecast_usd(), 50000.0);

        let acc_id = crm.convert_lead_to_account(lead_id, "Enterprise Software").unwrap();
        assert_eq!(acc_id, 1);
        assert_eq!(crm.accounts.len(), 1);
    }

    #[test]
    fn test_odoo_bitrix_suite() {
        let mut suite = SovereignOdooBitrixSuite::new();
        let emp_id = suite.hire_employee("Jane Smith", "Engineering", "Principal Architect");
        assert_eq!(emp_id, 1);

        let inv_id = suite.create_invoice("Acme Corp", vec![
            InvoiceItem { description: "Software License".to_string(), unit_price: 1000.0, quantity: 5 },
        ]);
        assert_eq!(inv_id, 1);

        suite.post_to_intranet("Jane Smith", "All systems operational for launch!");

        let overview = suite.get_enterprise_overview();
        assert!(overview.contains("Active Employees: 1"));
        assert!(overview.contains("Total Invoice Value: $5000.00"));

        assert!(suite.mark_invoice_paid(inv_id).is_ok());
    }
}
