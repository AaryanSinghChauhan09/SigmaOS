# SigmaOS Dependency Reduction Roadmap

*Last Updated: 2026-07-06*
*Version: 1.0*

## Executive Summary

SigmaOS has already achieved excellent dependency minimization in its core kernel - the Rust workspace uses zero external crates, relying entirely on internal sigma-* dependencies. This roadmap focuses on reducing dependencies in the userland, web UI, and desktop environment layers while maintaining functionality and improving performance.

## Current Dependency Analysis

### Core Kernel (Excellent Status)
- **Rust Workspace**: 0 external dependencies
- **Kernel**: 0 external dependencies
- **Status**: ✅ Already dependency-free

### Userland Tools (Good Status)
- **sigma-coreutils**: 0 external dependencies
- **sigma-sh**: Minimal dependencies (to be analyzed)
- **sigma-cli**: Minimal dependencies (to be analyzed)
- **Status**: ✅ Mostly dependency-free

### Web/Desktop UI (Needs Improvement)
- **package.json (Zenith Desktop)**:
  - electron: 42.5.1 (GUI framework)
  - vite: 8.1.0 (build tool)
  - vitest: 4.1.9 (testing)
  - eslint: 10.6.0 (linting)
  - prettier: 3.9.3 (formatting)
  - @eslint/js: 10.0.1
  - @vitest/ui: 4.1.9
  - jsdom: 29.1.1 (DOM testing)
  - globals: 17.7.0

- **web_ui/package.json**:
  - eslint: ^8.57.0 (linting)

### Build System
- **CMake**: External build system (industry standard, hard to replace)
- **Make**: Native Unix tool (acceptable)
- **Nim**: External language for some tools (consider replacing with Rust)

## Dependency Reduction Strategy

### Phase 1: Web UI Dependency Elimination (Weeks 1-4)

#### 1.1 Replace Electron with Native SigmaOS Desktop
**Current**: Electron 42.5.1 (Chromium + Node.js bundle, ~200MB)
**Target**: Native SigmaOS Zenith Desktop compositor
**Benefits**:
- Reduce binary size by ~200MB
- Eliminate Chromium security surface
- Better integration with SigmaOS kernel
- Improved performance (no Electron overhead)
- Native SigmaOS APIs

**Implementation**:
- Accelerate Zenith Desktop compositor development
- Implement web rendering via native SigmaOS web engine
- Port Electron APIs to SigmaOS equivalents
- **Timeline**: 4 weeks
- **Priority**: 🔴 CRITICAL

#### 1.2 Replace Vite with Native Build System
**Current**: Vite 8.1.0 (ESBuild + Rollup based)
**Target**: SigmaOS native build tool (sigma-build)
**Benefits**:
- Eliminate Node.js dependency for builds
- Faster builds (native Rust implementation)
- Better integration with SigmaOS toolchain
- Smaller build artifacts

**Implementation**:
- Create sigma-build Rust-based bundler
- Implement ES module support
- Add hot module replacement
- **Timeline**: 3 weeks
- **Priority**: 🟠 HIGH

#### 1.3 Replace ESLint with Native Linter
**Current**: ESLint 10.6.0 + plugins
**Target**: sigma-lint (Rust-based linter)
**Benefits**:
- Faster linting (Rust vs JavaScript)
- No Node.js dependency
- Better integration with SigmaOS toolchain
- Custom rules for SigmaOS patterns

**Implementation**:
- Create sigma-lint using Rust parser
- Implement common linting rules
- Add SigmaOS-specific rules
- **Timeline**: 2 weeks
- **Priority**: 🟠 HIGH

#### 1.4 Replace Vitest with Native Test Framework
**Current**: Vitest 4.1.9 + jsdom
**Target**: sigma-test (Rust-based test framework)
**Benefits**:
- Faster test execution
- No Node.js dependency
- Better integration with sigma-build
- Native SigmaOS API mocking

**Implementation**:
- Create sigma-test framework
- Implement test runner
- Add assertion library
- Implement DOM testing without jsdom
- **Timeline**: 3 weeks
- **Priority**: 🟡 MEDIUM

