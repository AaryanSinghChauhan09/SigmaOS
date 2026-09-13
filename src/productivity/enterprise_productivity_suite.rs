// Enterprise Productivity Suite for SigmaOS
// Inspired by Google Workspace (Looker Studio, Slides, Docs), Zoho Suites, Salesforce, Odoo, and Bitrix24.

use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::HashMap;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::HashMap;

// =========================================================================
// 1. Google Looker Studio / PowerBI Analytics Engine (SigmaLookerAnalyticsEngine)
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ChartMetricType {
    Sum,
    Average,
    Count,
    Min,
    Max,
}

#[derive(Debug, Clone)]
pub struct DataDimension {
    pub dimension_id: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DataMetric {
    pub metric_id: String,
    pub name: String,
    pub values: Vec<f64>,
    pub aggregation: ChartMetricType,
}

#[derive(Debug, Clone)]
pub struct AnalyticsWidget {
    pub widget_id: String,
    pub title: String,
    pub chart_type: String, // "bar", "pie", "line", "scorecard", "geo_map"
    pub dimension_id: String,
    pub metric_id: String,
}

/// Google Looker Studio & Microsoft PowerBI Inspired Sovereign Data Visualization Engine
pub struct SigmaLookerAnalyticsEngine {
    pub dashboard_name: String,
    pub dimensions: HashMap<String, DataDimension>,
    pub metrics: HashMap<String, DataMetric>,
    pub widgets: Vec<AnalyticsWidget>,
}

impl SigmaLookerAnalyticsEngine {
    pub fn new(dashboard_name: &str) -> Self {
        Self {
            dashboard_name: dashboard_name.to_string(),
            dimensions: HashMap::new(),
            metrics: HashMap::new(),
            widgets: Vec::new(),
        }
    }

    pub fn add_dimension(&mut self, id: &str, name: &str, values: Vec<String>) {
        self.dimensions.insert(
            id.to_string(),
            DataDimension {
                dimension_id: id.to_string(),
                name: name.to_string(),
                values,
            },
        );
    }

    pub fn add_metric(&mut self, id: &str, name: &str, values: Vec<f64>, agg: ChartMetricType) {
        self.metrics.insert(
            id.to_string(),
            DataMetric {
                metric_id: id.to_string(),
                name: name.to_string(),
                values,
                aggregation: agg,
            },
        );
    }

    pub fn add_widget(&mut self, widget_id: &str, title: &str, chart_type: &str, dim_id: &str, metric_id: &str) {
        self.widgets.push(AnalyticsWidget {
            widget_id: widget_id.to_string(),
            title: title.to_string(),
            chart_type: chart_type.to_string(),
            dimension_id: dim_id.to_string(),
            metric_id: metric_id.to_string(),
        });
    }

    pub fn compute_metric_aggregate(&self, metric_id: &str) -> Option<f64> {
        let metric = self.metrics.get(metric_id)?;
        if metric.values.is_empty() {
            return Some(0.0);
        }

        match metric.aggregation {
            ChartMetricType::Sum => Some(metric.values.iter().sum()),
            ChartMetricType::Average => Some(metric.values.iter().sum::<f64>() / metric.values.len() as f64),
            ChartMetricType::Count => Some(metric.values.len() as f64),
            ChartMetricType::Min => metric.values.iter().cloned().reduce(f64::min),
            ChartMetricType::Max => metric.values.iter().cloned().reduce(f64::max),
        }
    }

    pub fn render_widget_summary(&self, widget_id: &str) -> Result<String, &'static str> {
        let widget = self
            .widgets
            .iter()
            .find(|w| w.widget_id == widget_id)
            .ok_or("Widget not found")?;

        let dim = self
            .dimensions
            .get(&widget.dimension_id)
            .ok_or("Dimension not found")?;

        let agg = self
            .compute_metric_aggregate(&widget.metric_id)
            .ok_or("Metric not found")?;

        Ok(format!(
            "[{}] Chart: {}, Dim: {}, Aggregate: {:.2}",
            widget.title, widget.chart_type, dim.name, agg
        ))
    }
}

// =========================================================================
// 2. Google Slides / PowerPoint Presentation Engine (SigmaSlidesPresenterEngine)
// =========================================================================

#[derive(Debug, Clone)]
pub struct SlideTransition {
    pub effect_name: String, // "Fade", "Dissolve", "Slide Left", "Zoom"
    pub duration_ms: u32,
}

#[derive(Debug, Clone)]
pub struct SlideAnimation {
    pub element_id: String,
    pub animation_type: String, // "FlyIn", "FadeIn", "Spin", "Bounce"
    pub trigger_on_click: bool,
}

#[derive(Debug, Clone)]
pub struct PresentationSlide {
    pub slide_number: u32,
    pub title: String,
    pub speaker_notes: String,
    pub transition: SlideTransition,
    pub animations: Vec<SlideAnimation>,
}

