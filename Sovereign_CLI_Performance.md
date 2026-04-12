# Σ Sovereign CLI: Industrial Performance Upgrade

The **Sovereign Unified CLI Dispatcher** has been upgraded to a high-performance, hash-based architecture. This modernization ensures that SigmaOS maintains O(1) command lookup speed regardless of the number of active shards, facilitating rapid industrial automation.

## Key Improvements

### 1. DJB2 Native Hashing
The CLI now utilizes a hardware-aligned `djb2` hash algorithm implemented in pure C11. This algorithm minimizes collisions and provides near-instantaneous mapping from command strings to silicon execution entry points.

### 2. Linear-Probed Command Mapping
A specialized hash map with linear probing handles command registration. This ensures that even in the unlikely event of a hash collision, the system maintains deterministic and peak performance sovereignty.

### 3. Native Command Aliasing
Citizens can now create personalized command aliases to streamline their workflow. This feature allows for the customization of the terminal interface to suit individual mission objectives.

## New CLI Features

### `sigma-alias`
Create a native shortcut for any existing command.

```bash
# Register a personalized alias
sigma-alias quick-ai sigma-ai

# Use the new alias to trigger the AI matrix
quick-ai train
```

## Architectural Specifications

| Feature | Specification | Standard |
| :--- | :--- | :--- |
| Lookup Complexity | O(1) | Industrial |
| Max Commands | 128 | Zenith |
| Hash Table Size | 256 | Optimized |
| Collision Strategy | Linear Probing | Deterministic |

---
**Σ SIGMAOS: PERFORMANCE IS SOVEREIGN.**