#### 1.5 Replace Prettier with Native Formatter
**Current**: Prettier 3.9.3
**Target**: sigma-format (Rust-based formatter)
**Benefits**:
- Faster formatting
- No Node.js dependency
- Consistent with sigma-lint
- Custom SigmaOS formatting rules

**Implementation**:
- Create sigma-format using Rust parser
- Implement code formatting
- Add SigmaOS style guide
- **Timeline**: 2 weeks
- **Priority**: 🟡 MEDIUM

### Phase 2: Toolchain Standardization (Weeks 5-8)

#### 2.1 Replace Nim Tools with Rust
**Current**: sigma-cli.nim, sigma-sh scripting in Nim
**Target**: Pure Rust implementations
**Benefits**:
- Single language for SigmaOS
- Better performance
- Smaller binary size
- Easier maintenance

**Implementation**:
- Port sigma-cli.nim to Rust
- Port sigma-sh scripting to Rust
- Update build system
- **Timeline**: 4 weeks
- **Priority**: 🟠 HIGH

#### 2.2 Replace Python Scripts with Rust
**Current**: Various Python scripts in scripts/
**Target**: Rust equivalents
**Benefits**:
- No Python runtime dependency
- Faster execution
- Better integration with SigmaOS
- Smaller installation footprint

**Implementation**:
- Audit all Python scripts
- Port critical scripts to Rust
- Remove Python from build dependencies
- **Timeline**: 3 weeks
- **Priority**: 🟡 MEDIUM

#### 2.3 Native Package Manager
**Current**: sigma-pkg (may have external dependencies)
**Target**: Pure Rust implementation
**Benefits**:
- Zero external dependencies
- Faster operations
- Better security (no external package managers)
- Smaller attack surface

**Implementation**:
- Audit sigma-pkg dependencies
- Replace any external deps with native Rust
- Implement all package management in Rust
- **Timeline**: 3 weeks
- **Priority**: 🟠 HIGH

### Phase 3: Build System Optimization (Weeks 9-12)

#### 3.1 Evaluate CMake Replacement
**Current**: CMake for cross-platform builds
**Target**: SigmaOS native build system (sigma-build)
**Benefits**:
- Faster builds
- Better SigmaOS integration
- Smaller toolchain
- Custom build optimizations

**Implementation**:
- Evaluate feasibility of CMake replacement
- Design sigma-build architecture
- Implement core build features
- Migrate critical build targets
- **Timeline**: 4 weeks
- **Priority**: 🟡 MEDIUM (CMake is industry standard, careful evaluation needed)

#### 3.2 Native Toolchain
**Current**: External toolchain (GCC/Clang, etc.)
**Target**: SigmaOS native toolchain
**Benefits**:
- Complete control over toolchain
- Better optimizations
- Smaller binaries
- SigmaOS-specific optimizations

**Implementation**:
- Evaluate LLVM-based toolchain
- Implement SigmaOS-specific optimizations
- Create custom compiler flags
- **Timeline**: 4 weeks
- **Priority**: 🟡 MEDIUM (long-term goal)

### Phase 4: Runtime Dependency Elimination (Weeks 13-16)

#### 4.1 Static Linking Strategy
**Current**: Dynamic linking for some components
**Target**: Static linking where possible
**Benefits**:
- No runtime dependencies
- Smaller attack surface
- Easier deployment
- Better performance (no PLT overhead)

**Implementation**:
- Audit all dynamically linked components
- Implement static linking for userland tools
- Use musl libc for static linking
- Update build configuration
- **Timeline**: 3 weeks
- **Priority**: 🟠 HIGH

#### 4.2 Native Crypto Implementation
**Current**: May use external crypto libraries
**Target**: Pure Rust crypto (already partially done)
**Benefits**:
- No external crypto dependencies
- Better security (audited Rust implementations)
- Smaller binary size
- SigmaOS-specific optimizations

**Implementation**:
- Audit crypto dependencies
- Replace with pure Rust implementations
- Leverage existing RustCrypto ecosystem
- **Timeline**: 2 weeks
- **Priority**: 🟠 HIGH

