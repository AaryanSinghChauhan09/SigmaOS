# sigma-debug Manual

## NAME

`sigma-debug` — SigmaOS kernel shard debugger

## SYNOPSIS

```
sigma-debug <command> [options]
sigma-debug --version
sigma-debug --help
```

## DESCRIPTION

`sigma-debug` is a GDB-style CLI for inspecting live SigmaOS kernel shards, reading memory, dumping registers, resolving symbols, and managing breakpoints. On bare metal it connects to `/run/sigma/debugd.sock`. On non-SigmaOS platforms it operates in simulation mode.

## COMMANDS

### `shard <list|info|load|unload>`

Manage kernel lattice shards:

```bash
sigma-debug shard list               # list all loaded shards

sigma-debug shard info sigma-core    # show base address, size, sections

sigma-debug shard load mymod.shard   # load and verify a shard

sigma-debug shard unload sigma-net   # unload a shard

```

### `mem <read|dump|map> [--addr <hex>] [--len <n>]`

Inspect kernel memory:

```bash
sigma-debug mem dump --addr 0xffff000000001000 --len 64
sigma-debug mem map          # show full kernel memory map (r-x/rw-/r--)

sigma-debug mem read --addr 0xffff800000010000 --len 16 --json
```

### `reg [--pid <n>]`

Dump CPU registers for a process:

```bash
sigma-debug reg              # kernel thread (pid 1)

sigma-debug reg --pid 512    # specific process

sigma-debug reg --json
```

### `sym <resolve|search> [addr|name]`

Symbol resolution:

```bash
sigma-debug sym resolve 0xffff000000001234   # addr → function+offset

sigma-debug sym search sigma_syscall         # search by name fragment

```

### `bp <set|list|del|clear> [--addr <hex>]`

Breakpoint management:

```bash
sigma-debug bp set --addr 0xffff000000001234
sigma-debug bp list
sigma-debug bp del
sigma-debug bp clear
```

### `bt [--pid <n>]`

Stack backtrace:

```bash
sigma-debug bt
sigma-debug bt --pid 512 --json
```

### `attach --pid <n>`

Attach to a running process, pause all threads, and load its symbol table.

### `script <file>`

Execute a debug script (sequence of `sigma-debug` commands).

### `repl`

Interactive debug REPL — simulates a session connected to `sigma-debugd`.

## OPTIONS

| Flag | Description |
|------|-------------|
| `--pid <n>` | Target process ID |
| `--addr <hex>` | Memory address (e.g. `0xffff000000001000`) |
| `--len <n>` | Bytes to read/dump (default: 64) |
| `--json` | Machine-readable JSON output |
| `--version`, `-V` | Print version |
| `--help`, `-h` | Show help |

## EXAMPLES

```bash

# Show all loaded shards

sigma-debug shard list

# Hex dump 128 bytes at a kernel address

sigma-debug mem dump --addr 0xffff000000001000 --len 128

# Look up a symbol by address

sigma-debug sym resolve 0xffff000000001234

# Set a breakpoint and backtrace

sigma-debug bp set --addr 0xffff000000001234
sigma-debug bt --pid 1

# Start interactive session

sigma-debug repl
```

## VERSION

sigma-debug 1.0.0

## SEE ALSO

`sigma shard(1)`, `sigma-trace(1)`, `sigma-log(1)`
