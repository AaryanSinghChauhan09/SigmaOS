
# S-INIT(1) | SigmaOS Sovereign Init



## NAME

s-init - Profile-aware system bootstrap and shard orchestrator.


## SYNOPSIS

`s-init --profile <name> [--verbose]`


## DESCRIPTION

**s-init** is the primary bootstrap shard for the SigmaOS Sovereign Lattice. It is responsible for detecting the execution environment (via UAL) and activating the shards defined in the selected profile manifest.


## PROFILES

*   **ubuntu**: Rich desktop experience (UI, Lua, Loader).
*   **alpine**: Minimalist IoT/Embedded footprint.
*   **arch**: DIY/Developer mode with high-entropy hooks.
*   **server**: High-availability persistence and networking.


## FILES

*   `/meta/profiles/*.json`: Profile definitions.
*   `/meta/lattice.state`: Runtime shard activation registry.


## SEE ALSO

`s-cli(1)`, `sigma_libc(7)`
