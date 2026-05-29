# Automation & CLI Engine

## Scripts

| Script | Purpose |
|--------|---------|
| `scripts/sigma_automation.sh` | backup, update, recovery-check, wiki-sync |
| `scripts/sigma_git_sync.sh` | commit + push + wiki mirror |
| `scripts/ci_branch_check.sh` | branch feature parity |
| `scripts/sigma_cli_host.sh` | host `sigma-cli update` wrapper |

## sigma-cli (in-guest)

```
sigma-cli profile list|use <name>
sigma-cli alias list|add <name> <cmd>
sigma-cli update
sigma-cli branch-check
sigma-cli automation <command>
```

## Typical maintainer flow

```bash
./scripts/sigma_automation.sh update
./scripts/ci_branch_check.sh
./scripts/sigma_git_sync.sh -m "docs: phase 7-8 wiki sync"
```

Wiki auto-publishes on push to `main` when `wiki_repo/**` changes.
