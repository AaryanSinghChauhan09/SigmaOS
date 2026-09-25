# Bolt ⚡ Performance Journal

## 2025-05-20 - Kernel Hardening Memory Alignment & Library Build Optimization
**Learning:** Referencing packed struct fields directly in assertions creates unaligned reference safety hazards under x86_64 strict alignment checks. Assigning packed field values to local variables before assertion prevents undefined behavior without runtime overhead.
**Action:** Always copy/bind packed struct fields to local stack variables before taking references or passing to assertions.
