#[derive(Debug, Clone, PartialEq)]
pub enum CpuGovernor {
    Performance,
    Powersave,
    Schedutil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GpuProfile {
    Auto,
    HighPerformance,
}

pub struct GameModeClient {
    pub pid: u32,
    pub original_governor: CpuGovernor,
    pub original_priority: i32,
}

pub struct GameModeDaemon {
    pub active_clients: Vec<GameModeClient>,
    pub current_governor: CpuGovernor,
    pub current_gpu_profile: GpuProfile,
    pub compositor_unredirect: bool,
}

impl GameModeDaemon {
    pub fn new() -> Self {
        Self {
            active_clients: Vec::new(),
            current_governor: CpuGovernor::Schedutil,
            current_gpu_profile: GpuProfile::Auto,
            compositor_unredirect: false,
        }
    }

    pub fn register_client(&mut self, pid: u32) {
        if self.active_clients.is_empty() {
            self.enable_gamemode();
        }
        self.active_clients.push(GameModeClient {
            pid,
            original_governor: CpuGovernor::Schedutil, // Dummy
            original_priority: 0,
        });
    }

    pub fn unregister_client(&mut self, pid: u32) {
        self.active_clients.retain(|c| c.pid != pid);
        if self.active_clients.is_empty() {
            self.disable_gamemode();
        }
    }

    fn enable_gamemode(&mut self) {
        self.current_governor = CpuGovernor::Performance;
        self.current_gpu_profile = GpuProfile::HighPerformance;
        self.compositor_unredirect = true;
    }

    fn disable_gamemode(&mut self) {
        self.current_governor = CpuGovernor::Schedutil;
        self.current_gpu_profile = GpuProfile::Auto;
        self.compositor_unredirect = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_init() {
        let daemon = GameModeDaemon::new();
        assert_eq!(daemon.active_clients.len(), 0);
        assert_eq!(daemon.current_governor, CpuGovernor::Schedutil);
    }

    #[test]
    fn test_client_registration() {
        let mut daemon = GameModeDaemon::new();
        daemon.register_client(1234);
        assert_eq!(daemon.active_clients.len(), 1);
        assert_eq!(daemon.current_governor, CpuGovernor::Performance);
        assert_eq!(daemon.compositor_unredirect, true);
    }

    #[test]
    fn test_multiple_clients() {
        let mut daemon = GameModeDaemon::new();
        daemon.register_client(1234);
        daemon.register_client(5678);
        assert_eq!(daemon.active_clients.len(), 2);
    }

    #[test]
    fn test_client_unregistration() {
        let mut daemon = GameModeDaemon::new();
        daemon.register_client(1234);
        daemon.unregister_client(1234);
        assert_eq!(daemon.active_clients.len(), 0);
        assert_eq!(daemon.current_governor, CpuGovernor::Schedutil);
        assert_eq!(daemon.compositor_unredirect, false);
    }

    #[test]
    fn test_unregistration_with_remaining_clients() {
        let mut daemon = GameModeDaemon::new();
        daemon.register_client(1234);
        daemon.register_client(5678);
        daemon.unregister_client(1234);
        assert_eq!(daemon.active_clients.len(), 1);
        assert_eq!(daemon.current_governor, CpuGovernor::Performance); // Still in gamemode
    }
}
