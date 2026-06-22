# SigmaOS API Reference

Welcome to the SigmaOS API Reference. SigmaOS bypasses standard POSIX/glibc interfaces in favor of a zero-abstraction **Sovereign Interface** for maximum performance and security. 

*(Note: We do provide a POSIX shim via `sigma_musl_shim` for porting existing applications, but native apps should use the APIs below.)*

## 1. Process & Isolation (`sigma_jail`)

SigmaOS isolates all processes into zero-copy shards called "procs" or "realms".

```cpp
#include "sigma_jail.h"

// Create a new isolation shard
int sigma_jail_create(const char* name, int flags);

// Drop privileges permanently for the current shard
int sigma_jail_drop_caps(uint64_t cap_mask);
```

## 2. Storage & Vector Embedding (`SemanticFS`)

Instead of standard inodes, SigmaOS uses a vector-indexed filesystem.

```cpp
#include "sigma_fs.h"

// Insert a file into SemanticFS
int sigma_fs_write(const char* path, const void* data, size_t len);

// Query files by semantic meaning (uses native AI inference)
int sigma_fs_semantic_query(const char* natural_language, char** results, int max_results);
```

## 3. Mandatory Access Control (`sigma_mac`)

Permissions are label-based, preventing unauthorized syscalls entirely at Ring 0.

```cpp
#include "sigma_mac.h"

// Check if current shard has permission to access a target label
bool sigma_mac_enforce(const char* target_label, int action);
```

## 4. Zenith GUI Framework

Native windowing without X11 or Wayland overhead.

```cpp
#include "libzenith/zenith.h"

// Initialize native UI connection
zenith_ctx_t* zenith_init();

// Create an accelerated UI widget
int zenith_widget_create(zenith_ctx_t* ctx, widget_type_t type, const char* label);
```
