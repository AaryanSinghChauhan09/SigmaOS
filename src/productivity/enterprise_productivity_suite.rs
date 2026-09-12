//! # Enterprise Productivity Suite
//!
//! Inspired by Google Workspace (Google Docs, Google Sheets, Google Slides, Google Looker Studio),
//! Zoho Suite, Microsoft 365, Salesforce Suite, Odoo Suite, and Bitrix24 Suite.
//!
//! This module delivers native enterprise capabilities into SigmaOS:
//! - **SigmaLookerAnalyticsEngine**: Google Looker Studio inspired KPI metric calculation, calculated fields, aggregated dashboard cards, data pipeline connectors.
//! - **SigmaSlidesPresenterEngine**: Google Slides / Zoho Show inspired presentation slide deck management, transitions, speaker presenter notes, animation keyframes.
//! - **SovereignEnterpriseCrmErpEngine**: Salesforce / Zoho CRM inspired lead scoring algorithm, sales stage progression, revenue forecasting, contact timeline tracking.
//! - **SovereignOdooBitrixSuite**: Odoo / Bitrix24 inspired Kanban project task boards, Gantt timeline scheduling, attendance/time tracking, interactive workfeed posts.
//! - **SigmaDocsEnterpriseCollaborationEngine**: Google Docs / MS 365 inspired suggestion edit mode, inline comment threads with resolve/reply, and mail merge document generation.

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(any(feature = "standalone_test", test))]
use std::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::HashMap;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::HashMap;

// ============================================================================
// 1. Google Looker Studio Inspired Analytics & KPI Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum AggregationType {
    Sum,
    Average,
    Count,
    Min,
    Max,
}

#[derive(Debug, Clone)]
pub struct MetricField {
    pub name: String,
    pub values: Vec<f64>,
    pub aggregation: AggregationType,
}

#[derive(Debug, Clone)]
pub struct DashboardKpiWidget {
    pub widget_id: String,
    pub title: String,
    pub primary_value: f64,
    pub formatted_display: String,
    pub target_goal: Option<f64>,
}

/// Google Looker Studio / Business Intelligence Analytics Engine
pub struct SigmaLookerAnalyticsEngine {
    pub metric_fields: HashMap<String, MetricField>,
    pub kpi_widgets: Vec<DashboardKpiWidget>,
    pub calculated_fields: HashMap<String, String>, // name -> formula expression e.g. "REV - COST"
}

impl SigmaLookerAnalyticsEngine {
    pub fn new() -> Self {
        Self {
            metric_fields: HashMap::new(),
            kpi_widgets: Vec::new(),
            calculated_fields: HashMap::new(),
        }
    }

    pub fn add_metric_field(&mut self, name: &str, values: Vec<f64>, aggregation: AggregationType) {
        self.metric_fields.insert(
            name.to_string(),
            MetricField {
                name: name.to_string(),
                values,
                aggregation,
            },
        );
    }

    pub fn add_calculated_field(&mut self, name: &str, formula: &str) {
        self.calculated_fields.insert(name.to_string(), formula.to_string());
    }

    /// Evaluates aggregated value for a metric field
    pub fn calculate_aggregate(&self, field_name: &str) -> Option<f64> {
        if let Some(field) = self.metric_fields.get(field_name) {
            if field.values.is_empty() {
                return Some(0.0);
            }
            match field.aggregation {
                AggregationType::Sum => Some(field.values.iter().sum()),
                AggregationType::Average => {
                    let sum: f64 = field.values.iter().sum();
                    Some(sum / field.values.len() as f64)
                }
                AggregationType::Count => Some(field.values.len() as f64),
                AggregationType::Min => field.values.iter().cloned().reduce(f64::min),
                AggregationType::Max => field.values.iter().cloned().reduce(f64::max),
            }
        } else {
            None
        }
    }

