# Contributing

> Full guidelines: [CONTRIBUTING.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTING.md)

## Quick Start

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
cargo check
cargo test
```

## Core Principles

1. **Zero stdlib dependency** — use `src/klib/` collections, not `std::collections`
2. **No hardcoded credentials** — use domain-separation constants, not passwords
3. **All unsafe needs `// SAFETY:` comments**
4. **No `unwrap()` in library code** — return `Result<T, E>`
5. **OOP via traits** — model interfaces as Rust traits, not inheritance

## PR Format

```
feat(kernel): add NUMA-aware scheduler        ← type(scope): description
fix(crypto): remove hardcoded key in LUKS     
docs(wiki): add architecture overview         
security(capability): fix bitmask overlap     
refactor(klib): optimize hashmap probe seq    
```

## Priority Areas

| Area | Priority |
|---|---|
| Bootloader UEFI pointer safety | 🔴 Critical |
| JS XSS fixes in web UI | 🔴 Critical |
| Native filesystem drivers (ext4, Btrfs) | 🟠 High |
| GPU driver completion | 🟠 High |
| TCP/IP stack completeness | 🟠 High |
| Documentation & Wiki | 🟢 Low |

## Testing

Add a `#[cfg(test)]` module in every new file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_feature() {
        // arrange
        let mut svc = MyService::new();
        // act
        let result = svc.do_thing();
        // assert
        assert!(result.is_ok());
    }
}
```

## Code of Conduct

Be constructive. Personal attacks result in removal. See [CODE_OF_CONDUCT.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CODE_OF_CONDUCT.md).
