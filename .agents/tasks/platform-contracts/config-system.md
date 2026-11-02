# Config System Contract

> **Shared SigmaOS platform contract.** Define the common config schema, migration path, validation, and fallback behavior shared across all SigmaOS components.

---

## Invariants

- Config is validated at load time; invalid config falls back to defaults with a logged warning (never crashes).
- Config migration: each schema version has a migration function; applied automatically on load.
- Config precedence: CLI flags > env vars > user config > system config > defaults.
- Config changes are atomic: write to a temp file then rename (no partial writes).
- Config diff: `sigma-config diff <component>` shows current vs default.

## Schema

TOML files under $XDG_CONFIG_HOME/sigma/<component>.toml (user) or /etc/sigma/<component>.toml (system). Each component defines its schema; the config-system provides validation and migration.

## Failure Modes and Recovery

Config file missing (use defaults), config file corrupt (use defaults + log), schema mismatch (migrate or use defaults), permission denied (log + use defaults).

## Open Questions

Should config support hot-reload (SIGHUP)? Current: yes for services that opt in; documented per-component.