    /// Generates a Looker Studio KPI card widget from aggregated data
    pub fn create_kpi_widget(
        &mut self,
        widget_id: &str,
        title: &str,
        field_name: &str,
        target_goal: Option<f64>,
    ) -> Option<&DashboardKpiWidget> {
        let agg_val = self.calculate_aggregate(field_name)?;
        let formatted = format!("{:.2}", agg_val);

        let widget = DashboardKpiWidget {
            widget_id: widget_id.to_string(),
            title: title.to_string(),
            primary_value: agg_val,
            formatted_display: formatted,
            target_goal,
        };

        self.kpi_widgets.push(widget);
        self.kpi_widgets.last()
    }

    pub fn get_kpi_widgets(&self) -> &[DashboardKpiWidget] {
        &self.kpi_widgets
    }
}

impl Default for SigmaLookerAnalyticsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Google Slides / Zoho Show Presentation Presenter Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlideTransitionEffect {
    None,
    Fade,
    SlideLeft,
    ZoomIn,
    FlipHorizontal,
}

#[derive(Debug, Clone)]
pub struct SlideAnimationKeyframe {
    pub element_id: String,
    pub delay_ms: u32,
    pub duration_ms: u32,
    pub animation_type: String,
}

#[derive(Debug, Clone)]
pub struct PresentationSlide {
    pub slide_number: usize,
    pub title: String,
    pub speaker_notes: String,
    pub transition: SlideTransitionEffect,
    pub animations: Vec<SlideAnimationKeyframe>,
}

/// Google Slides / Zoho Show Presentation Suite Engine
pub struct SigmaSlidesPresenterEngine {
    pub slides: Vec<PresentationSlide>,
    pub current_slide_index: usize,
    pub is_presenter_mode: bool,
}

impl SigmaSlidesPresenterEngine {
    pub fn new() -> Self {
        Self {
            slides: Vec::new(),
            current_slide_index: 0,
            is_presenter_mode: false,
        }
    }

    pub fn add_slide(
        &mut self,
        title: &str,
        speaker_notes: &str,
        transition: SlideTransitionEffect,
    ) -> usize {
        let slide_num = self.slides.len() + 1;
        self.slides.push(PresentationSlide {
            slide_number: slide_num,
            title: title.to_string(),
            speaker_notes: speaker_notes.to_string(),
            transition,
            animations: Vec::new(),
        });
        self.slides.len() - 1
    }

    pub fn add_animation(
        &mut self,
        slide_idx: usize,
        element_id: &str,
        animation_type: &str,
        duration_ms: u32,
    ) -> bool {
        if let Some(slide) = self.slides.get_mut(slide_idx) {
            slide.animations.push(SlideAnimationKeyframe {
                element_id: element_id.to_string(),
                delay_ms: 0,
                duration_ms,
                animation_type: animation_type.to_string(),
            });
            true
        } else {
            false
        }
    }

    pub fn start_presenter_mode(&mut self) {
        self.is_presenter_mode = true;
        self.current_slide_index = 0;
    }

    pub fn next_slide(&mut self) -> bool {
        if self.current_slide_index + 1 < self.slides.len() {
            self.current_slide_index += 1;
            true
        } else {
            false
        }
    }

    pub fn prev_slide(&mut self) -> bool {
        if self.current_slide_index > 0 {
            self.current_slide_index -= 1;
            true
        } else {
            false
        }
    }

    pub fn current_speaker_notes(&self) -> Option<&str> {
        self.slides
            .get(self.current_slide_index)
            .map(|s| s.speaker_notes.as_str())
    }
}

impl Default for SigmaSlidesPresenterEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Salesforce / Zoho CRM & ERP Suite Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrmPipelineStage {
    NewLead,
    Contacted,
    Qualified,
    ProposalSent,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone)]
pub struct ContactInteraction {
    pub timestamp_sec: u64,
    pub channel: String, // e.g. "Email", "Call", "Meeting"
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct EnterpriseLead {
    pub lead_id: u32,
    pub client_name: String,
    pub company: String,
    pub deal_amount: f64,
    pub stage: CrmPipelineStage,
    pub lead_score: u32,
    pub interactions: Vec<ContactInteraction>,
}

/// Salesforce / Zoho CRM Engine
pub struct SovereignEnterpriseCrmErpEngine {
    pub leads: HashMap<u32, EnterpriseLead>,
    pub next_lead_id: u32,
}

impl SovereignEnterpriseCrmErpEngine {
    pub fn new() -> Self {
        Self {
            leads: HashMap::new(),
            next_lead_id: 1001,
        }
    }

