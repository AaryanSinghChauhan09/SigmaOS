# Fairness and Starvation Prevention Policy

The scheduler must bound starvation for runnable tasks while respecting
priority, deadlines, and explicit interactive-workload policy. Aging or
deadline correction must be deterministic and visible in queue selection.

Evidence includes long-run service distribution, priority inversion handling,
wake-up fairness, and a regression test for starved-task detection. A policy is
not supported solely because a single interactive trace looks responsive.

