# Sovereign Persona Shard (persona.shard)

## Overview

The **Persona Shard** is a core modular component of the SigmaOS Sovereign Lattice responsible for identity isolation, deep personalization, and secure context switching. It ensures that user identities are strictly siloed at the silicon level.

## Architecture

- **Shard ID**: persona.shard
- **Namespace**: SigmaOS::Shards::Persona
- **Pattern**: Industrial Singleton
- **Inheritance**: SigmaOS::SigmaObject

## Key Features

### 1. Atomic Context Switching

Persona transitions are handled atomically. When a user switches from one persona to another (e.g., Work to Private), the shard triggers a mandatory security purge:

- **L1/L2 Cache Flushing**: Prevents cross-persona data leakage.
- **Register Scrubbing**: Clears general-purpose registers before the new context takes control.

### 2. Deep Personalization Mapping

Each persona context contains:

- **Identity Shard**: Canonical ID for the user context.
- **Visual Theme**: Dynamic mapping to Zenith UI profiles (e.g., Industrial Dark, Frost, Aurora).
- **Capability Mask**: Bitmask defining allowed kernel-level operations for that specific identity.

## API Reference (C++ Shard)

`cpp
namespace SigmaOS::Shards::Persona {
    class SovereignPersonaShard {
        void init();
        sigma_status switchContext(const char* persona_id);
        void updateVisuals(const char* theme_id);
        const PersonaContext& getActiveContext() const;
    };
}
`

## C Bridge Integration

For low-level kernel and driver access:

- void persona_shard_init()
- void persona_shard_switch(const char* id)

---

### Status: MODULARIZED [ACTIVE]

### Lattice Integration: Phase 47
