# MODULAR REGISTRY SPEC



The Sovereign Registry is the central orchestration nexus for the 33-suite lattice. It manages the lifecycle, attestation, and dispatching of all Sovereign Shards.


All registries are located in: `kernel/suites/S10_Registry/shards/`


Shards must use the `SovereignRegistry_Register` API to bind to the lattice during stage-3 boot:


sigma_err_t SovereignRegistry_Register(const char* name,
                                       shard_category_t cat,
                                       shard_init_fn init);






Each registration triggers a **Global Integration Verification (GIV)** sequence:



---

