# SigmaOS Roadmap: Immutable Root Filesystem
Mount root as read-only with overlay for transient writes (OSTree-style).
## Goals
- dm-verity protected root partition
- OverlayFS writable upper layer for /etc and /var
## Key Milestones
- [ ] dm-verity hash tree construction tool
- [ ] OverlayFS mount point management
- [ ] Atomic root update with A/B partitions