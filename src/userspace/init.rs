/// SigmaOS Minimal Init System (PID 1)
/// Replacing incompatible abstractions with a single service model.

#[derive(Debug, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
}

pub struct Service {
    pub name: &'static str,
    pub executable_path: &'static str,
    pub state: ServiceState,
    pub restart_on_failure: bool,
}

pub struct InitManager {
    services: Vec<Service>,
}

impl InitManager {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register_service(&mut self, service: Service) {
        self.services.push(service);
    }

    pub fn start_all(&mut self) {
        for svc in self.services.iter_mut() {
            svc.state = ServiceState::Starting;
            // Simulated execve call
            // if libc::syscall3(Execve, ...) == 0 {
            svc.state = ServiceState::Running;
        }
    }

    pub fn status_report(&self) -> String {
        let mut report = String::from("Init Service Status:\n");
        for svc in &self.services {
            report.push_str(&format!("{} - {:?}\n", svc.name, svc.state));
        }
        report
    }
}
