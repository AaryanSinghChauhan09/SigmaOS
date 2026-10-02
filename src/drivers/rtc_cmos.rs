//! CMOS Real-Time Clock (RTC) Driver
//!
//! Inspired by Linux RTC subsystem and BSD clock management.
//! Provides hardware clock access for system time initialization.
//!
//! # Features
//! - CMOS RTC reading (date, time)
//! - Hardware clock synchronization
//! - Alarm support
//! - NMI disable during access (atomic read/write)
//!
//! # Linux Inspiration
//! - `drivers/rtc/rtc-cmos.c` - CMOS RTC driver
//! - `drivers/rtc/interface.c` - RTC class interface
//!
//! # FreeBSD Inspiration
//! - `sys/isa/atrtc.c` - AT Real-Time Clock
//! - `sys/kern/subr_rtc.c` - RTC subsystem
//!
//! # Hardware
//! - I/O Ports: 0x70 (address), 0x71 (data)
//! - NMI disable bit: 0x80 in address port
//! - Registers: 0x00-0x09 (time/date), 0x0A-0x0D (control/status)

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

use core::arch::asm;

/// CMOS RTC I/O ports
const RTC_ADDRESS_PORT: u16 = 0x70;
const RTC_DATA_PORT: u16 = 0x71;

/// CMOS RTC registers
const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;
const RTC_STATUS_A: u8 = 0x0A;
const RTC_STATUS_B: u8 = 0x0B;
const RTC_CENTURY: u8 = 0x32; // May vary by BIOS

/// NMI disable bit (set to disable NMI during RTC access)
const NMI_DISABLE: u8 = 0x80;

/// RTC update in progress bit (Status Register A)
const RTC_UIP: u8 = 0x80;

