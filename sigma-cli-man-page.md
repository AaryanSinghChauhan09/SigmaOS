# sigma-cli Manual Page

## NAME
`sigma-cli` - The sovereign command-line interface for SigmaOS workspace management, package handling, and lattice shard diagnostics.

## SYNOPSIS
`sigma-cli [COMMAND] [OPTIONS]`

## DESCRIPTION
`sigma-cli` provides direct access to the SigmaOS microkernel features, bypassing legacy shell utilities. It allows users to manage packages natively via `sigma-pkg`, query hardware shards, and monitor security contexts in real time.

## COMMANDS

### `pkg` (Package Management)
*`sigma-cli pkg update` : Synchronizes the local delta indices with the package nexus.* `sigma-cli pkg install [SHARD_NAME]` : Installs a cryptographically attested shard.
* `sigma-cli pkg rollback` : Reverts the system to the last verified clean state.

### `shard` (Kernel Module Management)
*`sigma-cli shard list` : Lists all currently active microkernel shards.* `sigma-cli shard load [PATH]` : Loads a compiled `.shard` dynamically into the kernel lattice.

### `diag` (Diagnostics & Telemetry)
*`sigma-cli diag health` : Outputs real-time telemetry from the `SovereignHealthMonitor`.* `sigma-cli diag pci` : Probes the silicon bus for hardware devices via `SovereignHAL`.

## EXAMPLES

```bash
# Update the package lattice and install the VR compositor
sigma-cli pkg update
sigma-cli pkg install vr-studio

# View active system shards
sigma-cli shard list

```
