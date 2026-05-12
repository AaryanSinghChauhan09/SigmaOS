# MODULAR REGISTRY SPEC

1

1

The Sovereign Registry is the central orchestration nexus for the 33-suite lattice. It manages the lifecycle, attestation, and dispatching of all Sovereign Shards.

1

All registries are located in: `kernel/suites/S10_Registry/shards/`

1

Shards must use the `SovereignRegistry_Register` API to bind to the lattice during stage-3 boot:

1

sigma_err_t SovereignRegistry_Register(const char* name,
                                       shard_category_t cat,
                                       shard_init_fn init);

1

1

1

1

1

Each registration triggers a **Global Integration Verification (GIV)** sequence:

1

1

---

1

