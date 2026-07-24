# 🌐 Nim Integration Plan for SigmaOS

This document specifies how Nim's ARC memory management and zero-dependency compilation target are utilized to develop unprivileged user-space micro-utilities for SigmaOS.

---

## 1. Zero-Dependency Micro-Shell Tools
Nim compiles directly to highly optimized C/C++ source code, which can be linked statically with SigmaOS’s userland interfaces.

### Nim Implementation (Secure Path Canonicalizer)
```nim
type
  PathMeta* = object
    length*: int
    isValid*: bool

  Canonicalizer* = ref object of RootObj
    basePath*: string
    meta*: PathMeta

method canonicalize*(self: Canonicalizer, relativePath: string): string {.base.} =
  if relativePath.contains(".."):
    self.meta.isValid = false
    return self.basePath
  else:
    self.meta.isValid = true
    return self.basePath & "/" & relativePath

proc newCanonicalizer*(base: string): Canonicalizer =
  new(result)
  result.basePath = base
  result.meta = PathMeta(length: base.len, isValid: true)
```

---

## 2. Dynamic Process Spawning
Utilizes userland IPC messaging to request capability escalation from the microkernel security shard.
