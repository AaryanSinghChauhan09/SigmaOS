# Security: FIPS-140 Lattice Integration

SigmaOS achieves enterprise compliance trust parity with **AlmaLinux** and **CentOS** without relying on their massive upstream catalog of dependencies. This is achieved via the Sovereign /security/ module.

## Modularisation & OOP Principles

We explicitly split the monolithic security logic into strict OOP namespaces (/security/crypto, /security/audit, /security/logging) to enforce zero-dependency encapsulation:

*   **Encapsulation**: Crypto APIs are strictly isolated. No standard library <openssl/crypto.h> is permitted; hardware RNG is accessed directly through BaseCryptoLattice.
*   **Abstraction**: Compliance hooks are exposed via the IComplianceHook interface, enabling dynamic validation checks without exposing underlying lattice mechanics.
*   **Inheritance**: Distinct lattice modules (e.g., FIPS140Lattice) extend the BaseCryptoLattice to inherit raw cryptographic capabilities.
*   **Polymorphism**: The system supports multiple compliance modes, allowing it to seamlessly switch between strict FIPS mode and standard Sovereign Mode dynamically.

By deploying this integration natively, SigmaOS neutralizes the necessity of enterprise distros for compliance-heavy environments, guaranteeing audited, secure operations strictly through the lattice.
