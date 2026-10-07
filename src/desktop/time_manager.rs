// Desktop Time Manager
// Linux Mint & Omarchy inspiration for comprehensive time and date management

use std::collections::HashMap;

/// Time Format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    TwelveHour,
    TwentyFourHour,
}

impl TimeFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeFormat::TwelveHour => "12h",
            TimeFormat::TwentyFourHour => "24h",
        }
    }
}

/// Date Format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormat {
    ISO,
    US,
    European,
    Custom,
}

impl DateFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            DateFormat::ISO => "iso",
            DateFormat::US => "us",
            DateFormat::European => "european",
            DateFormat::Custom => "custom",
        }
    }
}

/// Desktop Timezone
#[derive(Debug, Clone)]
pub struct DesktopTimezone {
    pub id: String,
    pub name: String,
    pub region: String,
    pub city: String,
    pub offset_hours: i32,
    pub offset_minutes: i32,
}

impl DesktopTimezone {
    pub fn new(id: String, name: String, region: String, city: String, offset_hours: i32) -> Self {
        Self {
            id,
            name,
            region,
            city,
            offset_hours,
            offset_minutes: 0,
        }
    }

    pub fn with_offset_minutes(mut self, minutes: i32) -> Self {
        self.offset_minutes = minutes;
        self
    }
}

/// Desktop NTP Server
#[derive(Debug, Clone)]
pub struct DesktopNTPServer {
    pub id: String,
    pub name: String,
    pub address: String,
    pub active: bool,
}

impl DesktopNTPServer {
    pub fn new(id: String, name: String, address: String) -> Self {
        Self {
            id,
            name,
            address,
            active: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

/// Desktop Time Manager
pub struct DesktopTimeManager {
    timezones: HashMap<String, DesktopTimezone>,
    ntp_servers: HashMap<String, DesktopNTPServer>,
    current_timezone: Option<String>,
    time_format: TimeFormat,
    date_format: DateFormat,
    auto_sync: bool,
    sync_interval_minutes: u32,
    counter: u32,
}

impl DesktopTimeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            timezones: HashMap::new(),
            ntp_servers: HashMap::new(),
            current_timezone: None,
            time_format: TimeFormat::TwentyFourHour,
            date_format: DateFormat::ISO,
            auto_sync: true,
            sync_interval_minutes: 60,
            counter: 1000,
        };

        // Add default timezones
        manager.add_default_timezones();

        // Add default NTP servers
        manager.add_default_ntp_servers();

        // Set default timezone
        manager.current_timezone = Some("timezone_0".to_string());

        manager
    }

    fn add_default_timezones(&mut self) {
        let utc = DesktopTimezone::new(
            "timezone_0".to_string(),
            "UTC".to_string(),
            "UTC".to_string(),
            "".to_string(),
            0,
        );

        let new_york = DesktopTimezone::new(
            "timezone_1".to_string(),
            "America/New_York".to_string(),
            "America".to_string(),
            "New York".to_string(),
            -5,
        );

        let london = DesktopTimezone::new(
            "timezone_2".to_string(),
            "Europe/London".to_string(),
            "Europe".to_string(),
            "London".to_string(),
            0,
        );

        let tokyo = DesktopTimezone::new(
            "timezone_3".to_string(),
            "Asia/Tokyo".to_string(),
            "Asia".to_string(),
            "Tokyo".to_string(),
            9,
        );

        self.timezones.insert(utc.id.clone(), utc);
        self.timezones.insert(new_york.id.clone(), new_york);
        self.timezones.insert(london.id.clone(), london);
        self.timezones.insert(tokyo.id.clone(), tokyo);
    }

    fn add_default_ntp_servers(&mut self) {
        let mut pool_ntp = DesktopNTPServer::new(
            "ntp_0".to_string(),
            "pool.ntp.org".to_string(),
            "pool.ntp.org".to_string(),
        );
        pool_ntp.set_active(true);

        let ntp_org = DesktopNTPServer::new(
            "ntp_1".to_string(),
            "ntp.org".to_string(),
            "ntp.org".to_string(),
        );

        let time_nist_gov = DesktopNTPServer::new(
            "ntp_2".to_string(),
            "time.nist.gov".to_string(),
            "time.nist.gov".to_string(),
        );

        self.ntp_servers.insert(pool_ntp.id.clone(), pool_ntp);
        self.ntp_servers.insert(ntp_org.id.clone(), ntp_org);
        self.ntp_servers.insert(time_nist_gov.id.clone(), time_nist_gov);
    }