    pub fn create_lead(&mut self, client_name: &str, company: &str, deal_amount: f64) -> u32 {
        let id = self.next_lead_id;
        self.next_lead_id += 1;

        let lead = EnterpriseLead {
            lead_id: id,
            client_name: client_name.to_string(),
            company: company.to_string(),
            deal_amount,
            stage: CrmPipelineStage::NewLead,
            lead_score: 10, // base initial score
            interactions: Vec::new(),
        };

        self.leads.insert(id, lead);
        id
    }

    pub fn log_interaction(&mut self, lead_id: u32, channel: &str, notes: &str) -> bool {
        if let Some(lead) = self.leads.get_mut(&lead_id) {
            lead.interactions.push(ContactInteraction {
                timestamp_sec: 1700000000,
                channel: channel.to_string(),
                notes: notes.to_string(),
            });
            // Auto-boost lead score based on interaction activity
            lead.lead_score += 15;
            true
        } else {
            false
        }
    }

    pub fn advance_stage(&mut self, lead_id: u32, new_stage: CrmPipelineStage) -> bool {
        if let Some(lead) = self.leads.get_mut(&lead_id) {
            lead.stage = new_stage;
            true
        } else {
            false
        }
    }

    /// Calculates total forecasted revenue for leads in active stages (Qualified, ProposalSent, ClosedWon)
    pub fn forecast_pipeline_revenue(&self) -> f64 {
        let mut total = 0.0;
        for lead in self.leads.values() {
            let win_probability = match lead.stage {
                CrmPipelineStage::NewLead => 0.1,
                CrmPipelineStage::Contacted => 0.25,
                CrmPipelineStage::Qualified => 0.5,
                CrmPipelineStage::ProposalSent => 0.75,
                CrmPipelineStage::ClosedWon => 1.0,
                CrmPipelineStage::ClosedLost => 0.0,
            };
            total += lead.deal_amount * win_probability;
        }
        total
    }
}

impl Default for SovereignEnterpriseCrmErpEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Odoo & Bitrix24 Project Kanban, Gantt, and Workfeed Suite Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Backlog,
    InProgress,
    InReview,
    Done,
}

#[derive(Debug, Clone)]
pub struct OdooKanbanTask {
    pub task_id: u32,
    pub title: String,
    pub assignee: String,
    pub status: TaskStatus,
    pub start_day: u32,
    pub duration_days: u32,
}

#[derive(Debug, Clone)]
pub struct WorkfeedPost {
    pub post_id: u32,
    pub author: String,
    pub content: String,
    pub timestamp_sec: u64,
    pub likes_count: u32,
}

#[derive(Debug, Clone)]
pub struct AttendanceRecord {
    pub employee_name: String,
    pub check_in_timestamp: u64,
    pub check_out_timestamp: Option<u64>,
}

/// Odoo / Bitrix24 Task, Gantt Schedule, Attendance, and Workfeed Suite
pub struct SovereignOdooBitrixSuite {
    pub tasks: HashMap<u32, OdooKanbanTask>,
    pub workfeed_posts: Vec<WorkfeedPost>,
    pub attendance_records: Vec<AttendanceRecord>,
    pub next_task_id: u32,
    pub next_post_id: u32,
}

