// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/power/sigma_power.zig — Power Management (ACPI P/C-states)
// Language: Zig — direct MMIO/port I/O, comptime state tables
// Pattern: struct with methods (OOP equivalent)

// ── CPU Power States ──────────────────────────────────────────────────────────

pub const CpuProfile = enum(u8) {
    UltraEco    = 0,  // max power saving, aggressive C-states
    Balanced    = 1,  // default
    Performance = 2,  // max freq, C0 only
    Realtime    = 3,  // no C-states, IRQ latency target < 10µs
};

// ── P-State (frequency / voltage) ────────────────────────────────────────────

pub const PState = struct {
    freq_mhz:     u32,
    voltage_mv:   u32,
    power_mw:     u32,
};

/// Intel x86-64 P-state table (representative — real values from ACPI _PSS)
pub const P_STATES: [8]PState = .{
    .{ .freq_mhz = 3600, .voltage_mv = 1200, .power_mw = 65000 }, // P0
    .{ .freq_mhz = 3200, .voltage_mv = 1100, .power_mw = 45000 }, // P1
    .{ .freq_mhz = 2800, .voltage_mv = 1050, .power_mw = 35000 }, // P2
    .{ .freq_mhz = 2400, .voltage_mv = 1000, .power_mw = 25000 }, // P3
    .{ .freq_mhz = 2000, .voltage_mv = 950,  .power_mw = 18000 }, // P4
    .{ .freq_mhz = 1600, .voltage_mv = 900,  .power_mw = 12000 }, // P5
    .{ .freq_mhz = 1200, .voltage_mv = 850,  .power_mw =  8000 }, // P6
    .{ .freq_mhz =  800, .voltage_mv = 800,  .power_mw =  4000 }, // P7
};

// ── MSR Addresses ─────────────────────────────────────────────────────────────

const MSR_IA32_PERF_CTL:     u32 = 0x199;
const MSR_IA32_PERF_STATUS:  u32 = 0x198;
const MSR_IA32_PM_ENABLE:    u32 = 0x770;
const MSR_IA32_HWP_REQUEST:  u32 = 0x774;
const MSR_IA32_ENERGY_PERF:  u32 = 0x1B0;

// ── MSR I/O ───────────────────────────────────────────────────────────────────

fn rdmsr(msr: u32) u64 {
    var lo: u32 = undefined;
    var hi: u32 = undefined;
    asm volatile ("rdmsr"
        : [lo] "={eax}" (lo), [hi] "={edx}" (hi)
        : [msr] "{ecx}" (msr)
        : "memory"
    );
    return @as(u64, hi) << 32 | lo;
}

fn wrmsr(msr: u32, val: u64) void {
    asm volatile ("wrmsr"
        : // no outputs
        : [lo]  "{eax}" (@as(u32, @truncate(val))),
          [hi]  "{edx}" (@as(u32, @truncate(val >> 32))),
          [msr] "{ecx}" (msr)
        : "memory"
    );
}

// ── C-State Entry ─────────────────────────────────────────────────────────────

fn enter_c1() void {
    asm volatile ("hlt" ::: "memory");
}

fn enter_c2(port: u16) void {
    // Read FADT-specified P_LVL2 port to enter C2
    _ = asm volatile ("inb %[port], %[ret]"
        : [ret] "=a" (-> u8)
        : [port] "Nd" (port)
        : "memory"
    );
}

// ── Power Governor ────────────────────────────────────────────────────────────

pub const PowerGovernor = struct {
    profile:    CpuProfile,
    p_state:    u8,        // current P-state index (0 = highest freq)
    cpu_load:   u8,        // 0–100 percent
    battery_pct: u8,       // 0–100
    on_battery:  bool,
    hwp_supported: bool,

    pub fn init() PowerGovernor {
        // Check HWP support (CPUID leaf 6)
        const has_hwp = blk: {
            var eax: u32 = undefined;
            asm volatile ("cpuid"
                : [eax] "={eax}" (eax)
                : [leaf] "{eax}" (@as(u32, 6))
                : "ebx", "ecx", "edx"
            );
            break :blk (eax & (1 << 7)) != 0;
        };
        return .{
            .profile    = CpuProfile.Balanced,
            .p_state    = 3,
            .cpu_load   = 0,
            .battery_pct = 100,
            .on_battery  = false,
            .hwp_supported = has_hwp,
        };
    }

    /// Set CPU power profile
    pub fn set_profile(self: *PowerGovernor, profile: CpuProfile) void {
        self.profile = profile;
        const target: u8 = switch (profile) {
            .UltraEco    => 7,
            .Balanced    => 3,
            .Performance => 0,
            .Realtime    => 0,
        };
        self.set_p_state(target);
    }

    /// Set P-state via IA32_PERF_CTL MSR
    pub fn set_p_state(self: *PowerGovernor, idx: u8) void {
        const i: usize = @min(idx, P_STATES.len - 1);
        self.p_state = @intCast(i);
        // IA32_PERF_CTL: bits 15:8 = target IDA ratio
        const ratio: u64 = P_STATES[i].freq_mhz / 100;
        wrmsr(MSR_IA32_PERF_CTL, ratio << 8);
    }

    /// Enable HWP (Hardware-managed P-states) if supported
    pub fn enable_hwp(self: *PowerGovernor) void {
        if (!self.hwp_supported) return;
        wrmsr(MSR_IA32_PM_ENABLE, 1); // enable HWP
        // Set HWP request: min=lowest, max=highest, desired=0 (auto)
        const hwp_req: u64 = 0x0080_0000_0000_0000; // desired = auto
        wrmsr(MSR_IA32_HWP_REQUEST, hwp_req);
    }

    /// Called from scheduler idle loop — enter lowest safe C-state
    pub fn idle(self: *const PowerGovernor) void {
        switch (self.profile) {
            .Realtime    => {}, // spin-wait, no C-states
            .Performance => enter_c1(),
            .Balanced    => enter_c1(),
            .UltraEco    => enter_c2(0x414), // port from ACPI FADT
        }
    }

    /// Update based on CPU load measurement
    pub fn update_load(self: *PowerGovernor, load_pct: u8) void {
        self.cpu_load = load_pct;
        if (self.profile != CpuProfile.Balanced) return;
        // Simple ondemand-style governor
        const target: u8 = if (load_pct > 80) 1
                      else if (load_pct > 50) 3
                      else if (load_pct > 20) 5
                      else                    7;
        if (target != self.p_state) self.set_p_state(target);
    }

    pub fn current_freq_mhz(self: *const PowerGovernor) u32 {
        return P_STATES[@min(self.p_state, P_STATES.len - 1)].freq_mhz;
    }

    pub fn current_power_mw(self: *const PowerGovernor) u32 {
        return P_STATES[@min(self.p_state, P_STATES.len - 1)].power_mw;
    }
};
