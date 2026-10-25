# SigmaOS Shell Maintenance & Documentation

## Overview
This document outlines shell architecture, expansion rules, diagnostic formatting, and maintenance guidelines for SigmaOS command interpretation.

## Execution and Safety Guidelines
- Strict POSIX subset expansion order
- Error handling and exit codes (0 = Success, 2 = Misuse, 126 = Cannot execute, 127 = Command not found)
- Diagnostic formatting and interactive shell prompt behavior