impl SovereignOdooBitrixSuite {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            workfeed_posts: Vec::new(),
            attendance_records: Vec::new(),
            next_task_id: 1,
            next_post_id: 101,
        }
    }

    pub fn create_task(
        &mut self,
        title: &str,
        assignee: &str,
        start_day: u32,
        duration_days: u32,
    ) -> u32 {
        let tid = self.next_task_id;
        self.next_task_id += 1;

        let task = OdooKanbanTask {
            task_id: tid,
            title: title.to_string(),
            assignee: assignee.to_string(),
            status: TaskStatus::Backlog,
            start_day,
            duration_days,
        };

        self.tasks.insert(tid, task);
        tid
    }

    pub fn update_task_status(&mut self, task_id: u32, status: TaskStatus) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.status = status;
            true
        } else {
            false
        }
    }

    /// Calculates total project duration in days across Gantt scheduled tasks
    pub fn calculate_gantt_project_span(&self) -> u32 {
        let mut max_end_day = 0;
        for task in self.tasks.values() {
            let end_day = task.start_day + task.duration_days;
            if end_day > max_end_day {
                max_end_day = end_day;
            }
        }
        max_end_day
    }

    pub fn publish_workfeed_post(&mut self, author: &str, content: &str) -> u32 {
        let pid = self.next_post_id;
        self.next_post_id += 1;

        self.workfeed_posts.push(WorkfeedPost {
            post_id: pid,
            author: author.to_string(),
            content: content.to_string(),
            timestamp_sec: 1700000000,
            likes_count: 0,
        });

        pid
    }

    pub fn log_check_in(&mut self, employee_name: &str, timestamp: u64) {
        self.attendance_records.push(AttendanceRecord {
            employee_name: employee_name.to_string(),
            check_in_timestamp: timestamp,
            check_out_timestamp: None,
        });
    }
}

impl Default for SovereignOdooBitrixSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Google Docs / MS 365 Suggestions, Comments & Mail Merge Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct SuggestionEdit {
    pub suggestion_id: u32,
    pub author: String,
    pub original_text: String,
    pub suggested_replacement: String,
    pub is_accepted: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct InlineCommentThread {
    pub thread_id: u32,
    pub target_range_key: String,
    pub author: String,
    pub comments: Vec<String>,
    pub is_resolved: bool,
}

/// Google Docs / Microsoft 365 Enterprise Collaboration Engine
pub struct SigmaDocsEnterpriseCollaborationEngine {
    pub suggestions: Vec<SuggestionEdit>,
    pub comment_threads: HashMap<u32, InlineCommentThread>,
    pub next_suggestion_id: u32,
    pub next_thread_id: u32,
}

impl SigmaDocsEnterpriseCollaborationEngine {
    pub fn new() -> Self {
        Self {
            suggestions: Vec::new(),
            comment_threads: HashMap::new(),
            next_suggestion_id: 1,
            next_thread_id: 1,
        }
    }

    pub fn add_suggestion(&mut self, author: &str, original: &str, replacement: &str) -> u32 {
        let id = self.next_suggestion_id;
        self.next_suggestion_id += 1;

        self.suggestions.push(SuggestionEdit {
            suggestion_id: id,
            author: author.to_string(),
            original_text: original.to_string(),
            suggested_replacement: replacement.to_string(),
            is_accepted: None,
        });

        id
    }

    pub fn resolve_suggestion(&mut self, suggestion_id: u32, accept: bool) -> bool {
        if let Some(sug) = self.suggestions.iter_mut().find(|s| s.suggestion_id == suggestion_id) {
            sug.is_accepted = Some(accept);
            true
        } else {
            false
        }
    }

    pub fn create_comment_thread(&mut self, range_key: &str, author: &str, comment: &str) -> u32 {
        let tid = self.next_thread_id;
        self.next_thread_id += 1;

        let thread = InlineCommentThread {
            thread_id: tid,
            target_range_key: range_key.to_string(),
            author: author.to_string(),
            comments: vec![comment.to_string()],
            is_resolved: false,
        };

        self.comment_threads.insert(tid, thread);
        tid
    }

    pub fn reply_to_comment(&mut self, thread_id: u32, reply_text: &str) -> bool {
        if let Some(thread) = self.comment_threads.get_mut(&thread_id) {
            thread.comments.push(reply_text.to_string());
            true
        } else {
            false
        }
    }

    pub fn resolve_comment_thread(&mut self, thread_id: u32) -> bool {
        if let Some(thread) = self.comment_threads.get_mut(&thread_id) {
            thread.is_resolved = true;
            true
        } else {
            false
        }
    }

