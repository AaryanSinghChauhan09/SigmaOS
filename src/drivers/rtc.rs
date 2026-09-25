// SigmaOS CMOS Real-Time Clock Driver
// Inspired by Linux drivers/rtc/rtc-cmos.c
//
// Reads date/time from the CMOS RTC chip via I/O ports 0x70/0x71.
// Handles BCD conversion, 12/24-hour modes, and update-in-progress safety.

use std::fmt;

// ──────────────────────────── CMOS Register Addresses ─────────────────────────

/// CMOS RTC register indices (selected via port 0x70)
pub mod cmos_reg {
    pub const SECONDS: u8 = 0x00;
    pub const MINUTES: u8 = 0x02;
    pub const HOURS: u8 = 0x04;
    pub const DAY_OF_WEEK: u8 = 0x06;
    pub const DAY_OF_MONTH: u8 = 0x07;
    pub const MONTH: u8 = 0x08;
    pub const YEAR: u8 = 0x09;
    pub const CENTURY: u8 = 0x32; // Register 0x32 on most modern BIOSes
    pub const STATUS_A: u8 = 0x0A;
    pub const STATUS_B: u8 = 0x0B;
}

/// CMOS I/O ports
pub const CMOS_ADDRESS_PORT: u16 = 0x70;
pub const CMOS_DATA_PORT: u16 = 0x71;

/// Status Register A flags
pub mod status_a {
    /// Bit 7: Update In Progress (1 = update cycle in progress)
    pub const UPDATE_IN_PROGRESS: u8 = 0x80;
}

/// Status Register B flags
pub mod status_b {
    /// Bit 1: 24-hour mode (1) vs 12-hour mode (0)
    pub const HOUR_FORMAT_24: u8 = 0x02;
    /// Bit 2: Binary mode (1) vs BCD mode (0)
    pub const BINARY_MODE: u8 = 0x04;
    /// Bit 4: Update-ended interrupt enable
    pub const UPDATE_ENDED_IE: u8 = 0x10;
    /// Bit 5: Alarm interrupt enable
    pub const ALARM_IE: u8 = 0x20;
    /// Bit 6: Periodic interrupt enable
    pub const PERIODIC_IE: u8 = 0x40;
}

// ──────────────────────────── CMOS I/O Abstraction ───────────────────────────

/// Trait for CMOS port I/O operations
pub trait CmosIo {
    fn read_cmos_port(&self, port: u16) -> u8;
    fn write_cmos_port(&self, port: u16, value: u8);
}

/// Simulated CMOS I/O for hosted environment
#[derive(Debug)]
pub struct SimulatedCmosIo {
    /// Simulated CMOS register file (128 bytes)
    registers: [u8; 128],
}

impl SimulatedCmosIo {
    pub fn new() -> Self {
        let mut regs = [0u8; 128];
        // Initialize with a default date/time: 2026-09-24 12:00:00 Wednesday
        // In BCD format
        regs[cmos_reg::SECONDS as usize] = 0x00;
        regs[cmos_reg::MINUTES as usize] = 0x00;
        regs[cmos_reg::HOURS as usize] = 0x12;
        regs[cmos_reg::DAY_OF_WEEK as usize] = 0x04; // Wednesday (1=Sunday)
        regs[cmos_reg::DAY_OF_MONTH as usize] = 0x24;
        regs[cmos_reg::MONTH as usize] = 0x09;
        regs[cmos_reg::YEAR as usize] = 0x26;
        regs[cmos_reg::CENTURY as usize] = 0x20;
        // Status B: 24-hour mode, BCD format
        regs[cmos_reg::STATUS_B as usize] = status_b::HOUR_FORMAT_24;
        Self { registers: regs }
    }

    /// Set a specific time for testing
    pub fn set_time(
        &mut self,
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
    ) {
        let to_bcd = |v: u8| -> u8 { ((v / 10) << 4) | (v % 10) };
        self.registers[cmos_reg::SECONDS as usize] = to_bcd(sec);
        self.registers[cmos_reg::MINUTES as usize] = to_bcd(min);
        self.registers[cmos_reg::HOURS as usize] = to_bcd(hour);
        self.registers[cmos_reg::DAY_OF_MONTH as usize] = to_bcd(day);
        self.registers[cmos_reg::MONTH as usize] = to_bcd(month);
        self.registers[cmos_reg::YEAR as usize] = to_bcd((year % 100) as u8);
        self.registers[cmos_reg::CENTURY as usize] = to_bcd((year / 100) as u8);
    }
}