/// Google Slides & Microsoft PowerPoint Inspired Sovereign Presentation Engine
pub struct SigmaSlidesPresenterEngine {
    pub presentation_title: String,
    pub slides: Vec<PresentationSlide>,
    pub active_slide_index: usize,
    pub presenter_mode_active: bool,
}

impl SigmaSlidesPresenterEngine {
    pub fn new(title: &str) -> Self {
        Self {
            presentation_title: title.to_string(),
            slides: Vec::new(),
            active_slide_index: 0,
            presenter_mode_active: false,
        }
    }

    pub fn create_slide(&mut self, title: &str, speaker_notes: &str) -> u32 {
        let slide_num = (self.slides.len() + 1) as u32;
        self.slides.push(PresentationSlide {
            slide_number: slide_num,
            title: title.to_string(),
            speaker_notes: speaker_notes.to_string(),
            transition: SlideTransition {
                effect_name: "Fade".to_string(),
                duration_ms: 300,
            },
            animations: Vec::new(),
        });
        slide_num
    }

    pub fn add_animation(&mut self, slide_num: u32, elem_id: &str, anim_type: &str, on_click: bool) -> Result<(), &'static str> {
        let slide = self
            .slides
            .iter_mut()
            .find(|s| s.slide_number == slide_num)
            .ok_or("Slide not found")?;

        slide.animations.push(SlideAnimation {
            element_id: elem_id.to_string(),
            animation_type: anim_type.to_string(),
            trigger_on_click: on_click,
        });
        Ok(())
    }

    pub fn start_presenter_view(&mut self) -> Result<String, &'static str> {
        if self.slides.is_empty() {
            return Err("Presentation has no slides");
        }
        self.presenter_mode_active = true;
        self.active_slide_index = 0;
        let slide = &self.slides[0];
        Ok(format!(
            "Presenter View Started [Slide 1/{}]: {} | Notes: {}",
            self.slides.len(),
            slide.title,
            slide.speaker_notes
        ))
    }

    pub fn next_slide(&mut self) -> Option<&PresentationSlide> {
        if self.active_slide_index + 1 < self.slides.len() {
            self.active_slide_index += 1;
            Some(&self.slides[self.active_slide_index])
        } else {
            None
        }
    }
}

// =========================================================================
// 3. Google Docs Enterprise Collaboration Engine (SigmaDocsEnterpriseCollaborationEngine)
// =========================================================================

#[derive(Debug, Clone)]
pub struct UserCursorState {
    pub username: String,
    pub cursor_offset: usize,
    pub selection_length: usize,
    pub user_color_hex: String,
}

#[derive(Debug, Clone)]
pub struct CommentThread {
    pub thread_id: u32,
    pub author: String,
    pub selected_text: String,
    pub comment_text: String,
    pub resolved: bool,
}

#[derive(Debug, Clone)]
pub struct SuggestionEdit {
    pub suggestion_id: u32,
    pub author: String,
    pub original_text: String,
    pub suggested_text: String,
    pub status: String, // "Pending", "Accepted", "Rejected"
}

/// Google Docs & MS Word Inspired Sovereign Collaborative Editor Engine
pub struct SigmaDocsEnterpriseCollaborationEngine {
    pub doc_id: String,
    pub document_text: String,
    pub live_cursors: HashMap<String, UserCursorState>,
    pub comments: Vec<CommentThread>,
    pub suggestions: Vec<SuggestionEdit>,
    pub next_id: u32,
}

