# SigmaOS CPU Affinity Policy

## Placement & Migration Rules

1. **Affinity Mask**:
   - Represented as a 64-bit integer (`cpu_affinity`), where bit $k$ set allows execution on CPU core $k$.

2. **Load Balancing**:
   - `select_best_cpu` evaluates candidate cores matching the process mask and selects the core with the minimal run queue depth.

3. **Fallback Behavior**:
   - Tasks with mask 0 or invalid mask fall back to CPU 0 (`affinity = 1`).
