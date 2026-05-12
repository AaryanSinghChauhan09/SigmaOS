# DEPENDENCY POLICY

1

// =============================================================================
// SigmaOS — DEPENDENCY PURITY POLICY
// =============================================================================
// This document defines which external dependencies are ALLOWED in SigmaOS.
// Any violation must be replaced before merging to main.
//
// STATUS:  SOVEREIGN (zero foreign runtime deps in kernel + core tools)
// VERSION: 2.0.0
// =============================================================================

ALLOWED LANGUAGES (kernel):

1

1

ALLOWED LANGUAGES (userland tools — in tools/):

1

FORBIDDEN IN KERNEL SUITES (S01–S10):

1

                 is isolated to S02_ZenithUI and must have a native path

1

ALLOWED EXTERNAL DEPS (vendored only, no network fetch):

1

MIGRATION STATUS:
  ? sovereign_audit.c        ? replaces global_integrated_audit.py [COMPLETED]
  ? sovereign_test_runner.c  ? replaces sovereign_test_runner.py [COMPLETED]
  ? sovereign_wiki_builder.c ? replaces wiki_builder.py [COMPLETED]
  ? sigma_libc_alloc.rs      ? replaces glibc malloc [COMPLETED]
  ? sovereign_pkg_manager.c  ? replaces sigpkg.py [COMPLETED]
  ? sovereign_shell.c        ? replaces zenith_shell.py [COMPLETED]
  ? sovereign_depgraph.c     ? replaces generate_dependency_graph.py [COMPLETED]
  ? sigma_libc_extended.c    ? glibc-free stdlib [COMPLETED]
  ? sovereign_ci.yml         ? Automated Purity Gate [COMPLETED]
  ? tools/Makefile           ? Static Linking Orchestrator [COMPLETED]
  ? sigma-build              ? Native Build Orchestrator [COMPLETED]
  ? S02_ZenithUI Isolation   ? Web UI moved to userland/ZenithWeb/ [COMPLETED]
  ? Global Purge             ? 100% of non-essential .py/.js files ELIMINATED

FINAL PURITY SCORE: 100% (Kernel & Core Tools)
RUNTIME DEPENDENCY: ZERO (S01-S10)
ARCHITECTURE: SOVEREIGN SINGULARITY

