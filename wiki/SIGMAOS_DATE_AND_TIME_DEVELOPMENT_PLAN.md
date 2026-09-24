# ⏱️ SigmaOS System Date, Time, NTP & Timezone Subsystem (`timedate`) Strategic Development Plan

## Executive Summary & Design Vision

System Date & Time management in modern operating systems underpins POSIX file system timestamps (`mtime`, `atime`, `ctime`), cryptographic TLS/SSL certificate validation, process scheduling, distributed consensus, security audit logging, and desktop user clock displays.

Drawing inspiration from Linux time daemons (**`systemd-timedated`**, **Chrony**, **NTPd**), BSD time synchronization tools (**OpenBSD `ntpd` with HTTPS constraint verification**, **FreeBSD `adjkerntz` / `ntpdate`**), and time zone standards (**IANA Time Zone Database `tzdb`**), the **SigmaOS Date & Time Subsystem** (`TimedateDaemon`, `BsdSecureNtpConstraintSync`, `RtcClockSyncEngine`, `TzDatabaseManager`) provides a zero-dependency, secure, high-precision timekeeping architecture in Safe Rust.

---

## 1. Multi-Distro & Multi-OS Date & Time Inspirations

### 1.1 OpenBSD `ntpd` & HTTPS Constraint Verification
- **Inspirations**:
  - **HTTPS Constraint Time Sync**: Mitigating NTP Man-in-the-Middle (MitM) packet spoofing and time-rollback attacks by validating NTP network time samples against TLS authenticated HTTPS server `Date` headers (`https://1.1.1.1` or `https://google.com`).
- **SigmaOS Integration**: `BsdSecureNtpConstraintSync` in `src/distro/bsd_linux_innovations.rs`.

### 1.2 Linux `systemd-timedated` & D-Bus Time Management API
- **Inspirations**:
  - **`timedatectl` D-Bus Control**: D-Bus interface (`org.freedesktop.timedate1`) allowing userland desktop applications, Zenith Control Center, and CLI utilities to inspect and configure system clock, RTC local/UTC mode, NTP synchronization state, and active time zone.
- **SigmaOS Integration**: `TimedateDaemon` D-Bus portal service and `sigma-sh` CLI command router (`timedatectl`).

### 1.3 FreeBSD `adjkerntz`, RTC Hardware Clock & Daylight Saving Time (DST)
- **Inspirations**:
  - **CMOS RTC Local vs. UTC Mode**: FreeBSD `adjkerntz -a` handling CMOS Real-Time Clock (RTC) offset adjustments for dual-boot compatibility (e.g., Windows local time vs Linux/BSD UTC hardware clock).
- **SigmaOS Integration**: `RtcClockSyncEngine` in `src/drivers/` and `/etc/adjtime` configuration.

### 1.4 IANA Time Zone Database (`tzdb`) & Olson Timezone Parsing
- **Inspirations**:
  - **POSIX `TZ` & `/etc/localtime` Symlink**: Symlinking `/etc/localtime` to `/usr/share/zoneinfo/America/New_York` or `/usr/share/zoneinfo/Asia/Kolkata` with POSIX `TZ` environment variable override support.
- **SigmaOS Integration**: `TzDatabaseManager` and timezone symlink resolver in `src/time/`.

---

## 2. Core Architectural Subsystems

```text
┌───────────────────────────────────────────────────────────────────────────┐
│              Zenith Desktop Clock Applet & `timedatectl` CLI              │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│                 `sigma-timedate` System Time Service Daemon               │
│     - D-Bus Portal Interface (`org.freedesktop.timedate1`)                │
│     - Timezone Configurator (`/etc/localtime -> /usr/share/zoneinfo/`)    │
│     - Local vs UTC Hardware Clock Offset Manager (`/etc/adjtime`)         │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│           OpenBSD HTTPS Constraint-Based Secure NTP Client Engine         │
│     - NTP UDP Network Poll (`pool.ntp.org` / `time.cloudflare.com`)       │
│     - HTTPS TLS Date Header Constraint Verification (MitM Guard)          │
│     - Monotonic Clock Slew (`adjtime(2)`) vs Hard Jump Handling           │
└───────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
┌───────────────────────────────────────────────────────────────────────────┐
│             CMOS RTC & x86_64 / ARM64 Hardware Timer Driver               │
│     - CMOS Real-Time Clock Read/Write (`/dev/rtc0`)                       │
│     - TSC (Time Stamp Counter) & ARM Generic Timer High-Res Counters      │
└───────────────────────────────────────────────────────────────────────────┘
```

### 2.1 Clock Accuracy & Slew Rate Target
- Max time drift offset target: **< 1.0 millisecond** during active NTP synchronization.
- System clock adjustment uses gradual frequency slewing (`adjtime`) to prevent backward jump glitches for file systems and database logs.

### 2.2 Security & MitM Protection
- HTTPS constraint verification rejects NTP samples that deviate more than **15 minutes** from authenticated TLS `Date` headers.

---

## 3. Phased Development Roadmap

### Phase 1: Native POSIX Time Calls & CMOS RTC Driver (Q4 2026)
- Standardize `clock_gettime`, `clock_settime`, `adjtime`, and `gettimeofday` syscalls in `src/time/`.
- Implement CMOS RTC hardware clock driver (`/dev/rtc0`) in `src/drivers/`.
- Support UTC vs. Local Time CMOS hardware clock configuration in `/etc/adjtime`.

### Phase 2: OpenBSD Constraint Secure NTP Client (Q1 2027)
- Stabilize `BsdSecureNtpConstraintSync` HTTPS constraint time validation in `src/distro/bsd_linux_innovations.rs`.
- Implement NTP UDP client packet parser and server pool peer selection.
- Deploy automatic clock slew (`adjtime`) vs step jump policy.

### Phase 3: IANA Time Zone Database & `systemd-timedated` D-Bus API (Q2 2027)
- Embed IANA `tzdb` time zone database compiler and `/etc/localtime` symlink router.
- Deploy `sigma-timedate` daemon with `org.freedesktop.timedate1` D-Bus portal parity.
- Build Zenith Desktop clock panel applet with world clock and calendar integration.

### Phase 4: Network Time Security (NTS) & Benchmarks (Q3 2027+)
- Implement Network Time Security (NTS) RFC 8915 extension fields.
- Publish timekeeping precision and NTP sync benchmarks in `scripts/tech_media_benchmark_suite.sh`.
- Conduct security audit on time parsing and NTP UDP packet deserializers (`cargo fuzz`).

---

## 4. Verification & Testing Standards

All date & time components must pass the unified verification runner:
```bash
./scripts/verify.sh
```

Which validates unit and integration tests across:
- `src/distro/bsd_linux_innovations.rs` (`BsdSecureNtpConstraintSync` HTTPS constraint NTP verification)
- `src/time/mod.rs` (POSIX timestamp conversion & timezone symlink resolving)
- `src/drivers/` (CMOS RTC clock read/write operations)
- `src/init/` (`sigma-timedate` time service initialization)
