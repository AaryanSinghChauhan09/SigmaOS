# Getting Started with SigmaOS

Welcome to the Sovereign Lattice. This guide will walk you through building, testing, and launching SigmaOS.

## Prerequisites
- **C Compiler**: GCC (MinGW-w64 on Windows, Standard GCC on Linux/macOS)
- **Assembler**: NASM
- **Linker**: LD
- **Node.js**: For the Zenith Dashboard and API Bridge

## Building the Sovereign Lattice

### Windows (PowerShell)
```powershell
./build_sovereign.ps1
```

### Linux / macOS (Bash)
```bash
chmod +x ./build_sovereign.sh
./build_sovereign.sh
```

## Running Atomic Tests
To certify your lattice shards:
```bash
./run_sigma_tests.sh  # Linux/macOS
./run_sigma_tests.ps1 # Windows
```

## Launching the Zenith Dashboard
The Zenith Dashboard provides real-time observability into the lattice state.
1. Navigate to `web_ui/`
2. Open `index.html` in a modern browser.
3. Observe the live Vitals HUD and Shard Grid.

## External Tooling (API Bridge)
To allow external tools to interact with SigmaOS:
```bash
cd gui/backend
npm install express express-graphql graphql
node server.js
```
Access the GraphQL playground at `http://localhost:3000/graphql`.
