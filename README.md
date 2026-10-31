# SigmaOS

SigmaOS is a sovereign, AI-native operating system project. This repository is
the central Rust codebase and coordination point for the kernel, system
services, desktop environment, and the component repositories listed in
`COMPONENTS.toml`.

## Development

Use the Rust toolchain declared by `Cargo.toml`. Run `make verify` for the
repository verification gates and `make test` for the native test suites. The
managed component inventory and local checkout status can be inspected with
`make registry-audit`.

For a live GitHub URL comparison, run
`python3 scripts/audit_component_fleet.py --github` with `gh` authenticated.

Prepared public repositories can be reviewed with
`python3 scripts/publish_registry_repositories.py`. It plans from the registry
and existing clean local checkouts; it does not generate placeholder repos.
Pass `--apply` with an exact `--name COMPONENT` or a bounded `--limit N` to
create and push selected `main` branches. Ambiguous registry aliases, dirty
checkouts, and mismatched origins are skipped. The legacy
`scripts/create_repository.sh` command delegates to the same dry-run-first tool.

## Component boundaries

Focused components live in separate repositories and integrate through
documented contracts. Their implementation and release status must be supported
by tests and integration evidence; registry presence alone does not establish
readiness.

## Desktop defaults

[`scripts/sigma_omarchy_bootstrap.sh`](scripts/sigma_omarchy_bootstrap.sh)
shows a dry-run plan by default. Pass `--apply` to stage the bundled Hyprland,
Waybar, and Ghostty defaults under `XDG_CONFIG_HOME` (or `~/.config`). It only
copies files whose destinations are absent and preserves existing configs. The
script does not install desktop packages or fonts.

## Upstream study and inspiration

SigmaOS studies projects including Omarchy and Linux Mint for concrete desktop
and repository practices. Adopted behavior should record its reference,
rationale, compatibility impact, and rollback path.
