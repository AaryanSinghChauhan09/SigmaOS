// SPDX-License-Identifier: MIT
// Integration Test for SigmaOS Elementary Linux Missing Components Suite V78
// (`tests/test_elementary_os_missing_components_v78.rs`)

#[path = "../src/desktop/elementary_os_missing_components_v78.rs"]
mod elementary_os_missing_components_v78;

use elementary_os_missing_components_v78::*;

#[test]
fn test_v78_elementary_os_missing_components_integration() {
    let mut suite = ElementaryOsMissingComponentsV78Suite::new();

    let tab_id = suite.files.new_tab("/home/user/Downloads");
    assert_eq!(tab_id, 1);

    suite.code.load_file("/home/user/lib.rs");
    assert_eq!(suite.code.run_diagnostics(), 1);

    suite.mail.add_account("developer@sigmaos.org");
    assert_eq!(suite.mail.accounts.len(), 1);

    suite.music.play_track("Track2.flac");
    assert!(suite.music.is_playing);

    suite.terminal.new_tab();
    assert_eq!(suite.terminal.open_tabs, 2);

    let val = suite.calculator.calculate("10 / 2");
    assert_eq!(val, 4.0);

    let tid = suite.tasks.add_task("Review V78 PR");
    assert_eq!(tid, 1);

    let eid = suite.calendar.add_event("Release V78", "12:00");
    assert_eq!(eid, 1);

    assert!(suite.shortcuts.toggle_overlay());

    suite.onboarding.complete_step("privacy_setup");
    assert_eq!(suite.onboarding.completed_steps.len(), 1);

    let specs = suite.collect_all_pr_specs();
    assert_eq!(specs.len(), 10);
    assert_eq!(suite.compute_coverage_score(), 100);
}
