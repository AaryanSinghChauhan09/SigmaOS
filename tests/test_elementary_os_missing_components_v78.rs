// SPDX-License-Identifier: MIT
// Integration Test for SigmaOS Elementary Linux Missing Components Suite V78
// (`tests/test_elementary_os_missing_components_v78.rs`)

#[path = "../src/desktop/elementary_os_missing_components_v78.rs"]
mod elementary_os_missing_components_v78;

use elementary_os_missing_components_v78::*;

#[test]
fn test_elementary_missing_components_v78_integration_suite() {
    let mut suite = ElementaryOsMissingComponentsV78Suite::new();

    suite.files.open_new_tab("/home/user/Downloads");
    assert_eq!(suite.files.open_tabs.len(), 2);

    suite.code.set_document_content("// TODO: optimize performance");
    assert_eq!(suite.code.diagnostics.len(), 1);

    suite.mail.add_account("dev@sigmaos.org");
    assert_eq!(suite.mail.accounts.len(), 1);

    suite.music.play_track("Symphony.flac");
    assert!(suite.music.is_playing);

    suite.terminal.new_tab();
    assert_eq!(suite.terminal.open_tabs, 2);

    let res = suite.calculator.calculate("10 * 5");
    assert_eq!(res, 50.0);

    let task_id = suite.tasks.add_task("Review PR #7817");
    assert_eq!(task_id, 1);

    let event_id = suite.calendar.add_event("Release V78", "14:00");
    assert_eq!(event_id, 1);

    assert!(suite.shortcuts.toggle_overlay());

    suite.onboarding.complete_step("dark_mode_schedule");
    assert_eq!(suite.onboarding.completed_steps.len(), 1);

    let specs = suite.collect_all_pr_specs();
    assert_eq!(specs.len(), 10);
    assert_eq!(suite.compute_coverage_score(), 100);

    for spec in specs {
        assert!(spec.status_verified);
        assert_eq!(spec.coverage_score, 100);
        assert!(!spec.repository.is_empty());
        assert!(!spec.title.is_empty());
    }
}
