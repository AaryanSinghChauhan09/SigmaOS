// SigmaOS Enterprise Productivity Suite
// Inspired by Google Workspace (Looker Studio, Slides, Docs), Zoho Suite, Salesforce, Odoo, and Bitrix24.
// Purpose-built high-performance enterprise engines for analytics, presentations, collaboration, CRM, and ERP.

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::HashMap;

#[cfg(any(feature = "standalone_test", test))]
use std::{
    collections::HashMap,
    string::{String, ToString},
    vec,
    vec::Vec,
};

// ==========================================================
// 1. Google Looker Studio Inspired Analytics & Business Intelligence
// ==========================================================

/// Metric aggregation types for Looker analytics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricAggregation {
    Sum,
    Average,
    Count,
    Min,
    Max,
}

/// Data row for Looker Studio analytics engine
#[derive(Debug, Clone)]
pub struct AnalyticsDataRow {
    pub dimension_values: HashMap<String, String>,
    pub metric_values: HashMap<String, f64>,
}

/// Pivot chart aggregation result
#[derive(Debug, Clone)]
pub struct PivotResult {
    pub dimension_key: String,
    pub aggregated_metrics: HashMap<String, f64>,
    pub sample_count: usize,
}

/// Google Looker Studio inspired Business Intelligence and Analytics Engine
pub struct SigmaLookerAnalyticsEngine {
    pub report_title: String,
    pub dimensions: Vec<String>,
    pub metrics: Vec<String>,
    pub rows: Vec<AnalyticsDataRow>,
}

impl SigmaLookerAnalyticsEngine {
    pub fn new(title: &str) -> Self {
        Self {
            report_title: title.to_string(),
            dimensions: Vec::new(),
            metrics: Vec::new(),
            rows: Vec::new(),
        }
    }

    pub fn add_dimension(&mut self, dim_name: &str) {
        if !self.dimensions.contains(&dim_name.to_string()) {
            self.dimensions.push(dim_name.to_string());
        }
    }

    pub fn add_metric(&mut self, metric_name: &str) {
        if !self.metrics.contains(&metric_name.to_string()) {
            self.metrics.push(metric_name.to_string());
        }
    }

    pub fn add_row(&mut self, row: AnalyticsDataRow) {
        self.rows.push(row);
    }

    /// Computes pivot aggregation grouped by a target dimension
    pub fn compute_pivot_table(
        &self,
        group_dimension: &str,
        metric_name: &str,
        agg: MetricAggregation,
    ) -> Vec<PivotResult> {
        let mut groups: HashMap<String, Vec<f64>> = HashMap::new();

        for row in &self.rows {
            if let Some(dim_val) = row.dimension_values.get(group_dimension) {
                if let Some(&metric_val) = row.metric_values.get(metric_name) {
                    groups
                        .entry(dim_val.clone())
                        .or_insert_with(Vec::new)
                        .push(metric_val);
                }
            }
        }

        let mut results = Vec::new();

        for (dim_key, values) in groups {
            if values.is_empty() {
                continue;
            }

            let agg_value = match agg {
                MetricAggregation::Sum => values.iter().sum(),
                MetricAggregation::Average => values.iter().sum::<f64>() / values.len() as f64,
                MetricAggregation::Count => values.len() as f64,
                MetricAggregation::Min => values.iter().cloned().fold(f64::MAX, f64::min),
                MetricAggregation::Max => values.iter().cloned().fold(f64::MIN, f64::max),
            };

            let mut agg_map = HashMap::new();
            agg_map.insert(metric_name.to_string(), agg_value);

            results.push(PivotResult {
                dimension_key: dim_key,
                aggregated_metrics: agg_map,
                sample_count: values.len(),
            });
        }

        results
    }

