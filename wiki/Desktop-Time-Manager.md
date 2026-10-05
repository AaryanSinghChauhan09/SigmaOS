# Desktop Time Manager

## Overview

The Desktop Time Manager provides comprehensive time and date management inspired by Linux Mint's time settings and Omarchy's time utilities. It supports timezone management, NTP synchronization, time/date format configuration, and automatic time sync.

## Features

- **Time Format**: 12-hour, 24-hour
- **Date Format**: ISO, US, European, Custom
- **Timezone Management**: Multiple timezones with region and city
- **Timezone Offset**: Offset hours and minutes
- **NTP Servers**: Multiple NTP server configuration
- **Active NTP Server**: Track and switch active NTP server
- **Auto-Sync**: Enable/disable automatic time synchronization
- **Sync Interval**: Configurable sync interval (in minutes)
- **Current Timezone**: Track and switch current timezone
- **Timezone Filtering**: List timezones by region
- **Statistics**: Track timezone count, NTP server count, and sync status

## Components

### TimeFormat

```rust
pub enum TimeFormat {
    TwelveHour,      // 12-hour format (AM/PM)
    TwentyFourHour,  // 24-hour format
}
```

### DateFormat

```rust
pub enum DateFormat {
    ISO,        // ISO 8601 format
    US,         // US format (MM/DD/YYYY)
    European,   // European format (DD/MM/YYYY)
    Custom,     // Custom format
}
```

### DesktopTimezone

Timezone with:
- Timezone ID and name
- Region and city
- Offset hours from UTC
- Offset minutes

### DesktopNTPServer

NTP server with:
- Server ID and name
- Server address
- Active flag

### DesktopTimeManager

Main management interface with:
- Timezone management (add, remove, retrieve)
- NTP server management
- Current timezone tracking
- Time and date format configuration
- Auto-sync configuration
- Sync interval configuration
- Active NTP server tracking
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopTimeManager;

let mut manager = DesktopTimeManager::new();

// Get configuration
println!("Total timezones: {}", manager.get_timezones().len());
println!("Total NTP servers: {}", manager.get_ntp_servers().len());
println!("Current timezone: {:?}", manager.get_current_timezone());
println!("Auto-sync enabled: {}", manager.is_auto_sync_enabled());
```

### Timezone Management

```rust
// Add timezone
let timezone = DesktopTimezone::new(
    "custom".to_string(),
    "Custom Timezone".to_string(),
    "Custom".to_string(),
    "City".to_string(),
    2,
);

let id = manager.add_timezone(timezone);

// Remove timezone
manager.remove_timezone(&id);

// Set current timezone
manager.set_current_timezone(&id);
```

### NTP Server Management

```rust
// Add NTP server
let server = DesktopNTPServer::new(
    "custom".to_string(),
    "Custom NTP".to_string(),
    "custom.ntp.org".to_string(),
);

let id = manager.add_ntp_server(server);

// Remove NTP server
manager.remove_ntp_server(&id);

// Set active NTP server
manager.set_active_ntp_server(&id);

// Get active NTP server
if let Some(active) = manager.get_active_ntp_server() {
    println!("Active NTP: {}", active.name);
}
```

### Time/Date Format

```rust
// Set time format
manager.set_time_format(TimeFormat::TwelveHour);
manager.set_time_format(TimeFormat::TwentyFourHour);

// Set date format
manager.set_date_format(DateFormat::US);
manager.set_date_format(DateFormat::European);
manager.set_date_format(DateFormat::ISO);
```

### Auto-Sync Configuration

```rust
// Enable/disable auto-sync
manager.set_auto_sync(true);
manager.set_auto_sync(false);

// Set sync interval (minutes)
manager.set_sync_interval(120);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total timezones: {}", stats.total_timezones);
println!("Total NTP servers: {}", stats.total_ntp_servers);
println!("Current timezone set: {}", stats.current_timezone_set);
println!("Active NTP server set: {}", stats.active_ntp_server_set);
println!("Auto-sync enabled: {}", stats.auto_sync_enabled);
```

## Default Timezones

The manager includes default timezones:

- **UTC**: UTC (offset 0)
- **America/New_York**: New York (offset -5)
- **Europe/London**: London (offset 0)
- **Asia/Tokyo**: Tokyo (offset 9)

## Default NTP Servers

The manager includes default NTP servers:

- **pool.ntp.org**: Active by default
- **ntp.org**
- **time.nist.gov**

## Default Configuration

The Time Manager includes default configuration:

- **Current Timezone**: UTC
- **Time Format**: 24-hour
- **Date Format**: ISO
- **Auto-Sync**: Enabled
- **Sync Interval**: 60 minutes
- **Active NTP Server**: pool.ntp.org

## AI Agent Maintenance Instructions

When maintaining the Time Manager:

1. **systemd-timesyncd Integration**: Integrate with systemd-timesyncd or equivalent
2. **NTP Client**: Implement actual NTP client for time synchronization
3. **RTC Management**: Add RTC (Real-Time Clock) management
3. **DST Support**: Add Daylight Saving Time support
4. **Leap Seconds**: Add leap second handling
5. **Timezone Database**: Use IANA timezone database
6. **Manual Time**: Add manual time setting
7. **Sync Status**: Display sync status and last sync time
8. **Multiple NTP**: Support multiple NTP servers with fallback
9. **Network Time**: Add network time detection
10. **Time Drift**: Add time drift detection and correction

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::time_manager
```

## Future Enhancements

- systemd-timesyncd integration
- Actual NTP client implementation
- RTC (Real-Time Clock) management
- Daylight Saving Time support
- Leap second handling
- IANA timezone database integration
- Manual time setting
- Sync status display
- Multiple NTP servers with fallback
- Network time detection
- Time drift detection and correction