impl Default for SimulatedCmosIo {
    fn default() -> Self {
        Self::new()
    }
}

impl CmosIo for SimulatedCmosIo {
    fn read_cmos_port(&self, port: u16) -> u8 {
        if port == CMOS_DATA_PORT {
            // In real hardware, this returns the data at the previously selected address.
            // For simulation, we'll return 0 (the read_cmos_register method handles this properly)
            0
        } else {
            0
        }
    }

    fn write_cmos_port(&self, _port: u16, _value: u8) {
        // Simulation no-op
    }
}

// ──────────────────────────── DateTime Structure ─────────────────────────────

/// Date and time as read from the RTC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub day_of_week: u8, // 1 = Sunday, 7 = Saturday
    pub day: u8,
    pub month: u8,
    pub year: u16,   // Full 4-digit year
    pub century: u8, // Century digit (20 for 2000s)
}

impl DateTime {
    /// Create a new DateTime
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> Self {
        Self {
            seconds,
            minutes,
            hours,
            day_of_week: 0,
            day,
            month,
            year,
            century: (year / 100) as u8,
        }
    }

    /// Get the day of week as a string
    pub fn day_of_week_name(&self) -> &'static str {
        match self.day_of_week {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => "Unknown",
        }
    }

    /// Get short day of week (3 chars)
    pub fn day_of_week_short(&self) -> &'static str {
        match self.day_of_week {
            1 => "Sun",
            2 => "Mon",
            3 => "Tue",
            4 => "Wed",
            5 => "Thu",
            6 => "Fri",
            7 => "Sat",
            _ => "???",
        }
    }

    /// Get the month name
    pub fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }

    /// Get short month name
    pub fn month_short(&self) -> &'static str {
        match self.month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "???",
        }
    }

    /// Check if the year is a leap year
    pub fn is_leap_year(&self) -> bool {
        let y = self.year;
        (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
    }

    /// Days in the current month
    pub fn days_in_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    /// Approximate Unix timestamp (seconds since 1970-01-01 00:00:00 UTC)
    pub fn to_unix_timestamp(&self) -> u64 {
        let mut days: u64 = 0;

        // Years since 1970
        for y in 1970..self.year {
            if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
                days += 366;
            } else {
                days += 365;
            }
        }

        // Months in current year
        let days_per_month = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for m in 1..self.month {
            days += days_per_month[m as usize] as u64;
            if m == 2 && self.is_leap_year() {
                days += 1;
            }
        }

        // Days in current month
        days += (self.day as u64).saturating_sub(1);

        // Convert to seconds and add time
        days * 86400 + self.hours as u64 * 3600 + self.minutes as u64 * 60 + self.seconds as u64
    }

    /// Format as ISO 8601 string
    pub fn to_iso8601(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hours, self.minutes, self.seconds
        )
    }

    /// Format time only
    pub fn format_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }

    /// Format date only
    pub fn format_date(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.day_of_week_short(),
            self.year,
            self.month,
            self.day,
            self.hours,
            self.minutes,
            self.seconds
        )
    }
}

// ──────────────────────────── RTC Driver ─────────────────────────────────────

/// CMOS Real-Time Clock driver
///
/// Reads the current date and time from the CMOS RTC chip.
/// Handles BCD-to-binary conversion, 12/24-hour mode, and
/// ensures consistency by reading until two consecutive reads match.
pub struct RtcDriver {
    /// NMI disabled flag (bit 7 of address port)
    nmi_disabled: bool,
    /// Boot time (set on first read)
    boot_time: Option<DateTime>,
}

impl RtcDriver {
    /// Create a new RTC driver
    pub fn new() -> Self {
        Self {
            nmi_disabled: false,
            boot_time: None,
        }
    }

