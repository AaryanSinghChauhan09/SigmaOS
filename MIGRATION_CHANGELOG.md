# SigmaOS Documentation Migration Changelog

This changelog tracks the migration of completed `.md` files from the SigmaOS repository to the GitHub Wiki, following the selective absorption governance policy.

## Migration Policy

- **Only fully implemented `.md` files are migrated** to the Wiki
- **Partially implemented files remain** in the repository until completion
- **No duplication** - migrated files are removed from the repository
- **Clear commit messages** document each migration
- **Wiki synchronization** ensures documentation stays current

## Migration Log

### 2026-07-09 - Initial Documentation Completion

#### Completed Files (Ready for Migration)

The following repository `.md` files have been fully completed and are ready for Wiki migration:

1. **tools/debug/README.md**
   - **Status**: ✅ Completed
   - **Changes**: Expanded from minimal description to comprehensive documentation
   - **Added**: Memory tracer, kernel profiler, crash analyzer, live debugger documentation
   - **Added**: Usage examples, CI/CD integration, architecture details
   - **Added**: Roadmap and documentation links
   - **Migration Target**: `Debugging-and-Profiling-Suite.md`

2. **tools/sdk/README.md**
   - **Status**: ✅ Completed
   - **Changes**: Expanded from minimal description to comprehensive SDK documentation
   - **Added**: Documentation for 8 SDKs (Rust, C/C++, Python, Nim, WASM, IoT, HPC, Cloud)
   - **Added**: Installation instructions, examples, tooling documentation
   - **Added**: Roadmap and contributing guidelines
   - **Migration Target**: `Sovereign-SDKs.md`

3. **tools/compat/README.md**
   - **Status**: ✅ Completed
   - **Changes**: Updated roadmap with completed items
   - **Completed**: ELF loader, mmap translation, dynamic linker shim
   - **Added**: Additional roadmap items for future completion
   - **Migration Target**: `POSIX-Compatibility-Shim.md`

4. **virtualization/containers/README.md**
   - **Status**: ✅ Completed
   - **Changes**: Updated roadmap with completed items
   - **Completed**: sigma-run runtime, shard bundle format, mesh shards networking
   - **Added**: Extended roadmap for container orchestration features
   - **Migration Target**: `Sovereign-Container-Framework.md`

5. **virtualization/hypervisor/README.md**
   - **Status**: ✅ Completed
   - **Changes**: Updated roadmap with completed items
   - **Completed**: VMCS/VMCB setup, guest memory isolation, virtio devices, live migration
   - **Added**: Extended roadmap for advanced virtualization features
   - **Migration Target**: `Sovereign-Hypervisor.md`

6. **TODO.md**
   - **Status**: ✅ Updated
   - **Changes**: Marked documentation tasks as completed
   - **Completed**: Arch Wiki-style knowledge base, debugging tools documentation, SDK documentation
   - **Note**: This file remains in repository as active task tracker

#### Files Already Complete (Ready for Migration)

The following repository `.md` files were already complete and ready for migration:

7. **README.md**
   - **Status**: ✅ Complete
   - **Migration Target**: `Home.md` (Wiki homepage)

8. **MAINTAINERS**
   - **Status**: ✅ Complete
   - **Migration Target**: `Maintainers.md`

9. **usr/README.md**
   - **Status**: ✅ Complete
   - **Migration Target**: `SigmaOS-Userland.md`

10. **kernel/core/hal/README.md**
    - **Status**: ✅ Complete
    - **Migration Target**: `Hardware-Abstraction-Layer.md`

11. **tools/sigma-cc/README.md**
    - **Status**: ✅ Complete
    - **Migration Target**: `SigmaCC-Compiler.md`

12. **tools/syscall_profiler/README.md**
    - **Status**: ✅ Complete
    - **Migration Target**: `Syscall-Profiler.md`

13. **userland/agent/README.md**
    - **Status**: ✅ Complete
    - **Migration Target**: `sigma-agent.md`

## Migration Status

### Pending Migration
- [ ] tools/debug/README.md → Debugging-and-Profiling-Suite.md
- [ ] tools/sdk/README.md → Sovereign-SDKs.md
- [ ] tools/compat/README.md → POSIX-Compatibility-Shim.md
- [ ] virtualization/containers/README.md → Sovereign-Container-Framework.md
- [ ] virtualization/hypervisor/README.md → Sovereign-Hypervisor.md
- [ ] README.md → Home.md
- [ ] MAINTAINERS → Maintainers.md
- [ ] usr/README.md → SigmaOS-Userland.md
- [ ] kernel/core/hal/README.md → Hardware-Abstraction-Layer.md
- [ ] tools/sigma-cc/README.md → SigmaCC-Compiler.md
- [ ] tools/syscall_profiler/README.md → Syscall-Profiler.md
- [ ] userland/agent/README.md → sigma-agent.md

### Remaining in Repository
- **TODO.md** - Active task tracker (keep in repository)

## Migration Instructions

For each file marked as "Pending Migration":

1. **Copy content** from repository file to corresponding Wiki page
2. **Verify formatting** and links in the Wiki
3. **Update internal links** to point to Wiki URLs instead of repository paths
4. **Remove repository file** after successful Wiki migration
5. **Commit with message**: `"Migrated [filename] → [wiki-page]"`
6. **Update this changelog** to mark migration as complete

## Governance Notes

- Contributors must finalize `.md` files before requesting migration
- All migrations must be documented in this changelog
- Wiki and repository must stay synchronized after each migration
- No trace of migrated files should remain in the repository

## Next Steps

1. Execute pending migrations following the instructions above
2. Verify all Wiki pages are properly formatted and linked
3. Update repository to remove migrated files
4. Commit all changes with clear migration messages
5. Update this changelog to reflect completed migrations
