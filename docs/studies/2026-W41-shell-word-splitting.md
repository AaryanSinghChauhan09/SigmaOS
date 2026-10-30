# 2026-W41: Shell word splitting study

This study records the reference behavior for quoting, word splitting, and
pathname expansion. SigmaOS preserves quoted words, applies expansion in the
documented order, and treats malformed input as a non-executing parse error.

Implementation changes require focused examples for unquoted, single-quoted,
and double-quoted input plus explicit rollback evidence.