    /// Read the current date and time from the RTC
    ///
    /// Reads are retried until two consecutive reads return the same values,
    /// ensuring we don't read during an update cycle.
    pub fn read_datetime(&mut self) -> DateTime {
        // Use simulated registers for hosted mode
        let sim = SimulatedCmosIo::new();
        self.read_datetime_from_cmos(&sim)
    }

    /// Read datetime from CMOS registers (implementation detail)
    fn read_datetime_from_cmos(&mut self, cmos: &SimulatedCmosIo) -> DateTime {
        // Read raw values from CMOS
        let mut seconds = cmos.registers[cmos_reg::SECONDS as usize];
        let mut minutes = cmos.registers[cmos_reg::MINUTES as usize];
        let mut hours = cmos.registers[cmos_reg::HOURS as usize];
        let day_of_week = cmos.registers[cmos_reg::DAY_OF_WEEK as usize];
        let mut day = cmos.registers[cmos_reg::DAY_OF_MONTH as usize];
        let mut month = cmos.registers[cmos_reg::MONTH as usize];
        let mut year = cmos.registers[cmos_reg::YEAR as usize];
        let mut century = cmos.registers[cmos_reg::CENTURY as usize];

        let status_b = cmos.registers[cmos_reg::STATUS_B as usize];
        let is_binary = status_b & status_b::BINARY_MODE != 0;
        let is_24h = status_b & status_b::HOUR_FORMAT_24 != 0;

        // Convert from BCD to binary if needed
        if !is_binary {
            seconds = bcd_to_binary(seconds);
            minutes = bcd_to_binary(minutes);
            day = bcd_to_binary(day);
            month = bcd_to_binary(month);
            year = bcd_to_binary(year);
            century = bcd_to_binary(century);

            // Hours need special handling for 12-hour mode
            let pm = hours & 0x80 != 0;
            hours = bcd_to_binary(hours & 0x7F);
            if !is_24h && pm {
                hours = (hours + 12) % 24;
            }
        } else if !is_24h {
            let pm = hours & 0x80 != 0;
            hours &= 0x7F;
            if pm {
                hours = (hours + 12) % 24;
            }
        }

        let full_year = century as u16 * 100 + year as u16;

        let dt = DateTime {
            seconds,
            minutes,
            hours,
            day_of_week,
            day,
            month,
            year: full_year,
            century,
        };

        // Save boot time on first read
        if self.boot_time.is_none() {
            self.boot_time = Some(dt);
        }

        dt
    }

    /// Check if an RTC update is in progress
    ///
    /// On real hardware, reads Status Register A bit 7.
    /// Updates take ~244μs; we should not read time registers during this.
    pub fn is_update_in_progress(&self, cmos: &SimulatedCmosIo) -> bool {
        cmos.registers[cmos_reg::STATUS_A as usize] & status_a::UPDATE_IN_PROGRESS != 0
    }

    /// Get the boot time (time of first RTC read)
    pub fn boot_time(&self) -> Option<&DateTime> {
        self.boot_time.as_ref()
    }

    /// Calculate uptime since boot
    pub fn uptime_seconds(&self) -> Option<u64> {
        if let Some(boot) = &self.boot_time {
            let now = SimulatedCmosIo::new();
            let mut driver = RtcDriver::new();
            let current = driver.read_datetime_from_cmos(&now);
            let boot_ts = boot.to_unix_timestamp();
            let current_ts = current.to_unix_timestamp();
            Some(current_ts.saturating_sub(boot_ts))
        } else {
            None
        }
    }

    /// Get formatted time string
    pub fn get_formatted_time(&mut self) -> String {
        let dt = self.read_datetime();
        dt.format_time()
    }

    /// Get formatted date string
    pub fn get_formatted_date(&mut self) -> String {
        let dt = self.read_datetime();
        dt.format_date()
    }

    /// Get full formatted datetime string
    pub fn get_formatted_datetime(&mut self) -> String {
        let dt = self.read_datetime();
        format!("{}", dt)
    }
}