    /// Evaluates a calculated field formula across rows (e.g. "Revenue - Cost")
    pub fn add_calculated_metric(
        &mut self,
        new_metric_name: &str,
        metric_a: &str,
        metric_b: &str,
        operation: char,
    ) {
        self.add_metric(new_metric_name);
        for row in &mut self.rows {
            let val_a = row.metric_values.get(metric_a).cloned().unwrap_or(0.0);
            let val_b = row.metric_values.get(metric_b).cloned().unwrap_or(0.0);
            let result = match operation {
                '+' => val_a + val_b,
                '-' => val_a - val_b,
                '*' => val_a * val_b,
                '/' => {
                    if val_b != 0.0 {
                        val_a / val_b
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            };
            row.metric_values.insert(new_metric_name.to_string(), result);
        }
    }
}

// ==========================================================
// 2. Google Slides & Microsoft PowerPoint Inspired Presenter Engine
// ==========================================================

/// Slide transition effect styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideTransitionEffect {
    None,
    Fade,
    SlideLeft,
    ZoomIn,
    Dissolve,
}

/// Individual slide structure
#[derive(Debug, Clone)]
pub struct PresentationSlide {
    pub slide_id: u32,
    pub title: String,
    pub subtitle: String,
    pub speaker_notes: String,
    pub transition: SlideTransitionEffect,
    pub background_color_hex: String,
}

/// Google Slides / PowerPoint inspired Slide Presenter Engine
pub struct SigmaSlidesPresenterEngine {
    pub presentation_title: String,
    pub slides: Vec<PresentationSlide>,
    pub current_slide_index: usize,
}

impl SigmaSlidesPresenterEngine {
    pub fn new(title: &str) -> Self {
        Self {
            presentation_title: title.to_string(),
            slides: Vec::new(),
            current_slide_index: 0,
        }
    }

    pub fn add_slide(&mut self, title: &str, subtitle: &str) -> u32 {
        let slide_id = (self.slides.len() + 1) as u32;
        self.slides.push(PresentationSlide {
            slide_id,
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            speaker_notes: String::new(),
            transition: SlideTransitionEffect::Fade,
            background_color_hex: "#FFFFFF".to_string(),
        });
        slide_id
    }

    pub fn set_speaker_notes(&mut self, slide_id: u32, notes: &str) -> bool {
        if let Some(slide) = self.slides.iter_mut().find(|s| s.slide_id == slide_id) {
            slide.speaker_notes = notes.to_string();
            true
        } else {
            false
        }
    }

    pub fn set_slide_transition(&mut self, slide_id: u32, effect: SlideTransitionEffect) -> bool {
        if let Some(slide) = self.slides.iter_mut().find(|s| s.slide_id == slide_id) {
            slide.transition = effect;
            true
        } else {
            false
        }
    }

    pub fn next_slide(&mut self) -> Option<&PresentationSlide> {
        if self.current_slide_index + 1 < self.slides.len() {
            self.current_slide_index += 1;
            Some(&self.slides[self.current_slide_index])
        } else {
            None
        }
    }

    pub fn prev_slide(&mut self) -> Option<&PresentationSlide> {
        if self.current_slide_index > 0 {
            self.current_slide_index -= 1;
            Some(&self.slides[self.current_slide_index])
        } else {
            None
        }
    }
}

// ==========================================================
// 3. Google Docs Inspired Real-time Collaboration Engine
// ==========================================================

/// Active collaborator presence
#[derive(Debug, Clone)]
pub struct ActiveCollaborator {
    pub user_id: String,
    pub username: String,
    pub cursor_position: usize,
    pub selection_range: (usize, usize),
    pub color_badge_hex: String,
}

/// Operational Transformation (OT) delta record for concurrent editing
#[derive(Debug, Clone)]
pub struct OtDeltaRecord {
    pub revision: u64,
    pub author_id: String,
    pub operation_type: String, // "insert", "delete", "replace"
    pub position: usize,
    pub content: String,
}

/// Google Docs inspired enterprise real-time collaboration engine
pub struct SigmaDocsEnterpriseCollaborationEngine {
    pub document_id: String,
    pub active_collaborators: HashMap<String, ActiveCollaborator>,
    pub revision_history: Vec<OtDeltaRecord>,
    pub current_revision: u64,
}

impl SigmaDocsEnterpriseCollaborationEngine {
    pub fn new(doc_id: &str) -> Self {
        Self {
            document_id: doc_id.to_string(),
            active_collaborators: HashMap::new(),
            revision_history: Vec::new(),
            current_revision: 0,
        }
    }

    pub fn join_session(&mut self, user_id: &str, username: &str, color_hex: &str) {
        self.active_collaborators.insert(
            user_id.to_string(),
            ActiveCollaborator {
                user_id: user_id.to_string(),
                username: username.to_string(),
                cursor_position: 0,
                selection_range: (0, 0),
                color_badge_hex: color_hex.to_string(),
            },
        );
    }

