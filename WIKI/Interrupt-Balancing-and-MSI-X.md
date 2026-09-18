# Interrupt Balancing and MSI-X Support

SigmaOS implements advanced interrupt balancing and MSI-X (Message Signaled Interrupts Extended) support to optimize interrupt handling across multiple CPU cores and improve system performance.

## Overview

Interrupt balancing distributes interrupt processing across available CPU cores to prevent any single core from becoming overwhelmed. MSI-X provides enhanced interrupt capabilities with per-vector masking and affinity control.

## Architecture

### Interrupt Types
- **Legacy IRQs**: Traditional pin-based interrupts (0-15)
- **PCI MSI**: Message Signaled Interrupts (32 vectors)
- **PCI MSI-X**: Extended MSI (up to 2048 vectors)
- **Local APIC**: Advanced Programmable Interrupt Controller
- **IOAPIC**: I/O Advanced Programmable Interrupt Controller

### Interrupt Pipeline
1. **Hardware**: Device raises interrupt
2. **I/O APIC**: Routes interrupt to Local APIC
3. **Local APIC**: Delivers to target CPU
4. **ISR**: Interrupt Service Routine executes
5. **Bottom Half**: Deferred processing runs

## Implementation

### Interrupt Controller
```rust
// src/kernel/interrupt.rs
pub struct InterruptController {
    ioapic: IoApic,
    local_apics: Vec<LocalApic>,
    msi_vector_allocator: MsiVectorAllocator,
    irq_affinity: BTreeMap<u32, CpuMask>,
    irq_balance_mode: BalanceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceMode {
    Disabled,
    RoundRobin,
    LeastLoaded,
    CacheAware,
    NumaAware,
}

impl InterruptController {
    pub fn balance_interrupts(&mut self) {
        match self.irq_balance_mode {
            BalanceMode::RoundRobin => self.balance_round_robin(),
            BalanceMode::LeastLoaded => self.balance_least_loaded(),
            BalanceMode::CacheAware => self.balance_cache_aware(),
            BalanceMode::NumaAware => self.balance_numa_aware(),
            BalanceMode::Disabled => {},
        }
    }

    fn balance_least_loaded(&mut self) {
        let cpu_loads = self.get_cpu_loads();
        
        for (irq, _) in self.irq_affinity.iter_mut() {
            let least_loaded = cpu_loads.iter()
                .min_by_key(|(_, load)| load)
                .map(|(cpu, _)| *cpu)
                .unwrap_or(0);
            
            self.set_irq_affinity(*irq, least_loaded);
        }
    }

    pub fn set_irq_affinity(&mut self, irq: u32, cpu: u32) {
        let cpu_mask = 1u64 << cpu;
        self.irq_affinity.insert(irq, cpu_mask);
        
        // Program hardware
        self.ioapic.set_irq_affinity(irq, cpu_mask);
        self.local_apics[cpu as usize].set_irq_affinity(irq, cpu_mask);
    }
}
```

### MSI-X Vector Allocation
```rust
// src/kernel/msix.rs
pub struct MsiVectorAllocator {
    vectors: BTreeMap<u16, bool>, // Vector -> Used
    next_vector: AtomicU16,
    max_vectors: u16,
}

impl MsiVectorAllocator {
    pub fn allocate(&self, count: u16) -> Result<Vec<u16>, MsiError> {
        let mut allocated = Vec::new();
        let mut current = self.next_vector.load(Ordering::Acquire);
        
        for _ in 0..count {
            loop {
                if current >= self.max_vectors {
                    return Err(MsiError::NoVectors);
                }
                
                if !self.vectors.contains_key(&current) {
                    allocated.push(current);
                    current += 1;
                    break;
                }
                current += 1;
            }
        }
        
        self.next_vector.store(current, Ordering::Release);
        Ok(allocated)
    }

    pub fn enable_msix(&self, device: &PciDevice, vectors: &[u16]) -> Result<(), MsiError> {
        // Enable MSI-X in PCI config space
        let config = device.config_space();
        config.write_u16(device.address, PCI_MSIX_ENABLE, 1);
        
        // Program MSI-X table
        for (i, &vector) in vectors.iter().enumerate() {
            let table_offset = i * 16;
            config.write_u32(device.address, PCI_MSIX_TABLE + table_offset, vector as u32);
            config.write_u32(device.address, PCI_MSIX_TABLE + table_offset + 4, 0);
        }
        
        Ok(())
    }
}
```