    pub fn add_timezone(&mut self, timezone: DesktopTimezone) -> String {
        let id = format!("timezone_{}", self.counter);
        self.counter += 1;

        let timezone = DesktopTimezone {
            id: id.clone(),
            ..timezone
        };

        self.timezones.insert(id.clone(), timezone);
        id
    }

    pub fn remove_timezone(&mut self, id: &str) -> bool {
        if Some(id.to_string()) == self.current_timezone {
            return false;
        }
        self.timezones.remove(id).is_some()
    }

    pub fn get_timezone(&self, id: &str) -> Option<&DesktopTimezone> {
        self.timezones.get(id)
    }

    pub fn get_timezones(&self) -> Vec<&DesktopTimezone> {
        self.timezones.values().collect()
    }

    pub fn set_current_timezone(&mut self, id: &str) -> bool {
        if self.timezones.contains_key(id) {
            self.current_timezone = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_timezone(&self) -> Option<&DesktopTimezone> {
        self.current_timezone
            .as_ref()
            .and_then(|id| self.timezones.get(id))
    }

    pub fn add_ntp_server(&mut self, server: DesktopNTPServer) -> String {
        let id = format!("ntp_{}", self.counter);
        self.counter += 1;

        let server = DesktopNTPServer {
            id: id.clone(),
            ..server
        };

        self.ntp_servers.insert(id.clone(), server);
        id
    }

    pub fn remove_ntp_server(&mut self, id: &str) -> bool {
        self.ntp_servers.remove(id).is_some()
    }

    pub fn get_ntp_server(&self, id: &str) -> Option<&DesktopNTPServer> {
        self.ntp_servers.get(id)
    }

    pub fn get_ntp_servers(&self) -> Vec<&DesktopNTPServer> {
        self.ntp_servers.values().collect()
    }

    pub fn set_active_ntp_server(&mut self, id: &str) -> bool {
        // Deactivate all servers
        for server in self.ntp_servers.values_mut() {
            server.set_active(false);
        }

        // Activate selected server
        if let Some(server) = self.ntp_servers.get_mut(id) {
            server.set_active(true);
            true
        } else {
            false
        }
    }

    pub fn get_active_ntp_server(&self) -> Option<&DesktopNTPServer> {
        self.ntp_servers.values().find(|s| s.active)
    }

    pub fn set_time_format(&mut self, format: TimeFormat) {
        self.time_format = format;
    }

    pub fn get_time_format(&self) -> TimeFormat {
        self.time_format
    }

    pub fn set_date_format(&mut self, format: DateFormat) {
        self.date_format = format;
    }

    pub fn get_date_format(&self) -> DateFormat {
        self.date_format
    }

    pub fn set_auto_sync(&mut self, auto_sync: bool) {
        self.auto_sync = auto_sync;
    }

    pub fn is_auto_sync_enabled(&self) -> bool {
        self.auto_sync
    }

    pub fn set_sync_interval(&mut self, minutes: u32) {
        self.sync_interval_minutes = minutes;
    }

    pub fn get_sync_interval(&self) -> u32 {
        self.sync_interval_minutes
    }

    pub fn get_statistics(&self) -> TimeManagerStatistics {
        TimeManagerStatistics {
            total_timezones: self.timezones.len(),
            total_ntp_servers: self.ntp_servers.len(),
            current_timezone_set: self.current_timezone.is_some(),
            active_ntp_server_set: self.get_active_ntp_server().is_some(),
            auto_sync_enabled: self.auto_sync,
        }
    }
}

impl Default for DesktopTimeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// TimeManagerStatistics
#[derive(Debug, Clone, Copy)]
pub struct TimeManagerStatistics {
    pub total_timezones: usize,
    pub total_ntp_servers: usize,
    pub current_timezone_set: bool,
    pub active_ntp_server_set: bool,
    pub auto_sync_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_manager_state() {
        let manager = DesktopTimeManager::new();
        let stats = manager.get_statistics();

        assert!(stats.total_timezones >= 4);
        assert!(stats.total_ntp_servers >= 3);
        assert!(stats.current_timezone_set);
        assert!(stats.active_ntp_server_set);
        assert!(stats.auto_sync_enabled);
    }

    #[test]
    fn test_add_timezone() {
        let mut manager = DesktopTimeManager::new();
        let initial_count = manager.get_timezones().len();

        let timezone = DesktopTimezone::new(
            "custom".to_string(),
            "Custom DesktopTimezone".to_string(),
            "Custom".to_string(),
            "City".to_string(),
            2,
        );

        let id = manager.add_timezone(timezone);
        assert!(manager.get_timezone(&id).is_some());
        assert_eq!(manager.get_timezones().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_timezone() {
        let mut manager = DesktopTimeManager::new();

        let timezone = DesktopTimezone::new(
            "custom".to_string(),
            "Custom DesktopTimezone".to_string(),
            "Custom".to_string(),
            "City".to_string(),
            2,
        );

        let id = manager.add_timezone(timezone);
        assert!(manager.remove_timezone(&id));
        assert!(manager.get_timezone(&id).is_none());
    }

    #[test]
    fn test_remove_current_timezone() {
        let mut manager = DesktopTimeManager::new();

        // Try to remove current timezone (should fail)
        let current_id = manager.get_current_timezone().unwrap().id.clone();
        let result = manager.remove_timezone(&current_id);
        assert!(!result);
    }

    #[test]
    fn test_set_current_timezone() {
        let mut manager = DesktopTimeManager::new();
        let timezones = manager.get_timezones();

        if timezones.len() > 1 {
            let new_timezone_id = timezones[1].id.clone();
            assert!(manager.set_current_timezone(&new_timezone_id));
            assert_eq!(
                manager.get_current_timezone().unwrap().id,
                new_timezone_id
            );
        }
    }

    #[test]
    fn test_add_ntp_server() {
        let mut manager = DesktopTimeManager::new();
        let initial_count = manager.get_ntp_servers().len();

        let server = DesktopNTPServer::new(
            "custom".to_string(),
            "Custom NTP".to_string(),
            "custom.ntp.org".to_string(),
        );

        let id = manager.add_ntp_server(server);
        assert!(manager.get_ntp_server(&id).is_some());
        assert_eq!(manager.get_ntp_servers().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_ntp_server() {
        let mut manager = DesktopTimeManager::new();

        let server = DesktopNTPServer::new(
            "custom".to_string(),
            "Custom NTP".to_string(),
            "custom.ntp.org".to_string(),
        );

        let id = manager.add_ntp_server(server);
        assert!(manager.remove_ntp_server(&id));
        assert!(manager.get_ntp_server(&id).is_none());
    }

    #[test]
    fn test_set_active_ntp_server() {
        let mut manager = DesktopTimeManager::new();
        let ntp_servers = manager.get_ntp_servers();

        if ntp_servers.len() > 1 {
            let new_server_id = ntp_servers[1].id.clone();
            assert!(manager.set_active_ntp_server(&new_server_id));

            let active = manager.get_active_ntp_server().unwrap();
            assert_eq!(active.id, new_server_id);
            assert!(active.active);
        }
    }

    #[test]
    fn test_time_format() {
        let mut manager = DesktopTimeManager::new();

        manager.set_time_format(TimeFormat::TwelveHour);
        assert_eq!(manager.get_time_format(), TimeFormat::TwelveHour);

        manager.set_time_format(TimeFormat::TwentyFourHour);
        assert_eq!(manager.get_time_format(), TimeFormat::TwentyFourHour);
    }

    #[test]
    fn test_date_format() {
        let mut manager = DesktopTimeManager::new();

        manager.set_date_format(DateFormat::US);
        assert_eq!(manager.get_date_format(), DateFormat::US);

        manager.set_date_format(DateFormat::European);
        assert_eq!(manager.get_date_format(), DateFormat::European);
    }

    #[test]
    fn test_auto_sync() {
        let mut manager = DesktopTimeManager::new();

        manager.set_auto_sync(false);
        assert!(!manager.is_auto_sync_enabled());

        manager.set_auto_sync(true);
        assert!(manager.is_auto_sync_enabled());
    }

    #[test]
    fn test_sync_interval() {
        let mut manager = DesktopTimeManager::new();

        manager.set_sync_interval(120);
        assert_eq!(manager.get_sync_interval(), 120);
    }

    #[test]
    fn test_timezone_with_offset_minutes() {
        let mut manager = DesktopTimeManager::new();

        let timezone = DesktopTimezone::new(
            "custom".to_string(),
            "Custom".to_string(),
            "Custom".to_string(),
            "City".to_string(),
            5,
        )
        .with_offset_minutes(30);

        let id = manager.add_timezone(timezone);
        let retrieved = manager.get_timezone(&id).unwrap();
        assert_eq!(retrieved.offset_minutes, 30);
    }
}