    pub fn update_cursor(&mut self, user_id: &str, cursor_pos: usize, sel_end: usize) -> bool {
        if let Some(collab) = self.active_collaborators.get_mut(user_id) {
            collab.cursor_position = cursor_pos;
            collab.selection_range = (cursor_pos, sel_end);
            true
        } else {
            false
        }
    }

    pub fn submit_delta(&mut self, author_id: &str, op_type: &str, pos: usize, content: &str) -> u64 {
        self.current_revision += 1;
        let delta = OtDeltaRecord {
            revision: self.current_revision,
            author_id: author_id.to_string(),
            operation_type: op_type.to_string(),
            position: pos,
            content: content.to_string(),
        };
        self.revision_history.push(delta);
        self.current_revision
    }

    pub fn leave_session(&mut self, user_id: &str) -> bool {
        self.active_collaborators.remove(user_id).is_some()
    }
}

// ==========================================================
// 4. Zoho & Salesforce Inspired Sovereign CRM Suite
// ==========================================================

/// Deal pipeline status stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DealStage {
    Lead,
    Qualification,
    ProposalSent,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

/// Customer CRM Lead entry
#[derive(Debug, Clone)]
pub struct CrmLead {
    pub lead_id: u32,
    pub company_name: String,
    pub contact_person: String,
    pub email: String,
    pub deal_value: f64,
    pub stage: DealStage,
    pub engagement_score: u32, // 0 to 100 lead score
}

/// Sovereign Enterprise CRM & Sales Automation Engine (Zoho/Salesforce Parity)
pub struct SovereignEnterpriseCrmErpEngine {
    pub leads: Vec<CrmLead>,
    pub next_lead_id: u32,
}

impl SovereignEnterpriseCrmErpEngine {
    pub fn new() -> Self {
        Self {
            leads: Vec::new(),
            next_lead_id: 1,
        }
    }

    pub fn create_lead(
        &mut self,
        company: &str,
        contact: &str,
        email: &str,
        deal_value: f64,
    ) -> u32 {
        let id = self.next_lead_id;
        self.next_lead_id += 1;

        self.leads.push(CrmLead {
            lead_id: id,
            company_name: company.to_string(),
            contact_person: contact.to_string(),
            email: email.to_string(),
            deal_value,
            stage: DealStage::Lead,
            engagement_score: 10, // Initial score
        });

        id
    }

    pub fn advance_deal_stage(&mut self, lead_id: u32, new_stage: DealStage) -> bool {
        if let Some(lead) = self.leads.iter_mut().find(|l| l.lead_id == lead_id) {
            lead.stage = new_stage;
            match new_stage {
                DealStage::Qualification => lead.engagement_score += 20,
                DealStage::ProposalSent => lead.engagement_score += 25,
                DealStage::Negotiation => lead.engagement_score += 25,
                DealStage::ClosedWon => lead.engagement_score = 100,
                DealStage::ClosedLost => lead.engagement_score = 0,
                _ => {}
            }
            true
        } else {
            false
        }
    }

    /// Calculates pipeline value forecast weighted by stage probabilities
    pub fn calculate_weighted_pipeline_forecast(&self) -> f64 {
        let mut total = 0.0;
        for lead in &self.leads {
            let win_probability = match lead.stage {
                DealStage::Lead => 0.10,
                DealStage::Qualification => 0.25,
                DealStage::ProposalSent => 0.50,
                DealStage::Negotiation => 0.75,
                DealStage::ClosedWon => 1.00,
                DealStage::ClosedLost => 0.00,
            };
            total += lead.deal_value * win_probability;
        }
        total
    }
}

impl Default for SovereignEnterpriseCrmErpEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// 5. Odoo & Bitrix24 Inspired ERP & Workflow Automation Suite
// ==========================================================

/// Task Kanban status for Bitrix24/Odoo project management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KanbanTaskStatus {
    Backlog,
    InProgress,
    InReview,
    Done,
}

/// Kanban task card entry
#[derive(Debug, Clone)]
pub struct KanbanTask {
    pub task_id: u32,
    pub title: String,
    pub assignee: String,
    pub status: KanbanTaskStatus,
    pub priority: u32, // 1 (low) to 5 (urgent)
}

