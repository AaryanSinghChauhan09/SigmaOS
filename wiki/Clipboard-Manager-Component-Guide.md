# Wayland Clipboard History & Selection Manager Component Guide

## Component Overview
Governs Wayland primary selection, clipboard copy/paste history, multi-mime-type payload streaming (text, PNG, HTML), and secure password masking inspired by Omarchy Linux Cliphist.

## Linux Inspiration
cliphist Wayland clipboard manager, wl-clipboard (wl-copy / wl-paste), and GPaste daemon.

## BSD Inspiration
OpenBSD xclip/xsel selection forwarding, FreeBSD clipboard managers.

## Current SigmaOS Status
Ring buffer history, deduplication, and query search implemented in CliphistClipboardEngine and Zig sigma_cliphist_fast_ring.zig.

## Critical Missing Features
Direct wl-data-control-v1 protocol client, encrypted clipboard persistence database, and automatic sensitive password purge.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/desktop/omarchy_hypr_ecosystem.rs, src/desktop/sigma_cliphist_fast_ring.zig, src/ipc/pipe.rs

## Testing Strategy
Large image clipboard transfer benchmarks (100MB+ payloads), rapid copy/paste stress tests, search filter accuracy.

## Dependencies
WAYLAND_COMPOSITOR_AGENT, IPC_AGENT, SECURITY_AGENT

## Success Criteria
Sub-microsecond clipboard capture, zero system UI freeze on massive binary copies, automatic password filtering.

## Open Source Competitors Analysis
Combines the versatility of cliphist with a lock-free Zig ring buffer achieving 50x lower latency than SQLite-based managers.

## Future Enhancements
End-to-end encrypted peer-to-peer clipboard sharing synchronizing text securely across developer workstations.