impl SigmaDocsEnterpriseCollaborationEngine {
    pub fn new(doc_id: &str, initial_content: &str) -> Self {
        Self {
            doc_id: doc_id.to_string(),
            document_text: initial_content.to_string(),
            live_cursors: HashMap::new(),
            comments: Vec::new(),
            suggestions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn update_user_cursor(&mut self, username: &str, offset: usize, sel_len: usize, color: &str) {
        self.live_cursors.insert(
            username.to_string(),
            UserCursorState {
                username: username.to_string(),
                cursor_offset: offset,
                selection_length: sel_len,
                user_color_hex: color.to_string(),
            },
        );
    }

    pub fn add_comment(&mut self, author: &str, selected_text: &str, comment: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.comments.push(CommentThread {
            thread_id: id,
            author: author.to_string(),
            selected_text: selected_text.to_string(),
            comment_text: comment.to_string(),
            resolved: false,
        });
        id
    }

    pub fn propose_suggestion(&mut self, author: &str, orig: &str, suggested: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.suggestions.push(SuggestionEdit {
            suggestion_id: id,
            author: author.to_string(),
            original_text: orig.to_string(),
            suggested_text: suggested.to_string(),
            status: "Pending".to_string(),
        });
        id
    }

    pub fn accept_suggestion(&mut self, suggestion_id: u32) -> Result<bool, &'static str> {
        let sug = self
            .suggestions
            .iter_mut()
            .find(|s| s.suggestion_id == suggestion_id)
            .ok_or("Suggestion not found")?;

        if sug.status != "Pending" {
            return Ok(false);
        }

        sug.status = "Accepted".to_string();
        if self.document_text.contains(&sug.original_text) {
            self.document_text = self
                .document_text
                .replace(&sug.original_text, &sug.suggested_text);
        }
        Ok(true)
    }
}

// =========================================================================
// 4. Zoho CRM & Salesforce Enterprise Suite (SovereignEnterpriseCrmErpEngine)
// =========================================================================

#[derive(Debug, Clone)]
pub struct CrmLead {
    pub lead_id: u32,
    pub contact_name: String,
    pub company: String,
    pub lead_score: u32,
    pub status: String, // "New", "Contacted", "Qualified", "Converted"
}

#[derive(Debug, Clone)]
pub struct SalesDeal {
    pub deal_id: u32,
    pub deal_name: String,
    pub amount: f64,
    pub probability: f64, // 0.0 to 1.0
    pub stage: String,    // "Prospecting", "Proposal", "Closed-Won", "Closed-Lost"
}

#[derive(Debug, Clone)]
pub struct SupportCase {
    pub case_id: u32,
    pub customer_name: String,
    pub issue_title: String,
    pub priority: String, // "Low", "Medium", "High", "Critical"
    pub is_closed: bool,
}

/// Zoho CRM & Salesforce Inspired Sovereign Enterprise CRM Engine
pub struct SovereignEnterpriseCrmErpEngine {
    pub leads: Vec<CrmLead>,
    pub deals: Vec<SalesDeal>,
    pub cases: Vec<SupportCase>,
    pub next_id: u32,
}

impl SovereignEnterpriseCrmErpEngine {
    pub fn new() -> Self {
        Self {
            leads: Vec::new(),
            deals: Vec::new(),
            cases: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_lead(&mut self, contact: &str, company: &str, score: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.leads.push(CrmLead {
            lead_id: id,
            contact_name: contact.to_string(),
            company: company.to_string(),
            lead_score: score,
            status: "New".to_string(),
        });
        id
    }

    pub fn add_deal(&mut self, name: &str, amount: f64, prob: f64, stage: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.deals.push(SalesDeal {
            deal_id: id,
            deal_name: name.to_string(),
            amount,
            probability: prob.clamp(0.0, 1.0),
            stage: stage.to_string(),
        });
        id
    }

    pub fn compute_forecasted_pipeline_revenue(&self) -> f64 {
        self.deals
            .iter()
            .filter(|d| d.stage != "Closed-Lost")
            .map(|d| d.amount * d.probability)
            .sum()
    }

    pub fn create_support_case(&mut self, customer: &str, issue: &str, priority: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.cases.push(SupportCase {
            case_id: id,
            customer_name: customer.to_string(),
            issue_title: issue.to_string(),
            priority: priority.to_string(),
            is_closed: false,
        });
        id
    }
}

impl Default for SovereignEnterpriseCrmErpEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. Odoo & Bitrix24 Sovereign ERP & Project Management Suite (SovereignOdooBitrixSuite)
// =========================================================================

#[derive(Debug, Clone)]
pub struct KanbanTask {
    pub task_id: u32,
    pub title: String,
    pub column: String, // "Backlog", "In Progress", "Code Review", "Done"
    pub assignee: String,
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub sku: String,
    pub item_name: String,
    pub quantity_on_hand: u32,
    pub unit_price: f64,
}

#[derive(Debug, Clone)]
pub struct InvoiceLine {
    pub description: String,
    pub quantity: u32,
    pub unit_price: f64,
}

#[derive(Debug, Clone)]
pub struct Invoice {
    pub invoice_id: u32,
    pub customer_name: String,
    pub currency: String,
    pub lines: Vec<InvoiceLine>,
    pub paid: bool,
}

#[derive(Debug, Clone)]
pub struct WorkOrder {
    pub order_id: u32,
    pub product_name: String,
    pub planned_quantity: u32,
    pub status: String, // "Draft", "In Production", "Completed"
}

/// Odoo & Bitrix24 Inspired Sovereign ERP & All-in-One Suite Engine
pub struct SovereignOdooBitrixSuite {
    pub tasks: Vec<KanbanTask>,
    pub inventory: HashMap<String, InventoryItem>,
    pub invoices: Vec<Invoice>,
    pub work_orders: Vec<WorkOrder>,
    pub employee_clock_ins: HashMap<String, u64>, // employee_id -> timestamp_ms
    pub next_id: u32,
}

impl SovereignOdooBitrixSuite {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            inventory: HashMap::new(),
            invoices: Vec::new(),
            work_orders: Vec::new(),
            employee_clock_ins: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn create_kanban_task(&mut self, title: &str, assignee: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(KanbanTask {
            task_id: id,
            title: title.to_string(),
            column: "Backlog".to_string(),
            assignee: assignee.to_string(),
        });
        id
    }

    pub fn move_kanban_task(&mut self, task_id: u32, target_column: &str) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.column = target_column.to_string();
            true
        } else {
            false
        }
    }

