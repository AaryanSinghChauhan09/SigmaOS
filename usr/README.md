# SigmaOS Userland

Provides standard user-space shell and core utilities for interacting with the SigmaOS microkernel.

## Sub-Modules

### 1. Sovereign Shell (`sh.c`)
Standard interactive command interpreter:
- Executes built-in command handlers.
- Interfaces with file APIs to view file structures and contents.
- Connects to the network stack via socket primitives to check host connectivity.

### 2. Core Utilities
Standard POSIX-inspired utilities:
- `ls`: Lists file and folder nodes.
- `cat`: Output file data to console standard output.
- `echo`: Formats and writes line string values to stdout.
- `pwd`: Outputs active directory.
- `clear`: Flushes console view and terminal cursor coordinates.
- `ping`: Verifies remote node round-trip time.
- `resolve`: Performs DNS host translation to IPv4 addresses.
