# API REFERENCE

Complete API mapping for all Sovereign Singleton shards.

---

## Core System APIs

### Per-process syscall gating with bitmask precision
- **sigma_pledge**: Capability-based syscall restriction
- **sigma_unveil**: Filesystem access control
- **sigma_mask**: Bitmask-precision permission system
- Granular control over process capabilities

### Hardware-level PQC key isolation
- **sigma_crypto**: Post-quantum cryptographic operations
- **sigma_key**: Hardware key management
- **sigma_sign**: Digital signature verification
- Kyber-1024 and Dilithium-5 integration

### Ring-0 cron replacement with macro recording
- **sigma_cron**: Advanced job scheduling
- **sigma_macro**: Workflow recording and playback
- **sigma_automate**: Task automation engine
- Sub-second precision scheduling

### AI-driven predictive workflow macro engine
- **sigma_ai_predict**: Predictive workflow suggestions
- **sigma_ai_learn**: Self-learning pattern recognition
- **sigma_ai_optimize**: Performance optimization
- Machine learning integration

### Vendor-agnostic GPU compute dispatcher
- **sigma_gpu**: Unified GPU interface
- **sigma_compute**: Compute shader management
- **sigma_render**: Graphics pipeline control
- Multi-vendor GPU support

### Self-learning PCIe driver shim generator
- **sigma_pci**: PCIe device management
- **sigma_driver**: Dynamic driver generation
- **sigma_shim**: Hardware abstraction layer
- Automatic device detection

### Hardware hang recovery timer
- **sigma_watchdog**: Hardware monitoring
- **sigma_recovery**: Automatic recovery
- **sigma_health**: System health tracking
- Fault tolerance mechanisms

### Zero-trust Ring-0 TCP/IP stack
- **sigma_net**: Network protocol stack
- **sigma_tcp**: TCP/IP implementation
- **sigma_security**: Network security layer
- Post-quantum network protocols

### Distributed, multi-node replicated filesystem
- **sigma_dfs**: Distributed file system
- **sigma_replica**: Data replication
- **sigma_sync**: Multi-node synchronization
- High availability storage

### SovereignVFS bridging for Micro-VM containers
- **sigma_vfs**: Virtual filesystem
- **sigma_container**: Container filesystem bridge
- **sigma_isolation**: Filesystem isolation
- Secure container storage

### Adaptive ambient-aware UI theme switcher
- **sigma_theme**: Dynamic theming
- **sigma_ambient**: Ambient light sensing
- **sigma_ui**: User interface adaptation
- Context-aware display

### Ring-0 hardware-accelerated gesture recognition
- **sigma_gesture**: Gesture input processing
- **sigma_input**: Hardware input acceleration
- **sigma_haptics**: Haptic feedback control
- Low-latency input processing

### Persona-driven sovereign setup wizard
- **sigma_setup**: System initialization
- **sigma_persona**: User profile management
- **sigma_config**: Configuration automation
- Personalized setup experience

### Contextual predictive quick-action engine
- **sigma_quick**: Quick action prediction
- **sigma_context**: Context awareness
- **sigma_suggest**: Action suggestions
- Intelligent workflow assistance

---

## API Categories

### System Calls
- Process management
- Memory management
- File I/O operations
- Inter-process communication

### Security APIs
- Capability-based access control
- Post-quantum cryptography
- Secure key management
- Audit trail generation

### Hardware Interfaces
- GPU compute
- Network protocols
- Storage management
- Device drivers

### User Experience
- Display management
- Input handling
- Audio processing
- Theme system

### Automation
- Task scheduling
- Workflow automation
- AI predictions
- Performance optimization

---

## API Documentation Structure

Each API includes:
- **Function signature**: Complete parameter and return type definitions
- **Usage examples**: Practical code samples
- **Error handling**: Comprehensive error codes and handling
- **Security considerations**: Capability requirements and restrictions
- **Performance characteristics**: Latency and throughput metrics

---

## Development Guidelines

### API Design Principles
- **Sovereign by default**: All APIs enforce capability-based security
- **Post-quantum ready**: Cryptographic APIs use PQC algorithms
- **Zero-dependency**: Self-contained implementations
- **Reproducible**: Deterministic behavior across executions

### Best Practices
- Always check capability requirements before API calls
- Handle errors gracefully with proper error codes
- Use appropriate security contexts for sensitive operations
- Follow capability least-privilege principles

---

## Version Compatibility

APIs are versioned with semantic versioning:
- **Major**: Breaking changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

Minimum SigmaOS version requirements are specified for each API.

---

*See also: [API-Reference.md](API-Reference.md) · [App-Manifest.md](App-Manifest.md) · [Security-Model.md](Security-Model.md)*
