1


To ensure the **Sovereignty** of the lattice, SigmaOS implements a chain of trust from the hardware root up to the userland Zenith UI.


1




1. **S-ROM**: Immutable silicon-level public key hash.
2. **S-BOOT**: The bootloader verified against the S-ROM key.



3. **S-KERNEL**: The core lattice shards verified by the bootloader.
4. **S-APP**: Userland shards signed by a trusted identity.


1



1



1



1


Developers can sign their custom shards using the `sigma-sign` tool:


1


sigma-sign --key my_identity.key --shard custom_driver.cpp


1



1


The `SovereignInit` shard enforces the boot policy:


1



1


---


1