#### 4.3 Native Compression
**Current**: May use external compression libraries
**Target**: Pure Rust compression
**Benefits**:
- No external dependencies
- Better performance
- Smaller attack surface

**Implementation**:
- Audit compression dependencies
- Replace with Rust implementations (flate2, etc.)
- **Timeline**: 2 weeks
- **Priority**: 🟡 MEDIUM

## Dependency Reduction Targets

### Quantitative Goals
- **External Dependencies**: Reduce from ~10 to <3
- **Node.js Dependencies**: Eliminate completely (0)
- **Python Dependencies**: Eliminate completely (0)
- **Nim Dependencies**: Eliminate completely (0)
- **Binary Size**: Reduce by ~250MB (Electron elimination)
- **Build Time**: Reduce by 30% (native tools)
- **Runtime Dependencies**: 0 (fully static)

### Qualitative Goals
- **Security**: Smaller attack surface, fewer vulnerabilities
- **Performance**: Faster execution, smaller binaries
- **Maintainability**: Single language (Rust), easier updates
- **Portability**: Easier cross-compilation, fewer platform deps
- **Control**: Complete control over all dependencies

## Implementation Priority Matrix

### High Impact, Low Effort (Quick Wins)
1. Replace Prettier with sigma-format (2 weeks)
2. Replace ESLint with sigma-lint (2 weeks)
3. Audit and eliminate Python scripts (3 weeks)
4. Static linking strategy (3 weeks)

### High Impact, Medium Effort
1. Replace Electron with native Zenith Desktop (4 weeks)
2. Replace Vite with sigma-build (3 weeks)
3. Replace Nim tools with Rust (4 weeks)
4. Native package manager (3 weeks)

### High Impact, High Effort
1. Replace Vitest with sigma-test (3 weeks)
2. Evaluate CMake replacement (4 weeks)
3. Native toolchain (4 weeks)

## Risk Mitigation

### Technical Risks
- **Functionality loss**: Ensure native replacements maintain all features
- **Performance regression**: Benchmark before/after each replacement
- **Compatibility**: Ensure cross-platform compatibility maintained
- **Developer adoption**: Provide migration guides and tooling

### IP Compliance
- **Cleanroom implementation**: All native implementations from scratch
- **License compatibility**: Ensure all replacements use compatible licenses
- **Attribution**: Properly document any algorithms used

### Resource Constraints
- **Parallel development**: Execute phases in parallel where possible
- **Community contribution**: Leverage open source Rust ecosystem
- **Prioritization**: Focus on high-impact dependencies first

## Success Metrics

### Dependency Metrics
- External dependency count: 0 (target)
- Node.js modules: 0 (target)
- Python packages: 0 (target)
- Nim packages: 0 (target)

### Performance Metrics
- Binary size reduction: >250MB
- Build time reduction: >30%
- Runtime performance: >20% improvement
- Memory footprint: >15% reduction

### Security Metrics
- Vulnerability count: 0 (target)
- Attack surface: Reduced by >80%
- Dependency update frequency: 0 (no external deps)

## Related Documents

- [Comprehensive Future Development Roadmap](Comprehensive-Future-Development-Roadmap.md)
- [CURRENT_PROBLEMS_MANIFEST.md](../CURRENT_PROBLEMS_MANIFEST.md)
- [Architecture.md](../Architecture.md)
- [Ethical Feature Absorption Framework](Ethical-Feature-Absorption-Framework.md)

## Conclusion

SigmaOS has already achieved excellent dependency minimization in its core kernel. This roadmap focuses on eliminating dependencies in the web/desktop UI layer and standardizing on Rust across the entire codebase. The phased approach ensures steady progress with measurable milestones, while the priority matrix ensures resources are focused on high-impact dependencies.

The end state will be a completely dependency-free SigmaOS with:
- Zero external runtime dependencies
- Single language (Rust) across all components
- Native implementations of all tools
- Static linking for deployment
- Complete control over the entire stack
