# Standard Library Reduction Plan

## Current State Analysis

The codebase currently has extensive usage of `std::` library components:
- **HashMap**: Used in ~15+ files
- **VecDeque**: Used in productivity modules
- **Path/PathBuf**: Used in filesystem and toolchain modules
- **Time/Duration**: Used in productivity and resilience modules
- **Atomic types**: Used in hardware and graphics modules
- **Allocator**: Used in ~20+ files for memory allocation

## Reduction Strategy

### Phase 1: Replace Collections (High Priority)

#### HashMap Replacement
**Files affected:**
- `src/shell/repl.rs`
- `src/compatibility/fedora.rs`
- `src/package/universal.rs`
- `src/compatibility/linux_adapter.rs`
- `src/sigpkg/recipe.rs`
- `src/productivity/tmux.rs`
- `src/productivity/advanced_app_absorber.rs`
- `src/filesystem/defragmenter.rs`
- `src/toolchain/self_host.rs`
- `src/filesystem/vfs.rs`
- `src/resilience/self_healing.rs`

**Action:** Implement custom HashMap in `src/klib/hashmap.rs`

#### VecDeque Replacement
**Files affected:**
- `src/productivity/advanced_app_absorber.rs`

**Action:** Implement custom VecDeque in `src/klib/vecdeque.rs`

### Phase 2: Replace Path Handling (Medium Priority)

#### Path/PathBuf Replacement
**Files affected:**
- `src/productivity/screen_recorder.rs`
- `src/productivity/advanced_app_absorber.rs`
- `src/toolchain/self_host.rs`

**Action:** Implement custom Path handling in `src/klib/path.rs`

### Phase 3: Replace Time Handling (Medium Priority)

#### Time/Duration Replacement
**Files affected:**
- `src/productivity/screen_recorder.rs`
- `src/resilience/self_healing.rs`

**Action:** Implement custom time handling in `src/klib/time.rs`

### Phase 4: Replace Atomic Types (Low Priority)

#### Atomic Types Replacement
**Files affected:**
- `src/hardware/compatibility.rs`
- `src/graphics/compositor.rs`

**Action:** These are already using `core::sync::atomic` which is part of core, not std

### Phase 5: Allocator Standardization (Already Implemented)

The allocator shim pattern is already implemented in ~20 files:
```rust
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}
```

**Action:** Keep this pattern for hosted targets, ensure bare-metal compatibility

## Implementation Priority

### Immediate (Week 1)
1. Implement custom HashMap
2. Replace HashMap in shell/repl.rs
3. Replace HashMap in sigpkg modules

### Short Term (Week 2-3)
1. Implement custom VecDeque
2. Replace VecDeque in productivity modules
3. Implement custom Path handling
4. Replace Path usage in toolchain modules

### Medium Term (Month 1)
1. Implement custom time handling
2. Replace time usage in productivity modules
3. Audit and replace remaining HashMap usage

### Long Term (Month 2-3)
1. Complete std:: removal from core modules
2. Ensure no_std compatibility for all modules
3. Testing and validation

## Custom Implementation Templates

### HashMap Template
```rust
// src/klib/hashmap.rs
pub struct HashMap<K, V> {
    buckets: Vec<Option<Vec<(K, V)>>>,
    capacity: usize,
}

impl<K, V> HashMap<K, V> 
where
    K: PartialEq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            capacity: 16,
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) {
        // Custom implementation
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        // Custom implementation
    }
}
```

### Path Template
```rust
// src/klib/path.rs
pub struct Path {
    components: Vec<[u8; 256]>,
}

impl Path {
    pub fn new(path: &[u8]) -> Self {
        // Custom implementation
    }
    
    pub fn extension(&self) -> Option<&[u8]> {
        // Custom implementation
    }
}
```

## Testing Strategy

1. **Unit Tests**: Test each custom implementation thoroughly
2. **Integration Tests**: Ensure compatibility with existing code
3. **Performance Tests**: Compare performance with std versions
4. **Memory Tests**: Verify memory usage is acceptable

## Success Criteria

- [ ] Zero `std::` usage in core kernel modules
- [ ] Minimal `std::` usage in userspace (only for hosted targets)
- [ ] All custom implementations fully tested
- [ ] Performance comparable to or better than std versions
- [ ] Memory usage within acceptable limits

## Rollback Plan

If custom implementations prove problematic:
1. Feature-gate custom implementations
2. Fall back to std:: versions when needed
3. Gradual migration path
4. Maintain compatibility during transition

## References

- [Rustonomicon - No_std](https://rust-embedded.github.io/book/intro/no-std.html)
- [The Rustonomicon - Alloc](https://rust-embedded.github.io/book/alloc/index.html)
- [Awesome No_std Collections](https://github.com/rust-embedded/awesome-embedded-rust#no_std-ecosystem)