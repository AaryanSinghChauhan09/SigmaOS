// SigmaOS Productivity Module
pub mod advanced_app_absorber;
pub mod calendar;
pub mod clipboard_manager;
pub mod document_engine;
pub mod editor;
pub mod email;
pub mod finance;
pub mod flint_chart;
pub mod gamification;
pub mod linux_bsd_tools;
pub mod media;
pub mod mind_map;
pub mod mint_competitor;
pub mod reminders_advanced;
pub mod sovereign_apps;
pub mod subtitle_editor;
pub mod tmux;
pub mod itsfoss_apps_synthesis;

pub use itsfoss_apps_synthesis::{
    IptvChannelNode, ItsFossBulkyBatchRenamerEngine, ItsFossHypnotixIptvEngine,
    ItsFossStickyNotesEngine, ItsFossWarpinatorLanSharingEngine, RenameRuleResult,
    SovereignItsFossAppsSuite, StickyNoteEntry, WarpinatorLanPeer,
};

pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use media::{AudioChannel, SigmaMediaEngine, GLOBAL_MEDIA_ENGINE};
pub use sovereign_apps::{
    ProductivityTask, SigmaOfficeDocument, SigmaTasksBoard, SigmaVaultContainer, TaskPriority,
    TextNode,
};
pub use reminders_advanced::{
    EnhancedRemindersEngine, RecurrencePattern, ReminderItem, ReminderPriority,
};
pub use tmux::{
    LayoutPreset, SplitDirection, TmuxPane, TmuxSession, TmuxSessionManager, TmuxWindow,
};

pub use mind_map::{IndentedTextMindMapParserEngine, MindMapCreator, MindMapNode, MindMapLayout, NodeShape, NodeStyle, RelationshipConnection};

pub mod cloud_enterprise_suite;
pub use cloud_enterprise_suite::{
    AnalyticsWidgetType, CellCoord, CrmLeadOpportunity, CrmLeadStage,
    DashboardAnalyticsWidget, DocumentOperation, ErpProjectTask, ErpTaskStatus,
    GoogleDocsCollaborativeDocumentEngine, GoogleLookerStudioBIAnalyticsDashboard,
    GoogleSheetsCellFormulaAnalyticsEngine, GoogleSlidesPresentationPresenterEngine,
    OdooBitrix24EnterpriseErpTaskSupervisor, PresentationSlide,
    SalesforceZohoCrmLeadPipelineGovernor, SlideTransition, SpreadsheetCellValue,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_docs_collaborative_document() {
        let mut doc = GoogleDocsCollaborativeDocumentEngine::new("doc-001", "Q3 Strategic Report");
        doc.join_collaborator(101);
        doc.join_collaborator(102);

        assert_eq!(doc.collaborator_count(), 2);

        let insert_op = DocumentOperation::InsertText {
            position: 0,
            text: "SigmaOS Enterprise Growth Plan".to_string(),
            author_id: 101,
        };
        assert!(doc.apply_operation(insert_op).is_ok());
        assert_eq!(doc.get_content(), "SigmaOS Enterprise Growth Plan");
        assert_eq!(doc.revision_count(), 1);
    }

    #[test]
    fn test_google_sheets_formula_analytics() {
        let mut sheet = GoogleSheetsCellFormulaAnalyticsEngine::new("Sales Q1", 5, 5);
        assert!(sheet.set_cell(0, 0, SpreadsheetCellValue::Number(100.0)).is_ok());
        assert!(sheet.set_cell(0, 1, SpreadsheetCellValue::Number(200.0)).is_ok());
        assert!(sheet.set_cell(1, 0, SpreadsheetCellValue::Number(300.0)).is_ok());
        assert!(sheet.set_cell(1, 1, SpreadsheetCellValue::Number(400.0)).is_ok());

        let sum = sheet.evaluate_sum_range(0, 0, 1, 1);
        assert_eq!(sum, 1000.0);

        let avg = sheet.evaluate_average_range(0, 0, 1, 1);
        assert_eq!(avg, 250.0);
    }

    #[test]
    fn test_google_slides_presenter() {
        let mut deck = GoogleSlidesPresentationPresenterEngine::new("Keynote Presentation");
        deck.add_slide("Introduction", &["Welcome to SigmaOS", "Zero-Dependency Architecture"], "Introduce core values", SlideTransition::Fade);
        deck.add_slide("Market Strategy", &["Defeating monolithic distros"], "Highlight benchmarks", SlideTransition::SlideFromRight);

        assert_eq!(deck.total_slides(), 2);
        let curr = deck.current_slide().unwrap();
        assert_eq!(curr.title, "Introduction");

        assert!(deck.next_slide());
        let curr_next = deck.current_slide().unwrap();
        assert_eq!(curr_next.title, "Market Strategy");
    }

    #[test]
    fn test_google_looker_studio_dashboard() {
        let mut dashboard = GoogleLookerStudioBIAnalyticsDashboard::new("Executive KPI Dashboard", "SigmaAnalyticsDB");
        dashboard.add_widget("widget-bar-1", "Quarterly Revenue", AnalyticsWidgetType::BarChart, "Quarter", "RevenueUSD");

        let sample_data = [("Q1", 150000.0), ("Q2", 220000.0), ("Q3", 310000.0)];
        assert!(dashboard.populate_widget_data("widget-bar-1", &sample_data));

        let widget = dashboard.get_widget("widget-bar-1").unwrap();
        assert_eq!(widget.data_points.len(), 3);
        assert_eq!(widget.data_points[2].1, 310000.0);
    }

    #[test]
    fn test_salesforce_zoho_crm_pipeline() {
        let mut crm = SalesforceZohoCrmLeadPipelineGovernor::new("Enterprise Enterprise Deal Flow");
        crm.add_lead("lead-100", "Alice Smith", "Acme Corp", 500000.0);

        assert_eq!(crm.total_leads(), 1);
        assert_eq!(crm.count_by_stage(CrmLeadStage::New), 1);

        assert!(crm.advance_lead_stage("lead-100", CrmLeadStage::Proposal));
        assert_eq!(crm.count_by_stage(CrmLeadStage::Proposal), 1);

        let weighted_val = crm.calculate_weighted_pipeline_value();
        assert_eq!(weighted_val, 375000.0); // 500,000 * 75%
    }

    #[test]
    fn test_odoo_bitrix24_erp_task_supervisor() {
        let mut erp = OdooBitrix24EnterpriseErpTaskSupervisor::new("SigmaOS 1.0 Release");
        erp.create_task("task-1", "Harden kernel buddy allocator", "Jules");
        erp.create_task("task-2", "Optimize Zenith framebuffer blit", "Bolt");

        assert!(erp.log_hours("task-1", 12.5));
        assert!(erp.log_hours("task-2", 8.0));
        assert_eq!(erp.total_hours_logged(), 20.5);

        assert!(erp.set_task_status("task-1", ErpTaskStatus::Done));
        assert_eq!(erp.completion_rate_percentage(), 50.0);
    }
}
