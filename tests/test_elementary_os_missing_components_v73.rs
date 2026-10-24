// SPDX-License-Identifier: MIT
// Integration Test for Elementary OS Missing Components V73 Suite
// (`tests/test_elementary_os_missing_components_v73.rs`)

#[path = "../src/desktop/elementary_os_missing_components_v73.rs"]
mod elementary_os_missing_components_v73;

use elementary_os_missing_components_v73::*;

#[test]
fn test_elementary_os_missing_components_v73_suite() {
    let mut suite = ElementaryOsMissingComponentsV73Suite::new();

    suite.files.open_new_tab("/home/user/Documents");
    assert_eq!(suite.files.open_tabs.len(), 2);

    suite.code.set_document_content("// TODO: fix bug v73");
    assert_eq!(suite.code.diagnostics.len(), 1);

    suite.mail.add_account("user_v73@example.com");
    assert_eq!(suite.mail.accounts.len(), 1);

    suite.music.play_track("Track1_v73.flac");
    assert!(suite.music.is_playing);

    suite.terminal.new_tab();
    assert_eq!(suite.terminal.open_tabs, 2);

    let ans = suite.calculator.calculate("2 + 2");
    assert_eq!(ans, 4.0);

    let tid = suite.tasks.add_task("Buy groceries v73");
    assert_eq!(tid, 1);

    let eid = suite.calendar.add_event("Meeting v73", "10:00");
    assert_eq!(eid, 1);

    assert!(suite.shortcuts.toggle_overlay());

    suite.onboarding.complete_step("theme_select_v73");
    assert_eq!(suite.onboarding.completed_steps.len(), 1);

    let specs = suite.collect_all_pr_specs();
    assert_eq!(specs.len(), 10);
    assert_eq!(suite.compute_coverage_score(), 100);
}
