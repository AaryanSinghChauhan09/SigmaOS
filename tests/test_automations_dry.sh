#!/bin/bash
# =============================================================================
# Σ SIGMAOS: SOVEREIGN AUTOMATION DRY-RUN TEST SUITE
# =============================================================================
# Purpose: Ensures all scripting automations (.sh, .ps1, .py) function 
#          according to the 'Sovereign Logic' protocol.
# =============================================================================

echo "Σ [TEST]: Starting Sovereign Automation Dry-Run..."

# 1. Test Wiki Builder Syntax
if [ -f scripts/wiki_builder.py ]; then
    echo "[PASS]: scripts/wiki_builder.py detected."
    python3 -m py_compile scripts/wiki_builder.py && echo "  [VERIFIED]: Syntax OK."
else
    echo "[FAIL]: scripts/wiki_builder.py missing." && exit 1
fi

# 2. Test Distro Forge Syntax
if [ -f scripts/launch_distro.py ]; then
    python3 -m py_compile scripts/launch_distro.py && echo "[PASS]: scripts/launch_distro.py syntax OK."
fi

# 3. Test Shell Script Integrity
SH_SCRIPTS="scripts/sigma_auto.sh scripts/sigma_automator.sh scripts/sigma_industrial_deploy.sh"
for s in $SH_SCRIPTS; do
    if [ -f "$s" ]; then
        bash -n "$s" && echo "[PASS]: $s syntax OK." || { echo "[FAIL]: $s syntax error." && exit 1; }
    fi
done

# 4. Check for forbidden 'sudo' usage in normal automations (Security Phase)
echo "Σ [TEST]: Scanning for unauthorized 'sudo' in user-level scripts..."
if grep -r "sudo " scripts/ --exclude-dir=distro_forge; then
    echo "[WARN]: Potential 'sudo' usage detected in user scripts. Review for security."
else
    echo "[PASS]: No unprivileged escalations found."
fi

echo "Σ [ZENITH]: Automation Dry-Run PASSED."
