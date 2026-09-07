# AI Agent Data Operation Management Guidelines

## Purpose
These guidelines define operational rules, implementation patterns, and safety guardrails for AI coding agents reading, writing, encrypting, or committing filesystem data in SigmaOS.

---

## Directives for AI Agents

1. **PQC Signature Verification**:
   - Always verify PQC digital signatures (`pqc_verify_signature`) before trusting data payload integrity from external volumes.

2. **Soft Updates Dependency Registration**:
   - Register data block allocation dependencies (`MetadataOp::BlockAlloc`) before directory entry additions to guarantee filesystem crash consistency.

3. **Code Pattern: CAS Data Block Store**:
```rust
let mut fs = ContentAddressedFs::new();
let data_hash = fs.write_file(b"Sovereign data block payload")?;
assert!(fs.get_node(&data_hash).is_some());
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm filesystem data operation and PQC encryption unit tests pass.

---

## Related Files
- `src/filesystem/sigmafs.rs`
- `src/filesystem/sigma_fs.rs`
- `docs/AI_AGENT_DATA_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_DATA_OPERATION_MANAGEMENT.md`
