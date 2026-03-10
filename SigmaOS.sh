#!/bin/bash
# SigmaOS Sovereign Launcher for Linux/macOS
echo "[SIGMA] Initializing Sovereign Suite..."
PYTHON_CMD="python3"
if ! command -v $PYTHON_CMD &> /dev/null
then
    PYTHON_CMD="python"
fi

if ! command -v $PYTHON_CMD &> /dev/null
then
    echo "[CRITICAL] Python not found. Please install Python 3."
    exit 1
fi

echo "[SIGMA] Hydrating environment..."
$PYTHON_CMD launcher.py
