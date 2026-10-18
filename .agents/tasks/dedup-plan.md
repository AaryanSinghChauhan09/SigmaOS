# SigmaOS E0428 Duplicate Type Definition Deduplication Plan

**Investigation Date**: 2025-01-19
**Total E0428 Errors**: 16
**Total Files Affected**: 7
**Strategy**: KEEP first occurrence, DELETE later duplicate definition(s)

---

## Priority Order (Most Duplicates First)

### 1. src/userland/init.rs
**Line Count**: 1996 lines  
**Error Count**: 6 duplicates  
**Estimated Reduction**: 6 E0428 errors

#### Duplicates to Fix:

1. **SystemdSocketActivationEngine**
   - KEEP: Line 169 (first definition) - `pub struct SystemdSocketActivationEngine { ... }`
   - DELETE: Line 439 (duplicate) - `pub struct SystemdSocketActivationEngine { ... }`
   - Action: Remove lines 439 and all fields/methods until the closing brace

2. **SystemdTargetDependencyResolver**
   - KEEP: Line 210 (first definition) - `pub struct SystemdTargetDependencyResolver;`
   - DELETE: Line 467 (duplicate) - `pub struct SystemdTargetDependencyResolver { ... }`
   - Action: Remove lines 467 and all fields/methods until the closing brace

3. **SystemdPathMonitorEngine**
   - KEEP: Line 249 (first definition) - `pub struct SystemdPathMonitorEngine { ... }`
   - DELETE: Line 502 (duplicate) - `pub struct SystemdPathMonitorEngine { ... }`
   - Action: Remove lines 502 and all fields/methods until the closing brace

4. **SystemdTransientScopeGovernor**
   - KEEP: Line 301 (first definition) - `pub struct SystemdTransientScopeGovernor { ... }`
   - DELETE: Line 533 (duplicate) - `pub struct SystemdTransientScopeGovernor { ... }`
   - Action: Remove lines 533 and all fields/methods until the closing brace

5. **BsdRcSubrEngine**
   - KEEP: Line 1009 (first definition) - `pub struct BsdRcSubrEngine { ... }`
   - DELETE: Line 1092 (duplicate) - `pub struct BsdRcSubrEngine { ... }`
   - Action: Remove lines 1092 and all fields/methods until the closing brace

6. **AnacronSpoolManager**
   - KEEP: Line 1210 (first definition) - `pub struct AnacronSpoolManager { ... }`
   - DELETE: Line 1421 (duplicate) - `pub struct AnacronSpoolManager { ... }`
   - Action: Remove lines 1421 and all fields/methods until the closing brace

7. **UserlandInitCleanRoomPrGatewayEngine**
   - KEEP: Line 1297 (first definition) - `pub struct UserlandInitCleanRoomPrGatewayEngine { ... }`
   - DELETE: Line 1725 (duplicate type alias) - `pub type UserlandInitCleanRoomPrGatewayEngine = UserlandInitPrGatewayEngine;`
   - Action: Remove line 1725 completely

---

### 2. src/creative/effectcraft_missing_components_parity_pr.rs
**Line Count**: 1748 lines  
**Error Count**: 3 duplicates  
**Estimated Reduction**: 3 E0428 errors

#### Duplicates to Fix:

1. **EffectcraftOpticalFlowMotionVectorPrEngine**
   - KEEP: Line 530 (first definition) - `pub struct EffectcraftOpticalFlowMotionVectorPrEngine { ... }`
   - DELETE: Line 536 (duplicate type alias) - `pub type EffectcraftOpticalFlowMotionVectorPrEngine = ...;`
   - Action: Remove line 536 completely

2. **FilmGrainProfile**
   - KEEP: Line 892 (first definition) - `pub struct FilmGrainProfile { ... }`
   - DELETE: Line 1160 (duplicate) - `pub struct FilmGrainProfile { ... }`
   - Action: Remove lines 1160 and all fields/methods until the closing brace

3. **TemporalNoiseReductionMotionDegrainPrEngine**
   - KEEP: Line 899 (first definition) - `pub struct TemporalNoiseReductionMotionDegrainPrEngine;`
   - DELETE: Line 1168 (duplicate) - `pub struct TemporalNoiseReductionMotionDegrainPrEngine { ... }`
   - Action: Remove lines 1168 and all fields/methods until the closing brace

---

### 3. src/virtualization/sovereign_virtualization_and_containers_pr.rs
**Line Count**: 1587 lines  
**Error Count**: 2 duplicates  
**Estimated Reduction**: 2 E0428 errors

#### Duplicates to Fix:

1. **QemuDiskImageSpec**
   - KEEP: Line 61 (first definition) - `pub struct QemuDiskImageSpec { ... }`
   - DELETE: Line 81 (duplicate) - `pub struct QemuDiskImageSpec { ... }`
   - Action: Remove lines 81 and all fields/methods until the closing brace

2. **LibvirtStorageVolume**
   - KEEP: Line 194 (first definition) - `pub struct LibvirtStorageVolume { ... }`
   - DELETE: Line 222 (duplicate) - `pub struct LibvirtStorageVolume { ... }`
   - Action: Remove lines 222 and all fields/methods until the closing brace