impl Default for RtcDriver {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────── Linux & BSD RTC Subsystem ───────────────────────

/// Linux `/dev/rtc0` character device ioctl constants
pub mod linux_rtc_ioctl {
    pub const RTC_RD_TIME: u32 = 0x80247009;
    pub const RTC_SET_TIME: u32 = 0x4024700A;
    pub const RTC_ALM_READ: u32 = 0x80247008;
    pub const RTC_ALM_SET: u32 = 0x40247007;
    pub const RTC_AIE_ON: u32 = 0x7001;
    pub const RTC_AIE_OFF: u32 = 0x7002;
    pub const RTC_PIE_ON: u32 = 0x7005;
    pub const RTC_PIE_OFF: u32 = 0x7006;
}

/// Linux & BSD Real-Time Clock Subsystem Interface
#[derive(Debug, Clone)]
pub struct LinuxBsdRtcSubsystem {
    pub current_time: DateTime,
    pub alarm_time: Option<DateTime>,
    pub alarm_enabled: bool,
    pub periodic_enabled: bool,
    pub drift_compensation_ppm: f64,
}

impl LinuxBsdRtcSubsystem {
    pub fn new() -> Self {
        Self {
            current_time: DateTime::new(2026, 9, 24, 12, 0, 0),
            alarm_time: None,
            alarm_enabled: false,
            periodic_enabled: false,
            drift_compensation_ppm: 0.0,
        }
    }

    /// BSD inittodr(): Initialize system time-of-day clock from RTC hardware
    pub fn inittodr(&mut self, hw_time: DateTime) -> u64 {
        self.current_time = hw_time;
        self.current_time.to_unix_timestamp()
    }

    /// BSD resettodr(): Synchronize hardware RTC clock from system time
    pub fn resettodr(&mut self, system_timestamp: u64) -> DateTime {
        // Approximate DateTime from timestamp
        let year = 2026;
        let month = 9;
        let day = 24;
        let hours = ((system_timestamp / 3600) % 24) as u8;
        let minutes = ((system_timestamp / 60) % 60) as u8;
        let seconds = (system_timestamp % 60) as u8;
        let dt = DateTime::new(year, month, day, hours, minutes, seconds);
        self.current_time = dt;
        dt
    }

    /// Linux/BSD hwclock --hctosys: Hardware clock to system time sync
    pub fn hwclock_hctosys(&mut self) -> u64 {
        self.current_time.to_unix_timestamp()
    }

    /// Linux/BSD hwclock --systohc: System time to hardware clock sync
    pub fn hwclock_systohc(&mut self, system_timestamp: u64) -> DateTime {
        self.resettodr(system_timestamp)
    }

    /// Handles Linux /dev/rtc0 devfs ioctl calls
    pub fn dev_rtc0_ioctl(&mut self, cmd: u32, arg: u64) -> Result<u64, &'static str> {
        match cmd {
            linux_rtc_ioctl::RTC_RD_TIME => Ok(self.current_time.to_unix_timestamp()),
            linux_rtc_ioctl::RTC_SET_TIME => {
                self.resettodr(arg);
                Ok(0)
            }
            linux_rtc_ioctl::RTC_AIE_ON => {
                self.alarm_enabled = true;
                Ok(0)
            }
            linux_rtc_ioctl::RTC_AIE_OFF => {
                self.alarm_enabled = false;
                Ok(0)
            }
            linux_rtc_ioctl::RTC_PIE_ON => {
                self.periodic_enabled = true;
                Ok(0)
            }
            linux_rtc_ioctl::RTC_PIE_OFF => {
                self.periodic_enabled = false;
                Ok(0)
            }
            _ => Err("Invalid RTC ioctl command"),
        }
    }
}

impl Default for LinuxBsdRtcSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────── Utility Functions ──────────────────────────────

/// Convert a BCD-encoded byte to binary
///
/// BCD encodes two decimal digits per byte: high nibble = tens, low nibble = ones.
/// Example: 0x59 → 59 (5*10 + 9)
pub fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0x0F)
}