/// Inventory warehouse item for Odoo ERP tracking
#[derive(Debug, Clone)]
pub struct InventoryStockItem {
    pub sku: String,
    pub product_name: String,
    pub quantity_on_hand: u32,
    pub unit_price: f64,
    pub reorder_threshold: u32,
}

/// Automated workflow trigger rule
#[derive(Debug, Clone)]
pub struct WorkflowTriggerRule {
    pub rule_name: String,
    pub event_name: String,
    pub action_script: String,
    pub active: bool,
}

/// Odoo & Bitrix24 inspired ERP inventory, task kanban, and workflow automation suite
pub struct SovereignOdooBitrixSuite {
    pub inventory: HashMap<String, InventoryStockItem>,
    pub kanban_tasks: Vec<KanbanTask>,
    pub workflow_rules: Vec<WorkflowTriggerRule>,
    pub next_task_id: u32,
}

impl SovereignOdooBitrixSuite {
    pub fn new() -> Self {
        Self {
            inventory: HashMap::new(),
            kanban_tasks: Vec::new(),
            workflow_rules: Vec::new(),
            next_task_id: 1,
        }
    }

    pub fn add_stock_item(&mut self, sku: &str, name: &str, qty: u32, price: f64, reorder: u32) {
        self.inventory.insert(
            sku.to_string(),
            InventoryStockItem {
                sku: sku.to_string(),
                product_name: name.to_string(),
                quantity_on_hand: qty,
                unit_price: price,
                reorder_threshold: reorder,
            },
        );
    }

    pub fn check_reorder_alerts(&self) -> Vec<String> {
        let mut alerts = Vec::new();
        for item in self.inventory.values() {
            if item.quantity_on_hand <= item.reorder_threshold {
                alerts.push(item.sku.clone());
            }
        }
        alerts
    }

    pub fn create_kanban_task(&mut self, title: &str, assignee: &str, priority: u32) -> u32 {
        let task_id = self.next_task_id;
        self.next_task_id += 1;

        self.kanban_tasks.push(KanbanTask {
            task_id,
            title: title.to_string(),
            assignee: assignee.to_string(),
            status: KanbanTaskStatus::Backlog,
            priority,
        });

        task_id
    }

    pub fn move_kanban_task(&mut self, task_id: u32, new_status: KanbanTaskStatus) -> bool {
        if let Some(task) = self.kanban_tasks.iter_mut().find(|t| t.task_id == task_id) {
            task.status = new_status;
            true
        } else {
            false
        }
    }

    pub fn register_workflow_trigger(&mut self, name: &str, event: &str, action: &str) {
        self.workflow_rules.push(WorkflowTriggerRule {
            rule_name: name.to_string(),
            event_name: event.to_string(),
            action_script: action.to_string(),
            active: true,
        });
    }

    pub fn trigger_event(&self, event_name: &str) -> Vec<String> {
        let mut executed_actions = Vec::new();
        for rule in &self.workflow_rules {
            if rule.active && rule.event_name == event_name {
                executed_actions.push(rule.action_script.clone());
            }
        }
        executed_actions
    }
}

impl Default for SovereignOdooBitrixSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// Unit Tests
// ==========================================================

#[cfg(any(feature = "standalone_test", test))]
mod tests {
    use super::*;

    #[test]
    fn test_looker_analytics_engine() {
        let mut engine = SigmaLookerAnalyticsEngine::new("Q3 Revenue Report");
        engine.add_dimension("Region");
        engine.add_metric("Sales");
        engine.add_metric("Cost");

        let mut row1_dim = HashMap::new();
        row1_dim.insert("Region".to_string(), "North".to_string());
        let mut row1_met = HashMap::new();
        row1_met.insert("Sales".to_string(), 1000.0);
        row1_met.insert("Cost".to_string(), 400.0);
        engine.add_row(AnalyticsDataRow {
            dimension_values: row1_dim,
            metric_values: row1_met,
        });

        let mut row2_dim = HashMap::new();
        row2_dim.insert("Region".to_string(), "North".to_string());
        let mut row2_met = HashMap::new();
        row2_met.insert("Sales".to_string(), 2000.0);
        row2_met.insert("Cost".to_string(), 800.0);
        engine.add_row(AnalyticsDataRow {
            dimension_values: row2_dim,
            metric_values: row2_met,
        });

        engine.add_calculated_metric("Profit", "Sales", "Cost", '-');
        assert_eq!(engine.rows[0].metric_values.get("Profit"), Some(&600.0));
        assert_eq!(engine.rows[1].metric_values.get("Profit"), Some(&1200.0));

        let pivot = engine.compute_pivot_table("Region", "Profit", MetricAggregation::Sum);
        assert_eq!(pivot.len(), 1);
        assert_eq!(pivot[0].dimension_key, "North");
        assert_eq!(pivot[0].aggregated_metrics.get("Profit"), Some(&1800.0));
    }

