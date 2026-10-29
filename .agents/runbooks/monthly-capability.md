# Monthly Capability Runbook — SigmaOS

## Objective
Deliver one vertical OS capability milestone according to the 10-month roadmap, validated by runtime evidence.

## Workflow
1. Review monthly theme and required evidence in `.agents/tasks/open/CAP-YYYY-Mxx.toml`.
2. Ensure unit tests, integration tests, and QEMU boot traces pass.
3. Verify evidence logs in `reports/`:
   - `reports/build/build-manifest.json`
   - `reports/test/test-report.json`
   - `reports/boot/qemu-x86_64.log`
   - `reports/boot/boot-matrix.json`
4. Confirm Definition of Done checklist.
5. Save monthly report to `research/reports/monthly/` and update capability matrix status with evidence link.
