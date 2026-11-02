# SigmaOS-dots-configuration

Dotfiles configuration management system providing version-controlled user dotfiles, profile templates, and automated deployment scripts across SigmaOS installations.

**Upstream inspiration:** https://github.com/linuxmint/cinnamon

**Dependencies:** `git`, `rsync`

**Build/Install:**
```
sudo ./src/main.sh install
```

## Upstream Reference

| Upstream | Concept absorbed |
|----------|-----------------|
| [omadots (Omarchy)](https://github.com/basecamp/omarchy) | Declarative dotfile manifest (source->target map); install/apply/backup/diff subcommands |
| [chezmoi](https://www.chezmoi.io/) | Idempotent re-apply; diff subcommand showing what would change |
