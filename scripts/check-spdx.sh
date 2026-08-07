#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-2.0-or-later
# check-spdx.sh — verify every new/modified C/C++ file has an SPDX header
# Called by pre-commit with changed file paths as arguments.
set -euo pipefail

FAIL=0
for f in "$@"; do
    [[ -f "$f" ]] || continue
    if ! head -3 "$f" | grep -q "SPDX-License-Identifier"; then
        echo "ERROR: missing SPDX-License-Identifier in $f"
        echo "  Add as first line: // SPDX-License-Identifier: GPL-2.0-or-later"
        FAIL=1
    fi
done
exit $FAIL