    #[test]
    fn test_slides_presenter_engine() {
        let mut slides_engine = SigmaSlidesPresenterEngine::new("SigmaOS Keynote");
        let id1 = slides_engine.add_slide("Welcome to SigmaOS", "The Sovereign Operating System");
        let id2 = slides_engine.add_slide("Architecture Overview", "Zero-Copy Sovereign IPC");

        assert!(slides_engine.set_speaker_notes(id1, "Remember to highlight boot latency <1ms"));
        assert!(slides_engine.set_slide_transition(id2, SlideTransitionEffect::ZoomIn));

        assert_eq!(slides_engine.slides.len(), 2);
        assert_eq!(slides_engine.current_slide_index, 0);

        let next_slide = slides_engine.next_slide().unwrap();
        assert_eq!(next_slide.slide_id, id2);
        assert_eq!(slides_engine.current_slide_index, 1);
    }

    #[test]
    fn test_docs_collaboration_engine() {
        let mut doc_engine = SigmaDocsEnterpriseCollaborationEngine::new("doc_sovereign_spec");
        doc_engine.join_session("usr_1", "Alice", "#FF0000");
        doc_engine.join_session("usr_2", "Bob", "#00FF00");

        assert_eq!(doc_engine.active_collaborators.len(), 2);

        doc_engine.update_cursor("usr_1", 10, 15);
        assert_eq!(
            doc_engine
                .active_collaborators
                .get("usr_1")
                .unwrap()
                .cursor_position,
            10
        );

        let rev1 = doc_engine.submit_delta("usr_1", "insert", 10, "SigmaOS");
        assert_eq!(rev1, 1);
        assert_eq!(doc_engine.revision_history.len(), 1);

        doc_engine.leave_session("usr_2");
        assert_eq!(doc_engine.active_collaborators.len(), 1);
    }

    #[test]
    fn test_enterprise_crm_erp_engine() {
        let mut crm = SovereignEnterpriseCrmErpEngine::new();
        let lead_id = crm.create_lead("Acme Corp", "John Doe", "john@acme.com", 100000.0);

        assert_eq!(crm.leads.len(), 1);
        assert_eq!(crm.leads[0].stage, DealStage::Lead);

        crm.advance_deal_stage(lead_id, DealStage::ProposalSent);
        assert_eq!(crm.leads[0].stage, DealStage::ProposalSent);
        assert_eq!(crm.leads[0].engagement_score, 35);

        let forecast = crm.calculate_weighted_pipeline_forecast();
        assert_eq!(forecast, 50000.0); // 100,000 * 0.50
    }

    #[test]
    fn test_odoo_bitrix_suite() {
        let mut suite = SovereignOdooBitrixSuite::new();
        suite.add_stock_item("SKU-001", "Sigma Workstation", 5, 2500.0, 10);
        suite.add_stock_item("SKU-002", "Sigma Server Blade", 20, 5000.0, 5);

        let reorder_alerts = suite.check_reorder_alerts();
        assert_eq!(reorder_alerts, vec!["SKU-001".to_string()]);

        let task_id = suite.create_kanban_task("Implement Odoo integration", "Alice", 1);
        assert!(suite.move_kanban_task(task_id, KanbanTaskStatus::InProgress));
        assert_eq!(suite.kanban_tasks[0].status, KanbanTaskStatus::InProgress);

        suite.register_workflow_trigger(
            "AutoNotifyOnLead",
            "lead_created",
            "send_email_notification()",
        );
        let actions = suite.trigger_event("lead_created");
        assert_eq!(actions, vec!["send_email_notification()".to_string()]);
    }
}
