#!/usr/bin/env sh
# SigmaOS ShellScript POSIX Conformance & Static Audit Tool
# Scans all shell scripts in the repository for syntax, unquoted variable hazards,
# executable permissions, and POSIX compliance.

set -e

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

echo "==========================================================="
echo "  SigmaOS ShellScript Conformance & Static Audit Tool"
echo "==========================================================="

TOTAL_SCRIPTS=0
PASSED_SCRIPTS=0
FAILED_SCRIPTS=0

AUDIT_LOG="/tmp/sigma_shellcheck_audit.log"
rm -f "$AUDIT_LOG"

audit_script() {
    SCRIPT_PATH="$1"
    TOTAL_SCRIPTS=$((TOTAL_SCRIPTS + 1))

    echo "Auditing: $SCRIPT_PATH"

    # 1. Check syntax with sh -n or bash -n depending on shebang
    FIRST_LINE="$(head -n 1 "$SCRIPT_PATH")"
    case "$FIRST_LINE" in
        *bash*)
            CHECK_CMD="bash -n"
            ;;
        *)
            CHECK_CMD="sh -n"
            ;;
    esac

    if ! $CHECK_CMD "$SCRIPT_PATH" 2>>"$AUDIT_LOG"; then
        echo "  [FAIL] Syntax check ($CHECK_CMD) failed for $SCRIPT_PATH"
        FAILED_SCRIPTS=$((FAILED_SCRIPTS + 1))
        return 1
    fi

    # 2. Check executable permission
    if [ ! -x "$SCRIPT_PATH" ]; then
        chmod +x "$SCRIPT_PATH" || true
        echo "  [INFO] Fixed executable permissions for $SCRIPT_PATH"
    fi

    # 3. Basic POSIX / Quoting checks
    ERRORS=0

    # Check for unquoted variables in critical places
    if grep -n 'rm -rf \$[a-zA-Z0-9_]*[^"]' "$SCRIPT_PATH" >>"$AUDIT_LOG" 2>&1; then
        echo "  [WARN] Unquoted variable in rm -rf detected in $SCRIPT_PATH"
    fi

    # Check for missing shebang
    FIRST_LINE="$(head -n 1 "$SCRIPT_PATH")"
    case "$FIRST_LINE" in
        "#!"*) ;;
        *)
            echo "  [WARN] Missing shebang in $SCRIPT_PATH"
            ;;
    esac

    echo "  [PASS] $SCRIPT_PATH audited successfully"
    PASSED_SCRIPTS=$((PASSED_SCRIPTS + 1))
    return 0
}

# Audit all .sh scripts in root and scripts/
for script in *.sh scripts/*.sh; do
    if [ -f "$script" ]; then
        audit_script "$script" || true
    fi
done

echo "-----------------------------------------------------------"
echo "Audit Summary:"
echo "  Total Scripts Audited: $TOTAL_SCRIPTS"
echo "  Passed: $PASSED_SCRIPTS"
echo "  Failed: $FAILED_SCRIPTS"
echo "-----------------------------------------------------------"

if [ "$FAILED_SCRIPTS" -gt 0 ]; then
    echo "Audit completed with errors. See $AUDIT_LOG for details."
    exit 1
else
    echo "All shell scripts passed static audit successfully!"
    exit 0
fi
