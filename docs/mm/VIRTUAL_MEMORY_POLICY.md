# Virtual Memory Manager Policy (`VIRTUAL_MEMORY_POLICY.md`)

## 1. Overview & Objectives
The Virtual Memory Manager (`VirtualMemoryManager` in `src/mm/virtual.rs`) maintains process address space layout, Virtual Memory Areas (VMAs), permission checks, user/kernel memory isolation, W^X security policies, and resource limits.

Key Requirements:
- **User/Kernel Space Isolation**: User space addresses reside in `0x00000000_00010000 .. 0x00007FFF_FFFF_FFFF` (`USER_SPACE_START`). Kernel space addresses reside above `0xFFFF8000_0000_0000` (`KERNEL_SPACE_START`). User-accessible VMAs in kernel space strictly fail with `MmError::PermissionDenied`.
- **W^X Protection Policy**: Virtual Memory Areas CANNOT be simultaneously Writable and Executable (`write == true && execute == true`). Violations yield `MmError::PermissionDenied`.
- **Permission Checking**: Access attempts validate requested operations (Read, Write, Execute) and ring mode (User vs Kernel).
- **Resource Limits**: Clamped by maximum VMA count (`max_vmas`) and byte capacity (`max_bytes`). Exceeding capacity returns `MmError::OutOfMemory`.

---

## 2. Protection & Boundary Rules

1. **User Mode Ring Check**: User mode access attempts to addresses above `0x00007FFF_FFFF_FFFF` strictly return `MmError::PermissionDenied`.
2. **W^X Security Enforcement**: Any VMA creation or modification request with both `write: true` and `execute: true` is rejected immediately.
3. **Unmapped Address Handling**: Access to unmapped virtual memory returns `MmError::NotFound`.
4. **Capacity Clamping**: Address space allocation exceeding configured VMA count or byte limit returns `MmError::OutOfMemory`.

---

## 3. Implementation Verification
Verified by unit tests in `tests/mm/virtual.rs`:
- `test_vma_user_kernel_isolation`
- `test_vma_wx_protection_policy`
- `test_vma_permission_checks`
- `test_vma_oom_limit_enforcement`
