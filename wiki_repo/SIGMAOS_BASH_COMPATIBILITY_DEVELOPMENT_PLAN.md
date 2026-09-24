# SigmaOS GNU Bash Compatibility Engine - Master Development Plan

## 1. Executive Summary & Vision

GNU Bash (Bourne Again SHell) is the ubiquitous shell across Linux distributions (RHEL, Debian, Ubuntu, Arch, Fedora, Alpine) and BSD compatibility layers. In **SigmaOS**, providing 100% native GNU Bash 5.2 script and interactive compatibility (`sigma-bash` / `/bin/bash`) ensures seamless execution of administrative scripts, build tools (`autotools`, `make`, `configure`), package installers, and developer workflows without requiring external binary ports.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Compatibility Mechanism Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **GNU Bash 5.2 (RHEL / Debian / Arch)** | Extended conditional syntax (`[[ ... ]]`), process substitution (`<(cmd)`, `>(cmd)`), associative arrays (`declare -A`), parameter pattern substitution (`${var//search/replace}`). | `src/shell/zsh_bash_parity.rs` |
| **FreeBSD `/bin/sh` & OpenBSD `ksh`** | POSIX compliance mode (`set -o posix` / `bash --posix`), rapid execution for system startup scripts, strict error handling (`set -e`, `set -u`, `set -o pipefail`). | `src/shell/sovereign_shell_parity.rs` |
| **Linux `bash-completion` Package** | Dynamic programmable completion (`complete -F _func_name`), `/etc/bash_completion.d/` script loading, path and option autocompletion. | `src/shell/intelligent_terminal.rs` |
| **GNU Readline (`libreadline`)** | Interactive line editing, Emacs (`Ctrl+A`, `Ctrl+E`, `Ctrl+K`) and Vi modal editing, reverse history search (`Ctrl+R`), inputrc configuration (`~/.inputrc`). | `src/shell/terminal_emulator.rs` |
| **Debian / Ubuntu `dash` & `dpkg` Scripting** | Lightweight fast-path parser execution for `/bin/sh` symlink targets executing Debian maintainer scripts (`preinst`, `postinst`). | `src/shell/busybox_applet.rs` |

---

## 3. 5-Layer GNU Bash Compatibility Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Programmable Completion Engine (`complete -F`, `bash-completion`)│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Interactive Line Editor & History Search (Readline / Vi/Emacs) │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Builtin Command Engine (`read`, `mapfile`, `shopt`, `trap`)   │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: Bash 5.2 Extended Syntax (`[[ ]]`, `<()`, `${//}`, `declare -A`)│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: POSIX / Sh Core AST Parser & Signal Trap Engine               │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: POSIX / Sh Core AST Parser & Signal Trap Engine
- **Parser Core:** Standard shell tokenization, pipelines (`\|`), redirections (`2>&1`, `&>`), heredocs (`<<EOF`), and compound commands.
- **Trap Management:** Asynchronous signal traps (`trap 'handler' SIGINT EXIT ERR`).

### Layer 2: Bash 5.2 Extended Syntax & Language Features
- **Extended Test Operator:** `[[ $a =~ ^[0-9]+$ ]]` regex matching, logical `&&`/`||` inside brackets.
- **Process Substitution:** Named pipe FIFOs and `/proc/self/fd/` descriptors created for `<(command)` and `>(command)`.
- **Arrays:** Indexed arrays (`arr=(a b c)`) and associative key-value maps (`declare -A map`).
- **Advanced Parameter Transformations:** `${var^}`, `${var,}`, `${var@Q}`, `${var//pattern/replacement}`.

### Layer 3: Builtin Command Engine
- **I/O Builtins:** `read -r -a array`, `mapfile` / `readarray -t`.
- **Options & Controls:** `shopt -s globstar nullglob extglob`, `set -e -u -o pipefail`.
- **Job Control Builtins:** `jobs`, `fg`, `bg`, `wait`, `disown`.

### Layer 4: Interactive Line Editor & History Search
- **Keybindings:** Full Emacs and Vi modal keymap bindings.
- **Incremental Search:** Case-insensitive reverse search (`Ctrl+R`) over persistent `~/.bash_history`.

### Layer 5: Programmable Completion Engine
- **`complete` Builtin:** `complete -F _service_complete service`, `complete -W "opt1 opt2" cmd`.
- **Directory Sourcing:** Auto-sourcing completion specs from `/etc/bash_completion.d/` and `/usr/share/bash-completion/completions/`.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | POSIX Parser Core | Implement POSIX `sh` parser, pipeline executor, and `trap` signal handling in `src/shell/`. | Implemented |
| **Milestone 2** | Bash Extended Syntax | Add `[[ ... ]]`, process substitution `<()`, and array types (`declare -a/-A`). | Implemented |
| **Milestone 3** | Builtin Parity | Implement `mapfile`, `read`, `shopt`, `export`, `alias`, and `set -o pipefail`. | Implemented |
| **Milestone 4** | Readline Editor | Integrate Emacs/Vi line editing, `Ctrl+R` history search, and `~/.inputrc` configuration. | Implemented |
| **Milestone 5** | Completion Engine | Implement `complete -F` programmable completion and `bash-completion` package loading. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test runners in `src/shell/zsh_bash_parity.rs` and `src/shell/sovereign_shell_parity.rs`.
2. **GNU Bash Test Suite:** Running standard `bash --posix` compliance scripts and maintainer script tests.
3. **Automated Verification:** Continuous testing via `./run_sigma_tests.sh`.
