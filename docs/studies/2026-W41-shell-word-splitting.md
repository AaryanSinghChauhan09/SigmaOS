# SigmaOS Study: POSIX Word Splitting and Quoting Integrity

## Abstract
This study evaluates the trade-offs of POSIX compliant word splitting vs zero-allocation word splitting algorithms for `sigma-sh` in no_std / kernel-userspace environments.

## Analysis
1. Unquoted parameter expansions undergo word splitting based on internal field separators (`$IFS`).
2. Single-quoted (`'...'`) and double-quoted (`"..."`) literals protect whitespace and wildcards from pathname expansion.
3. Pathname expansion (globbing) occurs strictly after word splitting step in sequence.

## Recommendations for SigmaOS
- Implement zero-allocation window iterators over string slices to perform splitting without extra heap allocations.
- Guarantee exact evaluation order to maintain POSIX compatibility.
