## 2026-11-04 - Zero-allocation ASCII case-insensitive search in MacOsSpotlightEngine
**Learning:** Querying in-memory index structures like `MacOsSpotlightEngine` previously called `.to_lowercase()` on both query strings and candidate titles inside iter filters, generating heap allocations proportional to the index size for every lookup.
**Action:** Use zero-allocation ASCII window matching via `haystack.as_bytes().windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle.as_bytes()))` with non-ASCII fallback, eliminating intermediate heap allocations.
