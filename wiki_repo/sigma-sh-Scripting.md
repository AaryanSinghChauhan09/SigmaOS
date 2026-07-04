# sigma-sh Scripting Reference

## Overview

`sigma-sh` is the SigmaOS Sovereign Shell — a full-featured interactive shell and scripting engine. Scripts use the `.sigma` extension. The scripting engine is implemented in `sigma-sh/src/scripting.rs` (Rust, std).

## Running Scripts

```bash
sigma-sh script.sigma              # execute a script
sigma-sh script.sigma arg1 arg2   # with positional arguments
chmod +x script.sigma && ./script.sigma   # executable scripts
```

Script shebang line:
```sh
#!/usr/bin/env sigma-sh
```

## Variables

### Assignment and expansion

```sh
name="sigma"
version=15
echo "Hello, $name v${version}"
echo "Greeting: ${name:-world}"    # default if unset or empty
```

### Special variables

| Variable | Description |
|----------|-------------|
| `$?` | Exit code of last command |
| `$$` | PID of the shell process |
| `$!` | PID of last background process |
| `$#` | Number of positional arguments |
| `$@` / `$*` | All positional arguments |
| `$0` | Script name |
| `$1` – `$9` | Positional arguments |
| `$RANDOM` | Pseudo-random integer 0–32767 |
| `$LINENO` | Current script line number |
| `$SECONDS` | Seconds since epoch |
| `$PWD` | Current working directory |

### Parameter expansion

```sh
${VAR}              # safe expansion
${VAR:-default}     # use default if VAR is unset or empty
${VAR:=default}     # assign and use default if VAR is unset
${#VAR}             # length of VAR
${VAR##pattern}     # strip longest prefix matching pattern
${VAR#pattern}      # strip shortest prefix
${VAR%%pattern}     # strip longest suffix
${VAR%pattern}      # strip shortest suffix
```

## Arithmetic

Use `$((expr))` for integer arithmetic:

```sh
x=10
y=$((x * 3 + 5))       # 35
echo $((y / 7))         # 5
echo $((2 ** 8))        # 256 (exponentiation)
echo $((RANDOM % 100))  # random 0-99
i=$((i + 1))            # increment
```

Supported operators: `+`, `-`, `*`, `/`, `%`, `(`, `)`

## Command Substitution

Use `$(...)` to capture command output:

```sh
kernel=$(uname -r)
echo "Kernel: $kernel"

files=$(ls /etc | wc -l)
echo "Files in /etc: $files"

today=$(date +%Y-%m-%d)
log_file="/var/log/sigma-$today.log"
```

## Control Flow

### if / else / fi

```sh
if [ -f /etc/sigma.toml ]; then
    echo "Config found"
elif [ -d /etc/sigma ]; then
    echo "Config directory found"
else
    echo "No config"
fi

# String comparison
if [ "$arch" = "x86_64" ]; then
    echo "Intel/AMD system"
fi

# Numeric comparison
if [ $count -gt 10 ]; then
    echo "Count exceeded threshold"
fi
```

### for loop

```sh
for arch in x86_64 aarch64 riscv64gc; do
    sigma build --target $arch --release
done

# Iterate over files
for f in /etc/sigma/*.toml; do
    echo "Processing $f"
done
```

### while loop

```sh
retries=0
while [ $retries -lt 3 ]; do
    sigma-net ping 8.8.8.8 -c 1 && break
    retries=$((retries + 1))
done
```

### Functions

```sh
greet() {
    echo "Hello, $1!"
}
greet sigma

# Function with return value (via exit code)
is_up() {
    sigma-net ping "$1" -c 1 > /dev/null 2>&1
}
if is_up 8.8.8.8; then
    echo "Network is up"
fi
```

## Pipelines and Redirections

```sh
# Pipes
sigma-log tail --lines 100 | grep ERROR | wc -l

# Redirections
sigma-monitor cpu > /tmp/cpu.txt        # stdout to file
sigma-monitor cpu >> /tmp/cpu.txt       # append stdout
sigma-diagnostics 2>/dev/null           # discard stderr
sigma-diagnostics > out.txt 2>&1        # both to file

# Background
sigma-monitor watch &
echo "Monitor PID: $!"
```

## Logic Operators

```sh
sigma build && sigma run          # run only if build succeeded
sigma build || echo "Build failed"  # run only if build failed
sigma test ; echo "Done"           # always run echo (regardless of exit code)
```

## Built-in Commands

| Command | Description |
|---------|-------------|
| `cd [dir]` | Change directory (`-` = previous, `~` = home) |
| `pwd` | Print working directory |
| `echo [-n] [...]` | Print text (`\n`, `\t` escapes supported) |
| `export [K=V]` | Set/export environment variable |
| `unset K` | Remove variable |
| `env` | List environment variables |
| `read VAR` | Read line from stdin into variable |
| `alias [K=V]` | Define or list aliases |
| `unalias [-a] K` | Remove alias |
| `type NAME` | Show how name is interpreted |
| `which NAME` | Locate executable on PATH |
| `history` | Show command history |
| `source FILE` / `.` | Execute script in current context |
| `kill [-SIG] PID` | Send signal to process |
| `test EXPR` / `[ EXPR ]` | Evaluate condition |
| `exit [code]` | Exit shell |
| `help` | Show built-in commands |

## Test Expressions

| Expression | True when |
|------------|-----------|
| `-e file` | File/dir exists |
| `-f file` | Regular file exists |
| `-d path` | Directory exists |
| `-z str` | String is empty |
| `-n str` | String is non-empty |
| `a = b` | String equality |
| `a != b` | String inequality |
| `n -eq m` | Integer equal |
| `n -ne m` | Integer not equal |
| `n -lt m` | Integer less than |
| `n -gt m` | Integer greater than |
| `n -le m` | Integer less or equal |
| `n -ge m` | Integer greater or equal |
| `! expr` | Negate expression |

## Example Script

```sh
#!/usr/bin/env sigma-sh
# build-and-test.sigma — build all targets, run tests, report

TARGETS="x86_64 aarch64"
PASS=0
FAIL=0

for target in $TARGETS; do
    echo "Building for $target..."
    if sigma build --target $target --release; then
        PASS=$((PASS + 1))
    else
        FAIL=$((FAIL + 1))
        echo "Build FAILED for $target"
    fi
done

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ $FAIL -gt 0 ]; then
    exit 1
fi
```

## Scripting Tips

- Use `set -e` idiom equivalent: `cmd || exit 1` to fail fast
- Quote variables: `"$var"` prevents word splitting
- Use `$()` instead of backticks `` `cmd` `` — cleaner nesting
- Check exit codes: `$?` immediately after the command
- Use `sigma-fix scan` before deploying scripts to catch common issues

## See Also

- [CLI Reference](CLI-Reference) — all sigma CLI commands
- [sigma-sh Manual](sigma-sh) — interactive shell usage
- [Coreutils](Coreutils) — standard utilities available in scripts
