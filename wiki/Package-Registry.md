# SigmaOS Sovereign Package Registry

NixOS-style reproducible builds without Nix — signed `.spkg` recipes curated here.

## Layout

```
sigma_pkg_registry/
  README.md
  recipes/
    example.hello.sigma.recipe
```

## Recipe format

See `recipes/example.hello.sigma.recipe` — declarative name, version, build steps, curation level.

## API

Kernel/userland: `include/security/sigma_pkg_registry.h`
Functions: `SovereignPkg_Register`, `SovereignPkg_Audit`, `SovereignPkg_Rollback`.

## Community workflow

1. Add recipe under `recipes/`.

2. Open PR; CI runs `ci_branch_check.sh`.

3. Wiki documents package via `sigma_automation.sh wiki-sync`.
