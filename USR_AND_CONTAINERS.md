# USR AND CONTAINERS



SigmaOS replaces `systemctl`, `dbus`, and `apt` with the **Sovereign Unified Shard Registry (USR)**. It provides amnesic-protected, quantum-safe service discovery and inter-process communication between all 600 kernel shards.



usr_register_shard("SovereignNetStack", 0x00A1);
usr_register_shard("SovereignVFS", 0x00A2);




sigma_u32 id = usr_discover_shard("SovereignVFS");


---


SovereignOS containers are automatically provisioned with sovereign veth interfaces routed through the `SovereignNetStack`.



container_net_attach("app-container-01", "AA:BB:CC:DD:EE:FF");
container_storage_mount("app-container-01", "/data");


---


State snapshots survive hardware resets by being sharded across the distributed VFS lattice:


persistence_snapshot("SovereignSEL");
persistence_restore("SovereignSEL");

