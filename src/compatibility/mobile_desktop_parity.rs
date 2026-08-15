// SigmaOS Mobile & Desktop Competitor Parity Subsystem
// Independent, zero-dependency implementations of macOS, iOS, and Android core technologies

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;

// =========================================================================
// 1. ANDROID AOSP BINDER IPC EMULATOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinderTransactionType {
    OneWay,
    TwoWay,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinderParcel {
    pub transaction_code: u32,
    pub payload_bytes: Vec<u8>,
    pub interface_token: String,
}

pub struct AospBinderIpc {
    pub registered_endpoints: BTreeMap<u32, String>, // handle -> interface descriptor
    pub transaction_queue: Vec<(u32, BinderParcel, BinderTransactionType)>, // (handle, parcel, type)
}

impl AospBinderIpc {
    pub fn new() -> Self {
        Self {
            registered_endpoints: BTreeMap::new(),
            transaction_queue: Vec::new(),
        }
    }

    pub fn register_endpoint(&mut self, handle: u32, descriptor: &str) {
        self.registered_endpoints.insert(handle, descriptor.to_string());
    }

    pub fn send_transaction(
        &mut self,
        handle: u32,
        code: u32,
        payload: &[u8],
        interface: &str,
        tx_type: BinderTransactionType,
    ) -> Result<(), &'static str> {
        if !self.registered_endpoints.contains_key(&handle) {
            return Err("Binder Transaction Failed: Dead Object / invalid handle");
        }

        let parcel = BinderParcel {
            transaction_code: code,
            payload_bytes: payload.to_vec(),
            interface_token: interface.to_string(),
        };

        self.transaction_queue.push((handle, parcel, tx_type));
        Ok(())
    }

    pub fn dispatch_next_transaction(&mut self) -> Option<(u32, BinderParcel, BinderTransactionType)> {
        if self.transaction_queue.is_empty() {
            None
        } else {
            Some(self.transaction_queue.remove(0))
        }
    }
}

impl Default for AospBinderIpc {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. MACOS & IOS LAUNCHD DAEMON SERVICE SUPERVISOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchdServiceState {
    Stopped,
    Running,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchdService {
    pub label: String,
    pub state: LaunchdServiceState,
    pub socket_activation_port: Option<u16>,
    pub keep_alive: bool,
    pub crash_restarts: usize,
}

pub struct MacosLaunchdDaemon {
    pub services: BTreeMap<String, LaunchdService>,
    pub active_socket_connections: BTreeMap<u16, String>, // port -> targeted service label
}

impl MacosLaunchdDaemon {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
            active_socket_connections: BTreeMap::new(),
        }
    }

    pub fn register_service_from_plist(
        &mut self,
        label: &str,
        port: Option<u16>,
        keep_alive: bool,
    ) -> Result<(), &'static str> {
        if self.services.contains_key(&label.to_string()) {
            return Err("Service already registered in launchd");
        }

        let service = LaunchdService {
            label: label.to_string(),
            state: LaunchdServiceState::Stopped,
            socket_activation_port: port,
            keep_alive,
            crash_restarts: 0,
        };

        self.services.insert(label.to_string(), service);

        if let Some(p) = port {
            self.active_socket_connections.insert(p, label.to_string());
        }

        Ok(())
    }

    pub fn trigger_socket_activation(&mut self, port: u16) -> Result<String, &'static str> {
        let label = self
            .active_socket_connections
            .get(&port)
            .ok_or("No socket-activated service mapped to this port")?
            .clone();

        if let Some(service) = self.services.get_mut(&label) {
            if service.state == LaunchdServiceState::Stopped {
                service.state = LaunchdServiceState::Running;
                return Ok(format!("launchd: service '{}' socket-activated successfully", label));
            }
        }
        Err("Service already running or failed")
    }

    pub fn handle_service_crash(&mut self, label: &str) -> Result<&'static str, &'static str> {
        let service = self
            .services
            .get_mut(&label.to_string())
            .ok_or("Service not found")?;

        service.state = LaunchdServiceState::Failed;

        if service.keep_alive {
            service.state = LaunchdServiceState::Running;
            service.crash_restarts += 1;
            Ok("launchd: KeepAlive true, auto-restarted service")
        } else {
            Ok("launchd: service terminated")
        }
    }
}

impl Default for MacosLaunchdDaemon {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. HARDWARE SECURE ENCLAVE KEYSTORE
// =========================================================================

pub struct SecureEnclaveKeyStore {
    pub keychain_records: BTreeMap<String, Vec<u8>>, // account_id -> encrypted_secret
    pub access_groups: BTreeMap<String, Vec<String>>, // group_id -> allowed_accounts
    pub active_biometric_session_token: Option<String>,
}

impl SecureEnclaveKeyStore {
    pub fn new() -> Self {
        Self {
            keychain_records: BTreeMap::new(),
            access_groups: BTreeMap::new(),
            active_biometric_session_token: None,
        }
    }

