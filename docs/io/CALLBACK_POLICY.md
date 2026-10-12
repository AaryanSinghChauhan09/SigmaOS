# Completion Callback Policy

## Overview
The Callback Manager manages completion callbacks for scheduled I/O requests.

## Policy Rules
- Callbacks are invoked exactly once upon request completion or failure.
- Callbacks pass request ID and typed outcome status (`Success`, `Timeout`, `HardwareError`, `Canceled`).
- Handlers are unmapped immediately upon invocation to prevent callback leaks or double execution.
