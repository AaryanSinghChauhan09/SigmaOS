# SPDX-License-Identifier: GPL-2.0-or-later
# ==========================================================================
# SigmaOS: Sovereign Core Utilities (Nim)
# Replaces: ecosystem/SovereignCoreUtils.cpp, SovereignCoreUtils.h, etc.
# ==========================================================================

proc sovereignHash*(data: string): string =
  # Simple hand-rolled Adler32/Fowler-Noll-Vo hash stub to remain zero-dependency
  var hash = 2166136261u32
  for c in data:
    hash = hash xor uint32(ord(c))
    hash = hash * 16777619u32
  return $hash

proc compareSovereign*(a, b: string): bool =
  return a == b