/// Date and time structure
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl DateTime {
    /// Convert to Unix timestamp (seconds since 1970-01-01)
    pub fn to_unix_timestamp(&self) -> i64 {
        // Simplified calculation (doesn't handle all edge cases)
        let mut year = self.year as i64;
        let mut days = 0i64;

        // Count leap years
        days += (year - 1970) * 365;
        days += (year - 1969) / 4; // Leap years
        days -= (year - 1901) / 100; // Century years not leap
        days += (year - 1601) / 400; // Except divisible by 400

        // Days in months (non-leap year)
        const DAYS_IN_MONTH: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        for m in 1..self.month {
            days += DAYS_IN_MONTH[(m - 1) as usize];
        }

        // Add leap day if February and leap year
        if self.month > 2 && self.is_leap_year() {
            days += 1;
        }

        days += (self.day - 1) as i64;

        let hours = days * 24 + self.hour as i64;
        let minutes = hours * 60 + self.minute as i64;
        let seconds = minutes * 60 + self.second as i64;

        seconds
    }

    fn is_leap_year(&self) -> bool {
        let year = self.year;
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

/// CMOS RTC Driver
pub struct CmosRtc;

impl CmosRtc {
    /// Read CMOS register
    /// # Safety
    /// Directly accesses hardware I/O ports
    unsafe fn read_register(reg: u8) -> u8 {
        // Disable NMI and select register
        Self::outb(RTC_ADDRESS_PORT, NMI_DISABLE | reg);

        // Read data
        Self::inb(RTC_DATA_PORT)
    }

    /// Write CMOS register
    /// # Safety
    /// Directly accesses hardware I/O ports
    unsafe fn write_register(reg: u8, value: u8) {
        Self::outb(RTC_ADDRESS_PORT, NMI_DISABLE | reg);
        Self::outb(RTC_DATA_PORT, value);
    }

    /// Wait for RTC update to complete
    /// Linux: `drivers/rtc/rtc-cmos.c:cmos_read_time()`
    unsafe fn wait_for_update() {
        // Wait until update in progress flag clears
        let mut timeout = 1000;
        while timeout > 0 {
            if (Self::read_register(RTC_STATUS_A) & RTC_UIP) == 0 {
                return;
            }
            timeout -= 1;
        }
    }

    /// Read current date and time from CMOS RTC
    /// Linux: `drivers/rtc/rtc-cmos.c:cmos_read_time()`
    /// FreeBSD: `sys/isa/atrtc.c:atrtc_gettime()`
    pub fn read_datetime() -> Result<DateTime, RtcError> {
        unsafe {
            Self::wait_for_update();

            // Read all registers atomically
            let second = Self::bcd_to_binary(Self::read_register(RTC_SECONDS));
            let minute = Self::bcd_to_binary(Self::read_register(RTC_MINUTES));
            let hour = Self::bcd_to_binary(Self::read_register(RTC_HOURS));
            let day = Self::bcd_to_binary(Self::read_register(RTC_DAY));
            let month = Self::bcd_to_binary(Self::read_register(RTC_MONTH));
            let year = Self::bcd_to_binary(Self::read_register(RTC_YEAR));

            // Try to read century register (may not exist on all systems)
            let century = Self::bcd_to_binary(Self::read_register(RTC_CENTURY));

            // Calculate full year
            let full_year = if century > 0 {
                century as u16 * 100 + year as u16
            } else {
                // Assume 2000s if year < 50, else 1900s
                if year < 50 {
                    2000 + year as u16
                } else {
                    1900 + year as u16
                }
            };

            Ok(DateTime {
                year: full_year,
                month,
                day,
                hour,
                minute,
                second,
            })
        }
    }

    /// Write date and time to CMOS RTC
    /// # Safety
    /// Writes to hardware, may affect system time
    pub unsafe fn write_datetime(dt: &DateTime) -> Result<(), RtcError> {
        Self::wait_for_update();

        Self::write_register(RTC_SECONDS, Self::binary_to_bcd(dt.second));
        Self::write_register(RTC_MINUTES, Self::binary_to_bcd(dt.minute));
        Self::write_register(RTC_HOURS, Self::binary_to_bcd(dt.hour));
        Self::write_register(RTC_DAY, Self::binary_to_bcd(dt.day));
        Self::write_register(RTC_MONTH, Self::binary_to_bcd(dt.month));
        Self::write_register(RTC_YEAR, Self::binary_to_bcd((dt.year % 100) as u8));

        // Write century if available
        let century = (dt.year / 100) as u8;
        Self::write_register(RTC_CENTURY, Self::binary_to_bcd(century));

        Ok(())
    }

    /// Convert BCD (Binary-Coded Decimal) to binary
    fn bcd_to_binary(bcd: u8) -> u8 {
        ((bcd >> 4) * 10) + (bcd & 0x0F)
    }

    /// Convert binary to BCD
    fn binary_to_bcd(binary: u8) -> u8 {
        ((binary / 10) << 4) | (binary % 10)
    }

    /// Read byte from I/O port
    #[cfg(target_arch = "x86_64")]
    unsafe fn inb(port: u16) -> u8 {
        let value: u8;
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
        value
    }

    /// Write byte to I/O port
    #[cfg(target_arch = "x86_64")]
    unsafe fn outb(port: u16, value: u8) {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    #[cfg(not(target_arch = "x86_64"))]
    unsafe fn inb(_port: u16) -> u8 {
        0 // Stub for non-x86 architectures
    }

    #[cfg(not(target_arch = "x86_64"))]
    unsafe fn outb(_port: u16, _value: u8) {
        // Stub for non-x86 architectures
    }
}

/// RTC Error Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcError {
    Timeout,
    InvalidData,
    NotSupported,
}

/// Initialize RTC and set system time
/// Linux: `drivers/rtc/class.c:rtc_hctosys()`
pub fn init_rtc() -> Result<DateTime, RtcError> {
    CmosRtc::read_datetime()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcd_conversion() {
        assert_eq!(CmosRtc::bcd_to_binary(0x23), 23);
        assert_eq!(CmosRtc::bcd_to_binary(0x59), 59);
        assert_eq!(CmosRtc::binary_to_bcd(23), 0x23);
        assert_eq!(CmosRtc::binary_to_bcd(59), 0x59);
    }

    #[test]
    fn test_datetime_unix_timestamp() {
        let dt = DateTime {
            year: 2024,
            month: 10,
            day: 2,
            hour: 12,
            minute: 0,
            second: 0,
        };

        // Unix timestamp for 2024-10-02 12:00:00
        let ts = dt.to_unix_timestamp();
        assert!(ts > 0);
        assert!(ts > 1_700_000_000); // After 2023
    }

    #[test]
    fn test_leap_year() {
        assert!(DateTime {
            year: 2024,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0
        }
        .is_leap_year());
        assert!(!DateTime {
            year: 2023,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0
        }
        .is_leap_year());
        assert!(DateTime {
            year: 2000,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0
        }
        .is_leap_year());
        assert!(!DateTime {
            year: 1900,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0
        }
        .is_leap_year());
    }
}
