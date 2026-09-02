# Declarative Package Management & Automation

SigmaPkg leverages declarative system state management and transactional automation:

*   **Atomic Rollbacks**: Instant generation rollbacks on configuration failures.
*   **Hermetic Build Closures**: Content-addressed dependency tracking ensuring zero unreferenced state.
*   **Automated Service Supervision**: Event-driven service dependency reconciliation with exponential backoff.
*   **Storage Tiering & Scrubbing**: Bcachefs/ZFS automated extent promotion/demotion and data integrity scrubbing.
