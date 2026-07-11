#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# check-commit-msg.sh — enforce Linux kernel-style commit message format
#
# Valid examples:
#   kernel/security: fix buffer overflow in sigma_zt_attest
#   pkg: add staged rollout support with karma gating
#   init: replace finite loop with infinite signalfd event loop
#   ci: uncomment kernel test execution steps
#
# Rules:
#   - Must match:  subsystem[/component]: description
#   - Subsystem:   lowercase letters, digits, underscores, hyphens, slashes
#   - Description: 10–72 characters
#   - No trailing period on the summary line

set -euo pipefail

MSG_FILE="$1"
MSG=$(head -1 "$MSG_FILE")

# Skip merge commits and fixup commits
if echo "$MSG" | grep -qE '^(Merge|Revert|fixup!|squash!)'; then
    exit 0
fi

PATTERN='^[a-z][a-z0-9/_-]+: .{10,72}$'

if ! echo "$MSG" | grep -qE "$PATTERN"; then
    echo "──────────────────────────────────────────────"
    echo "ERROR: Commit message does not match required format."
    echo ""
    echo "  Required: <subsystem>[/<component>]: <description>"
    echo "  Description must be 10–72 characters."
    echo ""
    echo "  Your message: $MSG"
    echo ""
    echo "  Valid examples:"
    echo "    kernel/security: fix buffer overflow in sigma_zt_attest"
    echo "    pkg: add staged rollout with karma gating"
    echo "    ci: uncomment kernel test execution steps"
    echo "    init: replace finite loop with signalfd event loop"
    echo "──────────────────────────────────────────────"
    exit 1
fi

# No trailing period on summary line
if echo "$MSG" | grep -qE '\.$'; then
    echo "ERROR: Summary line must not end with a period."
    exit 1
fi

exit 0
