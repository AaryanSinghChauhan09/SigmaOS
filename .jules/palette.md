# 🎨 Palette's UX and Accessibility Journal

This journal documents user experience, usability, and accessibility micro-interactions and refinements inside the Zenith desktop and command-line shell environment.

---

## 2026-03-02 - Un-Enabled Default Profiles
**Learning:** Initializing default accessibility profiles (e.g., Vision, Hearing, Mobility) with settings toggled to `false` by default makes active-profile checks fail upon activation.
**Action:** Call `enable_all()` on the newly instantiated default profiles inside `add_default_profiles()` to ensure those features are pre-enabled and responsive when selected.

## 2026-03-01 - Temporary String Lifetime Drops
**Learning:** Borrowing a temporary `String::new()` inside an `unwrap_or` pattern creates a dangling reference that is dropped at the end of the statement.
**Action:** Use `.map(|s| s.as_str()).unwrap_or("")` to map the option safely to a longer-lived `&str` slice, eliminating allocations and preventing compilation failures.