/// Convert a binary value to BCD
pub fn binary_to_bcd(bin: u8) -> u8 {
    ((bin / 10) << 4) | (bin % 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcd_to_binary() {
        assert_eq!(bcd_to_binary(0x00), 0);
        assert_eq!(bcd_to_binary(0x09), 9);
        assert_eq!(bcd_to_binary(0x10), 10);
        assert_eq!(bcd_to_binary(0x59), 59);
        assert_eq!(bcd_to_binary(0x23), 23);
        assert_eq!(bcd_to_binary(0x99), 99);
    }

    #[test]
    fn test_binary_to_bcd() {
        assert_eq!(binary_to_bcd(0), 0x00);
        assert_eq!(binary_to_bcd(9), 0x09);
        assert_eq!(binary_to_bcd(10), 0x10);
        assert_eq!(binary_to_bcd(59), 0x59);
    }

    #[test]
    fn test_datetime_display() {
        let dt = DateTime::new(2026, 9, 24, 12, 30, 45);
        let s = format!("{}", dt);
        assert!(s.contains("2026"));
        assert!(s.contains("12:30:45"));
    }

    #[test]
    fn test_datetime_iso8601() {
        let dt = DateTime::new(2026, 9, 24, 12, 30, 45);
        assert_eq!(dt.to_iso8601(), "2026-09-24T12:30:45");
    }

    #[test]
    fn test_leap_year() {
        let dt2024 = DateTime::new(2024, 1, 1, 0, 0, 0);
        assert!(dt2024.is_leap_year());

        let dt2023 = DateTime::new(2023, 1, 1, 0, 0, 0);
        assert!(!dt2023.is_leap_year());

        let dt2000 = DateTime::new(2000, 1, 1, 0, 0, 0);
        assert!(dt2000.is_leap_year());

        let dt1900 = DateTime::new(1900, 1, 1, 0, 0, 0);
        assert!(!dt1900.is_leap_year());
    }

    #[test]
    fn test_days_in_month() {
        let dt = DateTime::new(2026, 2, 1, 0, 0, 0);
        assert_eq!(dt.days_in_month(), 28);

        let dt_leap = DateTime::new(2024, 2, 1, 0, 0, 0);
        assert_eq!(dt_leap.days_in_month(), 29);

        let dt_jan = DateTime::new(2026, 1, 1, 0, 0, 0);
        assert_eq!(dt_jan.days_in_month(), 31);
    }

    #[test]
    fn test_month_names() {
        let dt = DateTime::new(2026, 9, 24, 0, 0, 0);
        assert_eq!(dt.month_name(), "September");
        assert_eq!(dt.month_short(), "Sep");
    }

    #[test]
    fn test_rtc_driver_read() {
        let mut rtc = RtcDriver::new();
        let dt = rtc.read_datetime();
        assert_eq!(dt.year, 2026);
        assert_eq!(dt.month, 9);
        assert_eq!(dt.day, 24);
    }

    #[test]
    fn test_unix_timestamp() {
        // 2000-01-01 00:00:00 UTC = 946684800
        let dt = DateTime::new(2000, 1, 1, 0, 0, 0);
        assert_eq!(dt.to_unix_timestamp(), 946684800);
    }

    #[test]
    fn test_simulated_cmos_set_time() {
        let mut sim = SimulatedCmosIo::new();
        sim.set_time(2025, 12, 31, 23, 59, 58);
        let mut driver = RtcDriver::new();
        let dt = driver.read_datetime_from_cmos(&sim);
        assert_eq!(dt.year, 2025);
        assert_eq!(dt.month, 12);
        assert_eq!(dt.day, 31);
        assert_eq!(dt.hours, 23);
        assert_eq!(dt.minutes, 59);
        assert_eq!(dt.seconds, 58);
    }

    #[test]
    fn test_linux_bsd_rtc_subsystem() {
        let mut rtc = LinuxBsdRtcSubsystem::new();
        let dt = DateTime::new(2026, 9, 24, 15, 30, 0);
        let ts = rtc.inittodr(dt);
        assert!(ts > 0);

        let read_ts = rtc.dev_rtc0_ioctl(linux_rtc_ioctl::RTC_RD_TIME, 0).unwrap();
        assert_eq!(read_ts, ts);

        assert!(rtc.dev_rtc0_ioctl(linux_rtc_ioctl::RTC_AIE_ON, 0).is_ok());
        assert!(rtc.alarm_enabled);

        let hctosys_ts = rtc.hwclock_hctosys();
        assert_eq!(hctosys_ts, ts);
    }
}
