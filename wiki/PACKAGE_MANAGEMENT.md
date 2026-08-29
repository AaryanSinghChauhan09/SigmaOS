# Package Management (sigpkg)

`sigpkg` is the native package manager for SigmaOS, providing secure dependency resolution and isolated container execution.

## Package Architecture
- `.sigpkg` packages contain raw binary images, metadata manifests, and sandboxing rules.
- **Verifier**: Cryptographically signs and verifies packages before execution.
- **Resolver**: Solves topological dependency chains recursively.\n