    pub fn register_keychain_item(
        &mut self,
        account: &str,
        secret: &[u8],
        group: &str,
    ) {
        self.keychain_records.insert(account.to_string(), secret.to_vec());
        if let Some(list) = self.access_groups.get_mut(&group.to_string()) {
            if !list.contains(&account.to_string()) {
                list.push(account.to_string());
            }
        } else {
            self.access_groups.insert(group.to_string(), vec![account.to_string()]);
        }
    }

    pub fn raise_biometric_token(&mut self, token: &str) {
        self.active_biometric_session_token = Some(token.to_string());
    }

    pub fn invalidate_biometric_token(&mut self) {
        self.active_biometric_session_token = None;
    }

    pub fn access_keychain_secret(
        &self,
        account: &str,
        group_id: &str,
        biometric_token: &str,
    ) -> Result<Vec<u8>, &'static str> {
        // Enforce Keychain access group validation
        if let Some(allowed_accounts) = self.access_groups.get(&group_id.to_string()) {
            if !allowed_accounts.contains(&account.to_string()) {
                return Err("SecureEnclave Access Denied: Keychain Access Group mismatch");
            }
        } else {
            return Err("SecureEnclave Access Denied: Access Group is not registered");
        }

        // Enforce Biometric Session validation
        if let Some(ref active_token) = self.active_biometric_session_token {
            if active_token != biometric_token {
                return Err("SecureEnclave Access Denied: Biometric Session mismatch / invalid token");
            }
        } else {
            return Err("SecureEnclave Access Denied: Biometric verification required");
        }

        self.keychain_records
            .get(&account.to_string())
            .cloned()
            .ok_or("SecureEnclave Error: Keychain record not found")
    }
}

impl Default for SecureEnclaveKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aosp_binder_ipc_transactions() {
        let mut binder = AospBinderIpc::new();
        binder.register_endpoint(101, "android.hardware.camera");

        // Send Transaction to registered handle
        assert!(binder
            .send_transaction(101, 1, b"capture_frame", "android.hardware.camera", BinderTransactionType::TwoWay)
            .is_ok());

        // Send Transaction to invalid handle
        assert!(binder
            .send_transaction(999, 1, b"capture_frame", "android.hardware.camera", BinderTransactionType::TwoWay)
            .is_err());

        // Dispatch next transaction
        let (handle, parcel, tx_type) = binder.dispatch_next_transaction().unwrap();
        assert_eq!(handle, 101);
        assert_eq!(parcel.transaction_code, 1);
        assert_eq!(parcel.payload_bytes, b"capture_frame");
        assert_eq!(tx_type, BinderTransactionType::TwoWay);

        assert!(binder.dispatch_next_transaction().is_none());
    }

    #[test]
    fn test_macos_launchd_daemon_supervision() {
        let mut launchd = MacosLaunchdDaemon::new();
        assert!(launchd.register_service_from_plist("com.apple.WindowServer", Some(8080), true).is_ok());
        assert!(launchd.register_service_from_plist("com.apple.WindowServer", Some(8080), true).is_err());

        // Socket activation trigger
        let act_msg = launchd.trigger_socket_activation(8080).unwrap();
        assert!(act_msg.contains("WindowServer"));
        assert_eq!(launchd.services.get("com.apple.WindowServer").unwrap().state, LaunchdServiceState::Running);

        // Crash and KeepAlive restart loop
        let crash_msg = launchd.handle_service_crash("com.apple.WindowServer").unwrap();
        assert!(crash_msg.contains("auto-restarted"));
        assert_eq!(launchd.services.get("com.apple.WindowServer").unwrap().crash_restarts, 1);
        assert_eq!(launchd.services.get("com.apple.WindowServer").unwrap().state, LaunchdServiceState::Running);
    }

    #[test]
    fn test_secure_enclave_biometrics_and_access_groups() {
        let mut enclave = SecureEnclaveKeyStore::new();
        enclave.register_keychain_item("user_token_abc", b"top-secret-password-123", "group.com.apple.mobilemail");

        // Attempt access without biometrics (fails)
        let fail_bio = enclave.access_keychain_secret("user_token_abc", "group.com.apple.mobilemail", "invalid_token");
        assert!(fail_bio.is_err());

        // Raise biometric verification token
        enclave.raise_biometric_token("bio-session-789");

        // Attempt access with mismatched access group (fails)
        let fail_group = enclave.access_keychain_secret("user_token_abc", "group.com.apple.safari", "bio-session-789");
        assert!(fail_group.is_err());

        // Correct credentials and biometrics (succeeds)
        let secret = enclave.access_keychain_secret("user_token_abc", "group.com.apple.mobilemail", "bio-session-789").unwrap();
        assert_eq!(secret, b"top-secret-password-123");

        // Invalidate biometric session
        enclave.invalidate_biometric_token();
        let fail_expired = enclave.access_keychain_secret("user_token_abc", "group.com.apple.mobilemail", "bio-session-789");
        assert!(fail_expired.is_err());
    }
}