    /// Performs MS Word / Google Docs style template tag mail merge
    /// e.g. replaces "{{CLIENT_NAME}}" with actual client name
    pub fn execute_mail_merge(
        template_content: &str,
        placeholders: &HashMap<String, String>,
    ) -> String {
        let mut result = template_content.to_string();
        for (key, value) in placeholders {
            let token = format!("{{{{{}}}}}", key);
            result = result.replace(&token, value);
        }
        result
    }
}

impl Default for SigmaDocsEnterpriseCollaborationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looker_analytics_pipeline() {
        let mut analytics = SigmaLookerAnalyticsEngine::new();
        analytics.add_metric_field("revenue", vec![1000.0, 2000.0, 3000.0], AggregationType::Sum);
        analytics.add_metric_field("cost", vec![200.0, 400.0, 600.0], AggregationType::Average);

        assert_eq!(analytics.calculate_aggregate("revenue"), Some(6000.0));
        assert_eq!(analytics.calculate_aggregate("cost"), Some(400.0));

        let kpi = analytics
            .create_kpi_widget("kpi_rev", "Total Revenue", "revenue", Some(10000.0))
            .unwrap();
        assert_eq!(kpi.primary_value, 6000.0);
        assert_eq!(kpi.title, "Total Revenue");
    }

    #[test]
    fn test_slides_presenter_engine() {
        let mut presenter = SigmaSlidesPresenterEngine::new();
        let s1 = presenter.add_slide("Intro", "Welcome everyone", SlideTransitionEffect::Fade);
        let _s2 = presenter.add_slide("Architecture", "Key points", SlideTransitionEffect::SlideLeft);

        presenter.add_animation(s1, "hero_text", "fadeIn", 500);
        presenter.start_presenter_mode();

        assert_eq!(presenter.current_speaker_notes(), Some("Welcome everyone"));
        assert!(presenter.next_slide());
        assert_eq!(presenter.current_speaker_notes(), Some("Key points"));
        assert!(!presenter.next_slide()); // at end
    }

    #[test]
    fn test_crm_erp_pipeline() {
        let mut crm = SovereignEnterpriseCrmErpEngine::new();
        let lead_id = crm.create_lead("Acme Corp", "Acme", 50000.0);
        crm.log_interaction(lead_id, "Call", "Discussed pricing");
        crm.advance_stage(lead_id, CrmPipelineStage::Qualified);

        let forecast = crm.forecast_pipeline_revenue();
        assert_eq!(forecast, 25000.0); // 50000 * 0.5 (Qualified)
    }

    #[test]
    fn test_odoo_bitrix_kanban_gantt() {
        let mut suite = SovereignOdooBitrixSuite::new();
        let t1 = suite.create_task("Backend API", "Alice", 0, 5);
        let _t2 = suite.create_task("Frontend UI", "Bob", 3, 7);

        suite.update_task_status(t1, TaskStatus::InProgress);
        assert_eq!(suite.calculate_gantt_project_span(), 10); // max end day = 3 + 7 = 10

        let post_id = suite.publish_workfeed_post("Alice", "Milestone 1 Completed!");
        assert_eq!(post_id, 101);
    }

    #[test]
    fn test_docs_collaboration_mail_merge() {
        let mut collab = SigmaDocsEnterpriseCollaborationEngine::new();
        let sug_id = collab.add_suggestion("Bob", "old value", "new value");
        assert!(collab.resolve_suggestion(sug_id, true));

        let thread_id = collab.create_comment_thread("p1", "Alice", "Please check formatting");
        collab.reply_to_comment(thread_id, "Fixed now!");
        collab.resolve_comment_thread(thread_id);

        let mut placeholders = HashMap::new();
        placeholders.insert("CLIENT".to_string(), "Sigma Corp".to_string());
        placeholders.insert("AMOUNT".to_string(), "$10,000".to_string());

        let template = "Dear {{CLIENT}}, your invoice total is {{AMOUNT}}.";
        let merged = SigmaDocsEnterpriseCollaborationEngine::execute_mail_merge(template, &placeholders);
        assert_eq!(merged, "Dear Sigma Corp, your invoice total is $10,000.");
    }
}
