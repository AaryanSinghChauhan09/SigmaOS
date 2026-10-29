# SigmaOS Buffer Manager Policy

## 1. Subsystem Architecture
The Buffer Manager (`src/fs/buffer.rs`) provides bounded page buffer cache sets for filesystem block I/O, cache eviction, dirty page flushing, and I/O buffer management.

## 2. Key Rules & Guarantees

1. **Buffer Set Representation**:
   - Buffer caches are organized as fixed-capacity buffer sets (`BufferSetManager`) mapping block IDs to page-aligned memory buffers (`BufferEntry`).

2. **Access & Update Semantics**:
   - Read/write access locks buffer entries.
   - Updates mark buffers dirty; dirty buffers are flushed to persistent block storage before eviction.

3. **Legal Eviction Semantics**:
   - Least Recently Used (LRU) eviction algorithm selects clean buffers for eviction when capacity is reached.
   - Locked or currently referenced buffers are exempt from eviction.

4. **Buffer Exhaustion & Error Semantics**:
   - If all buffers are locked or dirty during an eviction request, the manager triggers an emergency flush.
   - If flush fails or capacity remains exhausted, `BufferError::CacheFull` or `BufferError::Exhausted` is returned.

5. **Limits & NUMA Alignment**:
   - Buffer pool size is bounded (`DEFAULT_BUFFER_CAPACITY = 2048` blocks).
   - Buffer allocations align to 4096-byte page boundaries.