### Interrupt Statistics
```rust
// src/kernel/interrupt_stats.rs
pub struct InterruptStats {
    per_cpu_stats: Vec<CpuInterruptStats>,
    per_irq_stats: BTreeMap<u32, IrqStats>,
}

#[derive(Debug, Clone)]
pub struct CpuInterruptStats {
    pub total_interrupts: AtomicU64,
    pub spurious_interrupts: AtomicU64,
    pub latency_sum: AtomicU64,
    pub latency_count: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct IrqStats {
    pub total_interrupts: AtomicU64,
    pub last_cpu: AtomicU32,
    pub affinity_changes: AtomicU32,
}

impl InterruptStats {
    pub fn record_interrupt(&self, irq: u32, cpu: u32, latency_ns: u64) {
        self.per_cpu_stats[cpu as usize].total_interrupts.fetch_add(1, Ordering::Relaxed);
        self.per_cpu_stats[cpu as usize].latency_sum.fetch_add(latency_ns, Ordering::Relaxed);
        self.per_cpu_stats[cpu as usize].latency_count.fetch_add(1, Ordering::Relaxed);
        
        if let Some(stats) = self.per_irq_stats.get_mut(&irq) {
            stats.total_interrupts.fetch_add(1, Ordering::Relaxed);
            stats.last_cpu.store(cpu, Ordering::Relaxed);
        }
    }

    pub fn get_imbalance_score(&self) -> f64 {
        let mut counts: Vec<u64> = self.per_cpu_stats.iter()
            .map(|stats| stats.total_interrupts.load(Ordering::Relaxed))
            .collect();
        
        if counts.is_empty() {
            return 0.0;
        }
        
        let mean = counts.iter().sum::<u64>() as f64 / counts.len() as f64;
        let variance = counts.iter()
            .map(|&c| (c as f64 - mean).powi(2))
            .sum::<f64>() / counts.len() as f64;
        
        variance.sqrt() / mean
    }
}
```

## Configuration

### Interrupt Balancing Configuration
```toml
# /etc/sigmaos/interrupts.toml
[balancing]
mode = "least_loaded"
check_interval_ms = 1000
enable_migrator = true

[irq.affinity]
# Specific IRQ affinity settings
"eth0" = [0, 1, 2, 3]
"nvme0" = [4, 5, 6, 7]

[msix]
enabled = true
max_vectors_per_device = 2048
enable_masking = true

[latency]
target_us = 50
max_us = 1000
```

### Runtime Control
```bash
# View interrupt statistics
sigirq stats

# View per-CPU interrupt distribution
sigirq per-cpu

# View IRQ affinity
sigirq affinity <irq>

# Set IRQ affinity
sigirq set-affinity <irq> <cpu_mask>

# Enable interrupt balancing
sigirq balance enable

# Disable interrupt balancing
sigirq balance disable

# View MSI-X vectors
sigirq msix list <device>

# Allocate MSI-X vectors
sigirq msix allocate <device> <count>
```

## Performance Optimization

### Cache-Aware Balancing
Balance interrupts to maximize cache locality:
- Group related interrupts on same CPU
- Prefer CPU where data is cached
- Minimize cross-CPU cache invalidations

### NUMA-Aware Balancing
For NUMA systems:
- Balance interrupts within NUMA node
- Minimize cross-node interrupt traffic
- Consider memory locality

### IRQ Threading
Move interrupt processing to kernel threads:
- Reduces interrupt latency
- Allows preemption
- Improves system responsiveness

## Troubleshooting

### High Interrupt Latency
If interrupt latency is high:
1. Check interrupt statistics: `sigirq stats`
2. Verify CPU load: `sigtop`
3. Check for interrupt storms
4. Consider enabling IRQ threading
5. Adjust balancing mode

### Uneven Interrupt Distribution
If interrupts are unevenly distributed:
1. Check current affinity: `sigirq affinity <irq>`
2. Manually set affinity: `sigirq set-affinity <irq> <cpu_mask>`
3. Enable automatic balancing: `sigirq balance enable`
4. Adjust balancing interval

### MSI-X Allocation Fails
If MSI-X allocation fails:
1. Check available vectors: `sigirq msix list <device>`
2. Check driver MSI-X support
3. Verify PCI config space
4. Check for vector conflicts

---

**[Performance & Kernel](Category-Performance)** | **[Device Drivers](Device-Drivers)** | **[Hardware Support](Hardware-Support)**