    pub fn update_inventory(&mut self, sku: &str, name: &str, qty: u32, price: f64) {
        self.inventory.insert(
            sku.to_string(),
            InventoryItem {
                sku: sku.to_string(),
                item_name: name.to_string(),
                quantity_on_hand: qty,
                unit_price: price,
            },
        );
    }

    pub fn create_invoice(&mut self, customer: &str, currency: &str, lines: Vec<InvoiceLine>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.invoices.push(Invoice {
            invoice_id: id,
            customer_name: customer.to_string(),
            currency: currency.to_string(),
            lines,
            paid: false,
        });
        id
    }

    pub fn calculate_invoice_total(&self, invoice_id: u32) -> Option<f64> {
        let inv = self.invoices.iter().find(|i| i.invoice_id == invoice_id)?;
        Some(inv.lines.iter().map(|l| l.quantity as f64 * l.unit_price).sum())
    }

    pub fn create_manufacturing_work_order(&mut self, product: &str, qty: u32) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.work_orders.push(WorkOrder {
            order_id: id,
            product_name: product.to_string(),
            planned_quantity: qty,
            status: "Draft".to_string(),
        });
        id
    }

    pub fn hr_clock_in(&mut self, employee_id: &str, timestamp_ms: u64) {
        self.employee_clock_ins.insert(employee_id.to_string(), timestamp_ms);
    }
}

impl Default for SovereignOdooBitrixSuite {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(any(feature = "standalone_test", test))]
mod tests {
    use super::*;

    #[test]
    fn test_looker_analytics_engine() {
        let mut engine = SigmaLookerAnalyticsEngine::new("Sales Dashboard");
        engine.add_dimension("region", "Region", vec!["North".to_string(), "South".to_string()]);
        engine.add_metric("revenue", "Revenue", vec![1000.0, 2500.0, 1500.0], ChartMetricType::Sum);
        engine.add_widget("w1", "Regional Revenue", "bar", "region", "revenue");

        let agg = engine.compute_metric_aggregate("revenue").unwrap();
        assert_eq!(agg, 5000.0);

        let summary = engine.render_widget_summary("w1").unwrap();
        assert!(summary.contains("5000.00"));
    }

    #[test]
    fn test_slides_presenter_engine() {
        let mut slides = SigmaSlidesPresenterEngine::new("Keynote");
        let s1 = slides.create_slide("Intro", "Welcome everyone");
        slides.add_animation(s1, "title_box", "FlyIn", true).unwrap();

        let presenter_text = slides.start_presenter_view().unwrap();
        assert!(presenter_text.contains("Welcome everyone"));
        assert_eq!(slides.active_slide_index, 0);
    }

    #[test]
    fn test_docs_collaboration_engine() {
        let mut doc = SigmaDocsEnterpriseCollaborationEngine::new("doc1", "The Quick Brown Fox");
        doc.update_user_cursor("alice", 5, 0, "#FF0000");
        let sug_id = doc.propose_suggestion("bob", "Quick", "Fast");
        assert!(doc.accept_suggestion(sug_id).unwrap());
        assert_eq!(doc.document_text, "The Fast Brown Fox");
    }

    #[test]
    fn test_crm_erp_engine() {
        let mut crm = SovereignEnterpriseCrmErpEngine::new();
        crm.add_lead("John Doe", "Acme Corp", 85);
        crm.add_deal("Enterprise License", 100000.0, 0.8, "Proposal");
        let forecast = crm.compute_forecasted_pipeline_revenue();
        assert_eq!(forecast, 80000.0);
    }

    #[test]
    fn test_odoo_bitrix_suite() {
        let mut odoo = SovereignOdooBitrixSuite::new();
        let task_id = odoo.create_kanban_task("Setup Server", "DevOp");
        assert!(odoo.move_kanban_task(task_id, "In Progress"));

        odoo.update_inventory("SKU-100", "Widget A", 50, 19.99);

        let inv_id = odoo.create_invoice(
            "Global Tech",
            "USD",
            vec![InvoiceLine {
                description: "Consulting".to_string(),
                quantity: 10,
                unit_price: 150.0,
            }],
        );
        let total = odoo.calculate_invoice_total(inv_id).unwrap();
        assert_eq!(total, 1500.0);
    }
}