---

### 4. src/creative/cadcraft.rs
**Line Count**: 997 lines  
**Error Count**: 1 duplicate  
**Estimated Reduction**: 1 E0428 error

#### Duplicates to Fix:

1. **BRepSolidModel**
   - KEEP: Line 133 (first definition) - `pub struct BRepSolidModel { ... }`
   - DELETE: Line 230 (duplicate) - `pub struct BRepSolidModel { ... }`
   - Action: Remove lines 230 and all fields/methods until the closing brace

---

### 5. src/creative/photocraft_missing_components_parity_pr.rs
**Line Count**: 871 lines  
**Error Count**: 1 duplicate  
**Estimated Reduction**: 1 E0428 error

#### Duplicates to Fix:

1. **PhotocraftColorGradingFrequencySeparationPrEngine**
   - KEEP: Line 244 (first definition) - `pub struct PhotocraftColorGradingFrequencySeparationPrEngine { ... }`
   - DELETE: Line 324 (duplicate) - `pub struct PhotocraftColorGradingFrequencySeparationPrEngine;`
   - Action: Remove line 324 completely

---

### 6. src/governance/sovereign_task_guidelines_wiki_sync_engine.rs
**Line Count**: 834 lines  
**Error Count**: 1 duplicate  
**Estimated Reduction**: 1 E0428 error

#### Duplicates to Fix:

1. **SupremePerformanceAddpEngine**
   - KEEP: Line 436 (first definition) - `pub struct SupremePerformanceAddpEngine { ... }`
   - DELETE: Line 488 (duplicate) - `pub struct SupremePerformanceAddpEngine { ... }`
   - Action: Remove lines 488 and all fields/methods until the closing brace

---

### 7. src/sovereign_wiki_master_engine.rs
**Line Count**: 579 lines  
**Error Count**: 1 duplicate  
**Estimated Reduction**: 1 E0428 error

#### Duplicates to Fix:

1. **SovereignArchitectureDoc**
   - KEEP: Line 23 (first definition in enum) - `SovereignArchitectureDoc,`
   - DELETE: Line 29 (duplicate in same enum) - `SovereignArchitectureDoc,`
   - Action: Remove line 29 completely (single line removal)
   - Note: This is within an enum variant list, so just delete the duplicate variant entry

---

## Verification Strategy

After each file is fixed:

1. **Per-file verification**:
   ```bash
   cargo check --lib 2>&1 | grep -E "(error\[E0428\]|error\[E0252\])" | grep "<filename>"
   ```

2. **Full verification after all fixes**:
   ```bash
   cargo check --lib 2>&1 | grep "error\[E0428\]" | wc -l
   ```
   Expected result: 0 E0428 errors

3. **Build verification**:
   ```bash
   cargo check --lib
   ```
   Expected: No E0428 errors, only E0252 (name import conflicts) if any remain

---

## Implementation Notes

### Pattern Recognition

All duplicates follow one of three patterns:

1. **Struct redefinition**: Full struct defined twice with fields
   - Action: Delete entire second struct definition block
   
2. **Type alias after struct**: Struct defined first, then type alias with same name
   - Action: Delete the type alias line
   
3. **Enum variant duplication**: Same variant listed twice in enum
   - Action: Delete the duplicate variant line

### Critical Rules

1. **Always preserve the FIRST occurrence** (lowest line number)
2. **Delete the LATER occurrence** (higher line number)
3. **For struct definitions**: Delete from `pub struct Name` to closing `}`
4. **For type aliases**: Delete single line `pub type Name = ...;`
5. **For enum variants**: Delete single line with variant name
6. **Maintain file formatting**: Do not introduce extra blank lines or remove necessary spacing

### Dependency Order

Files can be fixed in any order since each duplicate is self-contained within its file. However, the recommended order prioritizes files with the most duplicates:

1. init.rs (6 errors) - HIGHEST PRIORITY
2. effectcraft_missing_components_parity_pr.rs (3 errors)
3. sovereign_virtualization_and_containers_pr.rs (2 errors)
4. cadcraft.rs (1 error)
5. photocraft_missing_components_parity_pr.rs (1 error)
6. sovereign_task_guidelines_wiki_sync_engine.rs (1 error)
7. sovereign_wiki_master_engine.rs (1 error)

---

## Expected Outcomes

- **Total E0428 errors eliminated**: 16
- **Files modified**: 7
- **Build status improvement**: Removal of all E0428 duplicate definition errors
- **No functional changes**: Only removing duplicate declarations; keeping first definitions ensures existing code references remain valid

---

## Post-Fix Actions

1. Run full test suite:
   ```bash
   cargo test --lib
   ```

2. Run clippy for additional warnings:
   ```bash
   cargo clippy --all-targets -- -D warnings
   ```

3. Verify formatting:
   ```bash
   cargo fmt --check
   ```

4. Address any remaining E0252 import conflicts (separate from E0428 duplicate definitions)

---

**End of Deduplication Plan**
