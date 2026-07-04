# WIKI-Contributing

1

> Join the evolution of digital sovereignty.

---

1

SigmaOS is an industrial-grade project. All contributions must adhere to:

1

1

1

We follow a staged release cadence:

1

1

1

Every Pull Request triggers the following automated suite:

1. **Lattice Rebuild**: All 600+ shards must compile with zero warnings.

2. **Regression Suite**: IRQ handlers and SHS v2 are verified for RDTSC-cycle precision.

3. **Security Scan**: Verify PQC signatures and TPM handshake protocols.

4. **Doc Lint**: Ensure all WIKI files follow the GitHub Flavored Markdown standard.

1

1. **Fork** the repository and create an `alpha` branch.

2. **Develop** your shard in the appropriate `suites/` directory.

3. **Sync** documentation in `WIKI/`.

4. **Submit** a PR to `beta` for review.

---
"Sovereignty is a collective intent."
