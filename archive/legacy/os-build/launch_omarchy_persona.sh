#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# Compatibility wrapper for SigmaOS Sovereign Desktop Profile
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec bash "$SCRIPT_DIR/launch_desktop_persona.sh" "$@"
