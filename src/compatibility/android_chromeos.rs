use std::format;
use std::vec;
// SigmaOS Android & ChromeOS Parity Subsystem Layer
// Replicates key Android and ChromeOS subsystem capabilities:
// 1. Android APK & AAB Manifest Parser & permission filter (ApkManifestParser)
// 2. Android ART / Dalvik bytecode execution sandbox (ArtBytecodeSandbox)
// 3. Android Intent routing & BroadcastReceiver dispatcher (IntentRouter)
// 4. ChromeOS Crostini / ArcVM container environment bridge (CrostiniContainerBridge)
// 5. ChromeOS Verified Boot dual A/B partition slot switcher & powerwash engine (VerifiedBootSlotSwitcher)
// 6. Activity Lifecycle & Activity State Management (AndroidActivityManager)
// 7. Application Architecture & Lifecycle Controller (AndroidApplicationController)
// 8. Low Memory Killer (LMK) & OOM Adjuster (AndroidLowMemoryKiller)
// 9. Process Termination & Force-Stop Engine (AndroidProcessKillingEngine)
// 10. Services Framework & Foreground/Background Execution (AndroidServiceManager)
// 11. System Libraries Loader & Bionic Syscall Translation (AndroidSystemLibraryLoader)
// 12. Storage Access Framework (SAF) & Content Provider Scoped Storage (StorageAccessFramework)
// 13. Android Power Manager & Wakelock Controller (AndroidPowerManager)
// 14. Binder IPC Hub & Looper Message Queue (AndroidBinderHub)
// 15. Android Thread Scheduler & Linux CGroup Priority Mapper (AndroidThreadScheduler)

use std::collections::BTreeMap;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

/// Android Activity Declaration Record
#[derive(Debug, Clone)]
pub struct AndroidActivity {
    pub name: String,
    pub exported: bool,
    pub intent_filter_actions: Vec<String>,
}

/// Android App Manifest Record (replicating AndroidManifest.xml parsed representation)
#[derive(Debug, Clone)]
pub struct AndroidAppManifest {
    pub package_name: String,
    pub version_code: u32,
    pub version_name: String,
    pub min_sdk_version: u32,
    pub target_sdk_version: u32,
    pub permissions_required: Vec<String>,
    pub activities: Vec<AndroidActivity>,
}

/// APK Manifest Parser Engine
pub struct ApkManifestParser {
    pub manifest: Option<AndroidAppManifest>,
}

impl Default for ApkManifestParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ApkManifestParser {
    pub fn new() -> Self {
        Self { manifest: None }
    }

    /// Parses Android app manifest metadata
    pub fn parse_manifest(
        &mut self,
        package_name: &str,
        version_code: u32,
        version_name: &str,
        min_sdk: u32,
        target_sdk: u32,
        permissions: &[&str],
    ) {
        self.manifest = Some(AndroidAppManifest {
            package_name: package_name.to_string(),
            version_code,
            version_name: version_name.to_string(),
            min_sdk_version: min_sdk,
            target_sdk_version: target_sdk,
            permissions_required: permissions.iter().map(|p| p.to_string()).collect(),
            activities: Vec::new(),
        });
    }

    /// Adds an activity declaration to the parsed manifest
    pub fn add_activity(&mut self, activity: AndroidActivity) -> Result<(), &'static str> {
        if let Some(manifest) = &mut self.manifest {
            manifest.activities.push(activity);
            Ok(())
        } else {
            Err("Manifest not initialized")
        }
    }

    /// Checks if the manifest requests a specific Android permission
    pub fn has_permission(&self, permission: &str) -> bool {
        if let Some(manifest) = &self.manifest {
            manifest
                .permissions_required
                .iter()
                .any(|p| p == permission)
        } else {
            false
        }
    }
}

/// Dalvik / ART DEX Class Definition
#[derive(Debug, Clone)]
pub struct DexClassDef {
    pub class_name: String,
    pub superclass_name: String,
    pub access_flags: u32,
    pub method_count: usize,
}

/// ART / Dalvik Bytecode Execution Sandbox
pub struct ArtBytecodeSandbox {
    pub loaded_classes: BTreeMap<String, DexClassDef>,
    pub granted_permissions: Vec<String>,
    pub heap_allocated_bytes: usize,
    pub jni_methods: BTreeMap<String, u64>,
}

impl Default for ArtBytecodeSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtBytecodeSandbox {
    pub fn new() -> Self {
        Self {
            loaded_classes: BTreeMap::new(),
            granted_permissions: Vec::new(),
            heap_allocated_bytes: 0,
            jni_methods: BTreeMap::new(),
        }
    }

    /// Loads a DEX class into the ART sandbox environment
    pub fn load_class(&mut self, class_def: DexClassDef) {
        let name = class_def.class_name.clone();
        self.loaded_classes.insert(name, class_def);
    }

    /// Grants an Android runtime permission to the sandbox
    pub fn grant_permission(&mut self, permission: &str) {
        self.granted_permissions.push(permission.to_string());
    }

    /// Verifies if a runtime method invocation satisfies security permissions
    pub fn verify_method_call(&self, required_permission: &str) -> bool {
        self.granted_permissions
            .iter()
            .any(|p| p == required_permission)
    }

    /// Registers a JNI native method entry point address
    pub fn register_jni_method(&mut self, method_signature: &str, address: u64) {
        self.jni_methods
            .insert(method_signature.to_string(), address);
    }

    /// Resolves a JNI native method entry point
    pub fn resolve_jni_method(&self, method_signature: &str) -> Option<u64> {
        self.jni_methods.get(method_signature).copied()
    }
}

/// Android Intent Target Component Match
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentTarget {
    pub package_name: String,
    pub activity_name: String,
}

/// Android Intent Routing & Resolution Engine
pub struct IntentRouter {
    pub registered_receivers: Vec<(String, IntentTarget)>, // (action_string, target)
}

impl Default for IntentRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl IntentRouter {
    pub fn new() -> Self {
        Self {
            registered_receivers: Vec::new(),
        }
    }

    /// Registers a component for an intent action string
    pub fn register_receiver(&mut self, action: &str, target: IntentTarget) {
        self.registered_receivers.push((action.to_string(), target));
    }

    /// Resolves an intent action string to all matching target components
    pub fn resolve_intent(&self, action: &str) -> Vec<IntentTarget> {
        self.registered_receivers
            .iter()
            .filter(|(act, _)| act == action)
            .map(|(_, target)| target.clone())
            .collect()
    }
}

/// ChromeOS Crostini / ArcVM Guest Container State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrostiniState {
    Stopped,
    Booting,
    Running,
    Suspended,
}

/// ChromeOS Crostini Container Bridge & Wayland Socket Forwarder
pub struct CrostiniContainerBridge {
    pub container_name: String,
    pub wayland_socket_forwarded: bool,
    pub sommelier_window_forwarding: bool,
    pub guest_memory_mb: usize,
    pub state: CrostiniState,
}

impl CrostiniContainerBridge {
    pub fn new(container_name: &str, memory_mb: usize) -> Self {
        Self {
            container_name: container_name.to_string(),
            wayland_socket_forwarded: true,
            sommelier_window_forwarding: true,
            guest_memory_mb: memory_mb,
            state: CrostiniState::Stopped,
        }
    }

    /// Boots the Crostini guest Linux environment
    pub fn start_container(&mut self) -> Result<(), &'static str> {
        self.state = CrostiniState::Running;
        Ok(())
    }

    /// Suspends guest memory allocations
    pub fn suspend_container(&mut self) {
        self.state = CrostiniState::Suspended;
    }
}

/// ChromeOS A/B Active Partition Slot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionSlot {
    SlotA,
    SlotB,
}

/// ChromeOS / Android Verified Boot A/B Partition Switcher & Powerwash Engine
pub struct VerifiedBootSlotSwitcher {
    pub active_slot: PartitionSlot,
    pub slot_a_verified: bool,
    pub slot_b_verified: bool,
    pub is_powerwashed: bool,
}

impl Default for VerifiedBootSlotSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

impl VerifiedBootSlotSwitcher {
    pub fn new() -> Self {
        Self {
            active_slot: PartitionSlot::SlotA,
            slot_a_verified: true,
            slot_b_verified: true,
            is_powerwashed: false,
        }
    }

    /// Toggles active boot partition slot to target slot if cryptographically verified
    pub fn switch_active_slot(&mut self, target: PartitionSlot) -> Result<(), &'static str> {
        match target {
            PartitionSlot::SlotA if self.slot_a_verified => {
                self.active_slot = PartitionSlot::SlotA;
                Ok(())
            }
            PartitionSlot::SlotB if self.slot_b_verified => {
                self.active_slot = PartitionSlot::SlotB;
                Ok(())
            }
            _ => Err("Target boot slot fails SHA-256 verified boot check"),
        }
    }

    /// Performs an atomic ChromeOS Powerwash user state wipe
    pub fn perform_powerwash(&mut self) {
        self.is_powerwashed = true;
    }
}

// ============================================================================
// Android Activity Lifecycle & Activity State Management
// ============================================================================

/// Android Activity Execution State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityState {
    Initialized,
    Created,
    Started,
    Resumed,
    Paused,
    Stopped,
    Destroyed,
}

/// Android Activity Execution Instance Record
#[derive(Debug, Clone)]
pub struct AndroidActivityRecord {
    pub activity_id: u64,
    pub name: String,
    pub package_name: String,
    pub state: ActivityState,
    pub saved_instance_state: BTreeMap<String, String>,
}

/// Android Activity Manager & Task Stack
pub struct AndroidActivityManager {
    pub activity_stack: Vec<AndroidActivityRecord>,
    pub next_activity_id: u64,
}

impl Default for AndroidActivityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidActivityManager {
    pub fn new() -> Self {
        Self {
            activity_stack: Vec::new(),
            next_activity_id: 1,
        }
    }

    /// Launches a new activity and pushes it onto the task stack
    pub fn launch_activity(&mut self, package_name: &str, name: &str) -> u64 {
        let id = self.next_activity_id;
        self.next_activity_id += 1;

        let record = AndroidActivityRecord {
            activity_id: id,
            name: name.to_string(),
            package_name: package_name.to_string(),
            state: ActivityState::Created,
            saved_instance_state: BTreeMap::new(),
        };

        self.activity_stack.push(record);
        id
    }

    /// Transitions an activity state along its lifecycle
    pub fn transition_state(
        &mut self,
        activity_id: u64,
        target_state: ActivityState,
    ) -> Result<(), &'static str> {
        if let Some(record) = self
            .activity_stack
            .iter_mut()
            .find(|a| a.activity_id == activity_id)
        {
            record.state = target_state;
            Ok(())
        } else {
            Err("Activity ID not found")
        }
    }

    /// Persists key-value instance state during low memory or screen rotation
    pub fn save_activity_state(
        &mut self,
        activity_id: u64,
        key: &str,
        value: &str,
    ) -> Result<(), &'static str> {
        if let Some(record) = self
            .activity_stack
            .iter_mut()
            .find(|a| a.activity_id == activity_id)
        {
            record
                .saved_instance_state
                .insert(key.to_string(), value.to_string());
            Ok(())
        } else {
            Err("Activity ID not found")
        }
    }

    /// Retrieves a persisted state variable for a restored activity
    pub fn restore_activity_state(&self, activity_id: u64, key: &str) -> Option<String> {
        self.activity_stack
            .iter()
            .find(|a| a.activity_id == activity_id)
            .and_then(|a| a.saved_instance_state.get(key).cloned())
    }

    /// Gets the foreground activity currently visible to the user
    pub fn get_top_activity(&self) -> Option<&AndroidActivityRecord> {
        self.activity_stack.last()
    }

    /// Pops the top activity off the task stack (e.g. on back press)
    pub fn pop_activity(&mut self) -> Option<AndroidActivityRecord> {
        self.activity_stack.pop()
    }
}

// ============================================================================
// Android Application Architecture & Lifecycle Controller
// ============================================================================

/// Android Process Execution Lifecycle State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationState {
    Unloaded,
    Starting,
    RunningForeground,
    RunningBackground,
    Terminated,
}

/// Android Application Process Metadata Record
#[derive(Debug, Clone)]
pub struct ApplicationRecord {
    pub package_name: String,
    pub pid: u32,
    pub uid: u32,
    pub state: ApplicationState,
    pub attached_activities: Vec<u64>,
    pub attached_services: Vec<String>,
}

/// Android Application Controller & System Process Registry
pub struct AndroidApplicationController {
    pub applications: BTreeMap<String, ApplicationRecord>,
    pub next_pid: u32,
}

impl Default for AndroidApplicationController {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidApplicationController {
    pub fn new() -> Self {
        Self {
            applications: BTreeMap::new(),
            next_pid: 1000,
        }
    }

    /// Registers a newly launched application process
    pub fn register_application(&mut self, package_name: &str, uid: u32) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;

        let record = ApplicationRecord {
            package_name: package_name.to_string(),
            pid,
            uid,
            state: ApplicationState::Starting,
            attached_activities: Vec::new(),
            attached_services: Vec::new(),
        };

        self.applications.insert(package_name.to_string(), record);
        pid
    }

    /// Updates the application state
    pub fn update_app_state(
        &mut self,
        package_name: &str,
        state: ApplicationState,
    ) -> Result<(), &'static str> {
        if let Some(app) = self.applications.get_mut(package_name) {
            app.state = state;
            Ok(())
        } else {
            Err("Application not registered")
        }
    }

    /// Attaches an activity instance ID to an application process
    pub fn attach_activity(
        &mut self,
        package_name: &str,
        activity_id: u64,
    ) -> Result<(), &'static str> {
        if let Some(app) = self.applications.get_mut(package_name) {
            app.attached_activities.push(activity_id);
            Ok(())
        } else {
            Err("Application not registered")
        }
    }

    /// Attaches a service name to an application process
    pub fn attach_service(
        &mut self,
        package_name: &str,
        service_name: &str,
    ) -> Result<(), &'static str> {
        if let Some(app) = self.applications.get_mut(package_name) {
            app.attached_services.push(service_name.to_string());
            Ok(())
        } else {
            Err("Application not registered")
        }
    }

    /// Retrieves application process metadata
    pub fn get_application(&self, package_name: &str) -> Option<&ApplicationRecord> {
        self.applications.get(package_name)
    }
}

// ============================================================================
// Low Memory Killer (LMK) & OOM Adjuster
// ============================================================================

/// Android Out-Of-Memory Adjustment Score (oom_adj / oom_score_adj)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OomAdjScore {
    Foreground = 0,
    Visible = 100,
    Perceptible = 200,
    Backup = 300,
    Service = 500,
    Home = 600,
    Previous = 700,
    ServiceB = 800,
    Cached = 900,
}

/// Process Memory and Priority Profile for LMK Evaluation
#[derive(Debug, Clone)]
pub struct ProcessMemoryProfile {
    pub pid: u32,
    pub package_name: String,
    pub oom_adj: OomAdjScore,
    pub rss_mb: usize,
    pub is_frozen: bool,
}

/// Android Low Memory Killer (LMK) Subsystem inspired by Linux psi / LMK and FreeBSD swap limits
pub struct AndroidLowMemoryKiller {
    pub memory_threshold_mb: usize,
    pub process_profiles: BTreeMap<u32, ProcessMemoryProfile>,
}

impl AndroidLowMemoryKiller {
    pub fn new(memory_threshold_mb: usize) -> Self {
        Self {
            memory_threshold_mb,
            process_profiles: BTreeMap::new(),
        }
    }

    /// Registers a process profile into the LMK monitor
    pub fn register_process(
        &mut self,
        pid: u32,
        package_name: &str,
        oom_adj: OomAdjScore,
        rss_mb: usize,
    ) {
        let profile = ProcessMemoryProfile {
            pid,
            package_name: package_name.to_string(),
            oom_adj,
            rss_mb,
            is_frozen: false,
        };
        self.process_profiles.insert(pid, profile);
    }

    /// Updates the oom_adj priority score for a process
    pub fn update_oom_adj(&mut self, pid: u32, oom_adj: OomAdjScore) {
        if let Some(profile) = self.process_profiles.get_mut(&pid) {
            profile.oom_adj = oom_adj;
        }
    }

    /// Assesses whether system RAM falls below the critical threshold
    pub fn assess_memory_pressure(&self, current_free_mb: usize) -> bool {
        current_free_mb < self.memory_threshold_mb
    }

    /// Selects and terminates lowest priority (highest OOM score) processes to free memory
    pub fn reclaim_memory(&mut self, current_free_mb: usize) -> Vec<u32> {
        let mut killed_pids = Vec::new();
        if !self.assess_memory_pressure(current_free_mb) {
            return killed_pids;
        }

        // Collect candidates sorted by highest oom_adj score descending
        let mut candidates: Vec<(u32, OomAdjScore, usize)> = self
            .process_profiles
            .iter()
            .map(|(&pid, p)| (pid, p.oom_adj, p.rss_mb))
            .collect();

        candidates.sort_by(|a, b| b.1.cmp(&a.1));

        let mut freed_mb = 0;
        let needed_mb = self.memory_threshold_mb.saturating_sub(current_free_mb);

        for (pid, score, rss) in candidates {
            if score == OomAdjScore::Foreground {
                // Do not kill active foreground apps unless under extreme memory collapse
                continue;
            }
            killed_pids.push(pid);
            freed_mb += rss;
            if freed_mb >= needed_mb {
                break;
            }
        }

        for pid in &killed_pids {
            self.process_profiles.remove(pid);
        }

        killed_pids
    }
}

// ============================================================================
// Process & Application Termination Engine
// ============================================================================

/// Reason for Process Termination
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationReason {
    OomKill,
    UserForceStop,
    Crash,
    SystemResourceReclaim,
}

/// Process Termination Event Record
#[derive(Debug, Clone)]
pub struct TerminationEvent {
    pub pid: u32,
    pub package_name: String,
    pub reason: TerminationReason,
    pub timestamp: u64,
}

/// Android Process & Application Termination Engine (Linux SIGKILL / FreeBSD kill(2) wrapper)
pub struct AndroidProcessKillingEngine {
    pub termination_history: Vec<TerminationEvent>,
}

impl Default for AndroidProcessKillingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidProcessKillingEngine {
    pub fn new() -> Self {
        Self {
            termination_history: Vec::new(),
        }
    }

    /// Gracefully terminates a process, ensuring activity states are saved first
    pub fn kill_process_gracefully(
        &mut self,
        app_ctrl: &mut AndroidApplicationController,
        act_mgr: &mut AndroidActivityManager,
        pid: u32,
        reason: TerminationReason,
        timestamp: u64,
    ) -> Result<(), &'static str> {
        let package_name = app_ctrl
            .applications
            .values()
            .find(|app| app.pid == pid)
            .map(|app| app.package_name.clone());

        if let Some(pkg) = package_name {
            // Save state of attached activities
            if let Some(app) = app_ctrl.applications.get(&pkg) {
                for &act_id in &app.attached_activities {
                    let _ =
                        act_mgr.save_activity_state(act_id, "killed_by", "ProcessKillingEngine");
                    let _ = act_mgr.transition_state(act_id, ActivityState::Destroyed);
                }
            }

            // Update app state to terminated
            let _ = app_ctrl.update_app_state(&pkg, ApplicationState::Terminated);

            self.termination_history.push(TerminationEvent {
                pid,
                package_name: pkg,
                reason,
                timestamp,
            });

            Ok(())
        } else {
            Err("PID not found in application registry")
        }
    }

    /// Forcefully kills all processes belonging to an application package
    pub fn force_kill_application(
        &mut self,
        app_ctrl: &mut AndroidApplicationController,
        package_name: &str,
        reason: TerminationReason,
        timestamp: u64,
    ) -> Result<u32, &'static str> {
        if let Some(app) = app_ctrl.applications.get_mut(package_name) {
            let pid = app.pid;
            app.state = ApplicationState::Terminated;

            self.termination_history.push(TerminationEvent {
                pid,
                package_name: package_name.to_string(),
                reason,
                timestamp,
            });

            Ok(pid)
        } else {
            Err("Package not found")
        }
    }
}

// ============================================================================
// Services Framework & Foreground/Background Execution
// ============================================================================

/// Android Service Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    Foreground,
    Background,
    Bound,
}

/// Android Service State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Destroyed,
}

/// Android Service Metadata Record
#[derive(Debug, Clone)]
pub struct AndroidServiceRecord {
    pub service_name: String,
    pub package_name: String,
    pub service_type: ServiceType,
    pub state: ServiceState,
    pub active_bindings: usize,
    pub foreground_notification_id: Option<u32>,
}

/// Android Service Manager
pub struct AndroidServiceManager {
    pub services: BTreeMap<String, AndroidServiceRecord>,
}

impl Default for AndroidServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidServiceManager {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    /// Starts an Android service
    pub fn start_service(
        &mut self,
        package_name: &str,
        service_name: &str,
        service_type: ServiceType,
    ) -> Result<(), &'static str> {
        let record = AndroidServiceRecord {
            service_name: service_name.to_string(),
            package_name: package_name.to_string(),
            service_type,
            state: ServiceState::Running,
            active_bindings: 0,
            foreground_notification_id: None,
        };

        self.services.insert(service_name.to_string(), record);
        Ok(())
    }

    /// Binds a foreground notification to prevent service kill during background throttling
    pub fn set_foreground_notification(
        &mut self,
        service_name: &str,
        notification_id: u32,
    ) -> Result<(), &'static str> {
        if let Some(srv) = self.services.get_mut(service_name) {
            srv.service_type = ServiceType::Foreground;
            srv.foreground_notification_id = Some(notification_id);
            Ok(())
        } else {
            Err("Service not found")
        }
    }

    /// Binds a client to the service
    pub fn bind_service(&mut self, service_name: &str) -> Result<usize, &'static str> {
        if let Some(srv) = self.services.get_mut(service_name) {
            srv.active_bindings += 1;
            Ok(srv.active_bindings)
        } else {
            Err("Service not found")
        }
    }

    /// Unbinds a client from the service
    pub fn unbind_service(&mut self, service_name: &str) -> Result<usize, &'static str> {
        if let Some(srv) = self.services.get_mut(service_name) {
            srv.active_bindings = srv.active_bindings.saturating_sub(1);
            Ok(srv.active_bindings)
        } else {
            Err("Service not found")
        }
    }

    /// Stops and destroys a service
    pub fn stop_service(&mut self, service_name: &str) -> Result<(), &'static str> {
        if let Some(srv) = self.services.get_mut(service_name) {
            srv.state = ServiceState::Destroyed;
            Ok(())
        } else {
            Err("Service not found")
        }
    }
}

// ============================================================================
// System Libraries Loader & Bionic C Translation
// ============================================================================

/// Native Export Symbol
#[derive(Debug, Clone)]
pub struct NativeSymbol {
    pub name: String,
    pub address: u64,
}

/// Android System Library Loader & Bionic C Syscall Translation
pub struct AndroidSystemLibraryLoader {
    pub loaded_libraries: BTreeMap<String, Vec<NativeSymbol>>,
    pub bionic_syscall_bindings: BTreeMap<u32, String>,
}

impl Default for AndroidSystemLibraryLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidSystemLibraryLoader {
    pub fn new() -> Self {
        Self {
            loaded_libraries: BTreeMap::new(),
            bionic_syscall_bindings: BTreeMap::new(),
        }
    }

    /// Loads a native shared library (.so) into memory space
    pub fn load_library(&mut self, lib_name: &str, symbols: &[&str]) -> Result<(), &'static str> {
        let mut native_symbols = Vec::new();
        let mut base_addr =
            0x7F00_0000_0000u64 + (self.loaded_libraries.len() as u64 * 0x1000_0000);

        for sym in symbols {
            native_symbols.push(NativeSymbol {
                name: sym.to_string(),
                address: base_addr,
            });
            base_addr += 0x100;
        }

        self.loaded_libraries
            .insert(lib_name.to_string(), native_symbols);
        Ok(())
    }

    /// Resolves an exported symbol from a loaded library
    pub fn resolve_symbol(&self, lib_name: &str, symbol_name: &str) -> Option<u64> {
        self.loaded_libraries.get(lib_name).and_then(|syms| {
            syms.iter()
                .find(|s| s.name == symbol_name)
                .map(|s| s.address)
        })
    }

    /// Registers a Bionic C syscall translation mapping
    pub fn register_bionic_syscall(&mut self, syscall_num: u32, name: &str) {
        self.bionic_syscall_bindings
            .insert(syscall_num, name.to_string());
    }

    /// Translates Bionic syscall to SigmaOS native syscall name
    pub fn translate_bionic_syscall(&self, syscall_num: u32) -> Option<&String> {
        self.bionic_syscall_bindings.get(&syscall_num)
    }
}

// ============================================================================
// Storage Access Framework (SAF) & Content Provider Scoped Storage
// ============================================================================

/// Android Content URI Representation (content://authority/path)
#[derive(Debug, Clone)]
pub struct ContentUri {
    pub authority: String,
    pub path: String,
}

impl ContentUri {
    pub fn parse(uri_str: &str) -> Option<Self> {
        if let Some(stripped) = uri_str.strip_prefix("content://") {
            let mut parts = stripped.splitn(2, '/');
            let authority = parts.next()?;
            let path = parts.next().unwrap_or("");
            Some(Self {
                authority: authority.to_string(),
                path: path.to_string(),
            })
        } else {
            None
        }
    }

    pub fn to_string_uri(&self) -> String {
        alloc::format!("content://{}/{}", self.authority, self.path)
    }
}

/// Android Content Provider Registration Record
#[derive(Debug, Clone)]
pub struct ContentProviderRecord {
    pub authority: String,
    pub package_name: String,
    pub read_permission: String,
    pub write_permission: String,
    pub data_store: BTreeMap<String, String>,
}

/// Android Storage Access Framework (SAF) & OpenBSD unveil/pledge-style scoped storage manager
pub struct StorageAccessFramework {
    pub granted_uris: Vec<(String, String)>, // (package_name, uri_str)
    pub providers: BTreeMap<String, ContentProviderRecord>,
}

impl Default for StorageAccessFramework {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageAccessFramework {
    pub fn new() -> Self {
        Self {
            granted_uris: Vec::new(),
            providers: BTreeMap::new(),
        }
    }

    /// Registers a Content Provider
    pub fn register_provider(&mut self, provider: ContentProviderRecord) {
        self.providers.insert(provider.authority.clone(), provider);
    }

    /// Grants a URI permission grant to an application package
    pub fn grant_uri_permission(&mut self, package_name: &str, uri_str: &str) {
        self.granted_uris
            .push((package_name.to_string(), uri_str.to_string()));
    }

    /// Queries a Content Provider using SAF security scoping
    pub fn query(
        &self,
        package_name: &str,
        uri_str: &str,
        key: &str,
    ) -> Result<Option<&String>, &'static str> {
        let parsed = ContentUri::parse(uri_str).ok_or("Invalid content URI")?;

        let has_grant = self
            .granted_uris
            .iter()
            .any(|(pkg, u)| pkg == package_name && u == uri_str);

        if !has_grant {
            return Err("StorageAccessFramework: Permission denied for URI");
        }

        if let Some(provider) = self.providers.get(&parsed.authority) {
            Ok(provider.data_store.get(key))
        } else {
            Err("Content provider not found for authority")
        }
    }

    /// Inserts data into a Content Provider
    pub fn insert(
        &mut self,
        package_name: &str,
        uri_str: &str,
        key: &str,
        value: &str,
    ) -> Result<(), &'static str> {
        let parsed = ContentUri::parse(uri_str).ok_or("Invalid content URI")?;

        let has_grant = self
            .granted_uris
            .iter()
            .any(|(pkg, u)| pkg == package_name && u == uri_str);

        if !has_grant {
            return Err("StorageAccessFramework: Permission denied for URI write");
        }

        if let Some(provider) = self.providers.get_mut(&parsed.authority) {
            provider
                .data_store
                .insert(key.to_string(), value.to_string());
            Ok(())
        } else {
            Err("Content provider not found for authority")
        }
    }
}

// ============================================================================
// Android Power Manager & Wakelock Controller
// ============================================================================

/// Android Wakelock Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakelockType {
    Partial,
    ScreenDim,
    ScreenBright,
    Full,
}

/// Android Wakelock Record
#[derive(Debug, Clone)]
pub struct WakelockRecord {
    pub tag: String,
    pub package_name: String,
    pub lock_type: WakelockType,
    pub held: bool,
}

/// Android Power Manager & Doze Mode Engine
pub struct AndroidPowerManager {
    pub wakelocks: BTreeMap<String, WakelockRecord>,
    pub is_doze_mode: bool,
    pub screen_on: bool,
}

impl Default for AndroidPowerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidPowerManager {
    pub fn new() -> Self {
        Self {
            wakelocks: BTreeMap::new(),
            is_doze_mode: false,
            screen_on: true,
        }
    }

    /// Acquires a power wakelock preventing CPU sleep or screen turn-off
    pub fn acquire_wakelock(&mut self, package_name: &str, tag: &str, lock_type: WakelockType) {
        let record = WakelockRecord {
            tag: tag.to_string(),
            package_name: package_name.to_string(),
            lock_type,
            held: true,
        };
        self.wakelocks.insert(tag.to_string(), record);
    }

    /// Releases a held wakelock
    pub fn release_wakelock(&mut self, tag: &str) -> Result<(), &'static str> {
        if let Some(wl) = self.wakelocks.get_mut(tag) {
            wl.held = false;
            Ok(())
        } else {
            Err("Wakelock tag not found")
        }
    }

    /// Sets system Doze Mode (deep power sleep)
    pub fn set_doze_mode(&mut self, enabled: bool) {
        self.is_doze_mode = enabled;
        if enabled {
            self.screen_on = false;
        }
    }

    /// Checks if CPU must remain awake based on active held wakelocks
    pub fn is_cpu_awake(&self) -> bool {
        if self.screen_on {
            return true;
        }
        self.wakelocks.values().any(|wl| wl.held)
    }
}

// ============================================================================
// Binder IPC Hub & Looper Message Queue
// ============================================================================

/// Binder Inter-Process Transaction Payload
#[derive(Debug, Clone)]
pub struct BinderTransactionPayload {
    pub sender_pid: u32,
    pub target_handle: u32,
    pub code: u32,
    pub data: Vec<u8>,
}

/// Android Handler / Looper Message Item
#[derive(Debug, Clone)]
pub struct LooperMessage {
    pub what: u32,
    pub arg1: i32,
    pub arg2: i32,
    pub target_handler: String,
}

/// Android Binder IPC Hub & Handler Queue
pub struct AndroidBinderHub {
    pub message_queues: BTreeMap<u32, Vec<LooperMessage>>, // pid -> messages
    pub pending_transactions: Vec<BinderTransactionPayload>,
}

impl Default for AndroidBinderHub {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidBinderHub {
    pub fn new() -> Self {
        Self {
            message_queues: BTreeMap::new(),
            pending_transactions: Vec::new(),
        }
    }

    /// Sends an IPC transaction over the Binder IPC bus
    pub fn send_binder_transaction(&mut self, payload: BinderTransactionPayload) {
        self.pending_transactions.push(payload);
    }

    /// Enqueues a message into an application main thread Looper
    pub fn post_looper_message(&mut self, pid: u32, msg: LooperMessage) {
        self.message_queues.entry(pid).or_default().push(msg);
    }

    /// Polls messages from an application thread Looper
    pub fn poll_looper(&mut self, pid: u32) -> Vec<LooperMessage> {
        self.message_queues.remove(&pid).unwrap_or_default()
    }
}

// ============================================================================
// Android Thread Scheduler & Linux CGroup Priority Mapper
// ============================================================================

/// Android Thread Execution Role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidThreadType {
    MainUiThread,
    HandlerThread,
    WorkerThread,
    RenderThread,
}

/// Android Thread Metadata
#[derive(Debug, Clone)]
pub struct AndroidThreadInfo {
    pub tid: u32,
    pub pid: u32,
    pub thread_type: AndroidThreadType,
    pub linux_nice_priority: i32,
    pub cgroup_path: String,
}

/// Android Thread Scheduler & Linux CGroup / pthread Mapper
pub struct AndroidThreadScheduler {
    pub threads: BTreeMap<u32, AndroidThreadInfo>,
}

impl Default for AndroidThreadScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidThreadScheduler {
    pub fn new() -> Self {
        Self {
            threads: BTreeMap::new(),
        }
    }

    /// Registers a thread and maps it to appropriate Linux cgroup and nice priority
    pub fn register_thread(&mut self, tid: u32, pid: u32, thread_type: AndroidThreadType) {
        let (nice, cgroup) = match thread_type {
            AndroidThreadType::MainUiThread => (-10, "/sys/fs/cgroup/top-app"),
            AndroidThreadType::RenderThread => (-10, "/sys/fs/cgroup/top-app"),
            AndroidThreadType::HandlerThread => (0, "/sys/fs/cgroup/foreground"),
            AndroidThreadType::WorkerThread => (10, "/sys/fs/cgroup/background"),
        };

        let info = AndroidThreadInfo {
            tid,
            pid,
            thread_type,
            linux_nice_priority: nice,
            cgroup_path: cgroup.to_string(),
        };

        self.threads.insert(tid, info);
    }

    /// Retrieves thread scheduling information
    pub fn get_thread_info(&self, tid: u32) -> Option<&AndroidThreadInfo> {
        self.threads.get(&tid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apk_manifest_parsing() {
        let mut parser = ApkManifestParser::new();
        parser.parse_manifest(
            "org.sigmaos.calculator",
            100,
            "1.0.0",
            26,
            34,
            &["android.permission.INTERNET", "android.permission.CAMERA"],
        );

        assert!(parser.has_permission("android.permission.INTERNET"));
        assert!(!parser.has_permission("android.permission.ACCESS_FINE_LOCATION"));

        let activity = AndroidActivity {
            name: "org.sigmaos.calculator.MainActivity".to_string(),
            exported: true,
            intent_filter_actions: vec!["android.intent.action.MAIN".to_string()],
        };
        parser.add_activity(activity).unwrap();
        assert_eq!(parser.manifest.as_ref().unwrap().activities.len(), 1);
    }

    #[test]
    fn test_art_bytecode_sandbox() {
        let mut art = ArtBytecodeSandbox::new();
        let class_def = DexClassDef {
            class_name: "Ljava/lang/String;".to_string(),
            superclass_name: "Ljava/lang/Object;".to_string(),
            access_flags: 0x0001,
            method_count: 42,
        };
        art.load_class(class_def);
        assert_eq!(art.loaded_classes.len(), 1);

        art.grant_permission("android.permission.CAMERA");
        assert!(art.verify_method_call("android.permission.CAMERA"));
        assert!(!art.verify_method_call("android.permission.RECORD_AUDIO"));

        art.register_jni_method("Java_com_example_Native_compute", 0xDEAD_BEEF);
        assert_eq!(
            art.resolve_jni_method("Java_com_example_Native_compute"),
            Some(0xDEAD_BEEF)
        );
    }

    #[test]
    fn test_intent_routing() {
        let mut router = IntentRouter::new();
        let target = IntentTarget {
            package_name: "com.android.camera".to_string(),
            activity_name: "com.android.camera.CameraActivity".to_string(),
        };
        router.register_receiver("android.media.action.IMAGE_CAPTURE", target.clone());

        let matches = router.resolve_intent("android.media.action.IMAGE_CAPTURE");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], target);
    }

    #[test]
    fn test_crostini_container_bridge() {
        let mut crostini = CrostiniContainerBridge::new("penguin", 4096);
        assert_eq!(crostini.state, CrostiniState::Stopped);

        crostini.start_container().unwrap();
        assert_eq!(crostini.state, CrostiniState::Running);
        assert!(crostini.wayland_socket_forwarded);
    }

    #[test]
    fn test_verified_boot_slot_switcher() {
        let mut vboot = VerifiedBootSlotSwitcher::new();
        assert_eq!(vboot.active_slot, PartitionSlot::SlotA);

        vboot.switch_active_slot(PartitionSlot::SlotB).unwrap();
        assert_eq!(vboot.active_slot, PartitionSlot::SlotB);

        vboot.perform_powerwash();
        assert!(vboot.is_powerwashed);
    }

    #[test]
    fn test_activity_lifecycle_management() {
        let mut manager = AndroidActivityManager::new();
        let act_id = manager.launch_activity("com.example.app", "MainActivity");
        assert_eq!(act_id, 1);

        assert_eq!(
            manager.get_top_activity().unwrap().state,
            ActivityState::Created
        );

        manager
            .transition_state(act_id, ActivityState::Resumed)
            .unwrap();
        assert_eq!(
            manager.get_top_activity().unwrap().state,
            ActivityState::Resumed
        );

        manager
            .save_activity_state(act_id, "scroll_position", "1250")
            .unwrap();
        assert_eq!(
            manager
                .restore_activity_state(act_id, "scroll_position")
                .unwrap(),
            "1250"
        );

        let popped = manager.pop_activity().unwrap();
        assert_eq!(popped.name, "MainActivity");
        assert!(manager.get_top_activity().is_none());
    }

    #[test]
    fn test_application_controller() {
        let mut ctrl = AndroidApplicationController::new();
        let pid = ctrl.register_application("com.example.app", 10001);
        assert_eq!(pid, 1000);

        ctrl.update_app_state("com.example.app", ApplicationState::RunningForeground)
            .unwrap();
        ctrl.attach_activity("com.example.app", 1).unwrap();
        ctrl.attach_service("com.example.app", "SyncService")
            .unwrap();

        let app = ctrl.get_application("com.example.app").unwrap();
        assert_eq!(app.state, ApplicationState::RunningForeground);
        assert_eq!(app.attached_activities, vec![1]);
        assert_eq!(app.attached_services, vec!["SyncService".to_string()]);
    }

    #[test]
    fn test_low_memory_killer_and_oom_adj() {
        let mut lmk = AndroidLowMemoryKiller::new(1024); // threshold 1024 MB
        lmk.register_process(1001, "com.fg.app", OomAdjScore::Foreground, 300);
        lmk.register_process(1002, "com.cached.app1", OomAdjScore::Cached, 400);
        lmk.register_process(1003, "com.cached.app2", OomAdjScore::Cached, 500);

        assert!(lmk.assess_memory_pressure(500)); // 500 < 1024 MB free

        let killed = lmk.reclaim_memory(500);
        // Cached apps should be killed first, foreground app preserved
        assert!(killed.contains(&1002) || killed.contains(&1003));
        assert!(!killed.contains(&1001));
    }

    #[test]
    fn test_process_killing_engine() {
        let mut app_ctrl = AndroidApplicationController::new();
        let mut act_mgr = AndroidActivityManager::new();
        let mut kill_engine = AndroidProcessKillingEngine::new();

        let pid = app_ctrl.register_application("com.example.app", 10001);
        let act_id = act_mgr.launch_activity("com.example.app", "MainActivity");
        app_ctrl.attach_activity("com.example.app", act_id).unwrap();

        kill_engine
            .kill_process_gracefully(
                &mut app_ctrl,
                &mut act_mgr,
                pid,
                TerminationReason::OomKill,
                12345678,
            )
            .unwrap();

        let app = app_ctrl.get_application("com.example.app").unwrap();
        assert_eq!(app.state, ApplicationState::Terminated);

        assert_eq!(
            act_mgr.restore_activity_state(act_id, "killed_by").unwrap(),
            "ProcessKillingEngine"
        );
        assert_eq!(kill_engine.termination_history.len(), 1);
    }

    #[test]
    fn test_service_manager_lifecycle() {
        let mut srv_mgr = AndroidServiceManager::new();
        srv_mgr
            .start_service(
                "com.example.app",
                "AudioPlaybackService",
                ServiceType::Background,
            )
            .unwrap();

        srv_mgr
            .set_foreground_notification("AudioPlaybackService", 99)
            .unwrap();
        let srv = srv_mgr.services.get("AudioPlaybackService").unwrap();
        assert_eq!(srv.service_type, ServiceType::Foreground);
        assert_eq!(srv.foreground_notification_id, Some(99));

        assert_eq!(srv_mgr.bind_service("AudioPlaybackService").unwrap(), 1);
        assert_eq!(srv_mgr.unbind_service("AudioPlaybackService").unwrap(), 0);

        srv_mgr.stop_service("AudioPlaybackService").unwrap();
        assert_eq!(
            srv_mgr.services.get("AudioPlaybackService").unwrap().state,
            ServiceState::Destroyed
        );
    }

    #[test]
    fn test_system_library_loader_and_art() {
        let mut loader = AndroidSystemLibraryLoader::new();
        loader
            .load_library("libbinder.so", &["binder_init", "binder_txn"])
            .unwrap();

        let addr = loader.resolve_symbol("libbinder.so", "binder_init");
        assert!(addr.is_some());

        loader.register_bionic_syscall(64, "sys_write");
        assert_eq!(
            loader.translate_bionic_syscall(64),
            Some(&"sys_write".to_string())
        );
    }

    #[test]
    fn test_content_provider_and_saf() {
        let mut saf = StorageAccessFramework::new();

        let mut provider = ContentProviderRecord {
            authority: "com.example.app.provider".to_string(),
            package_name: "com.example.app".to_string(),
            read_permission: "android.permission.READ".to_string(),
            write_permission: "android.permission.WRITE".to_string(),
            data_store: BTreeMap::new(),
        };
        provider
            .data_store
            .insert("user_setting".to_string(), "dark_mode".to_string());

        saf.register_provider(provider);

        let uri = "content://com.example.app.provider/settings";

        // Query before grant should fail
        assert!(saf.query("com.other.app", uri, "user_setting").is_err());

        // Grant permission
        saf.grant_uri_permission("com.other.app", uri);

        // Query after grant should succeed
        let val = saf.query("com.other.app", uri, "user_setting").unwrap();
        assert_eq!(val, Some(&"dark_mode".to_string()));

        // Insert new value
        saf.insert("com.other.app", uri, "theme", "blue").unwrap();
        let theme = saf.query("com.other.app", uri, "theme").unwrap();
        assert_eq!(theme, Some(&"blue".to_string()));
    }

    #[test]
    fn test_power_manager_wakelocks() {
        let mut pm = AndroidPowerManager::new();
        assert!(pm.is_cpu_awake());

        pm.set_doze_mode(true);
        assert!(!pm.screen_on);
        assert!(!pm.is_cpu_awake());

        pm.acquire_wakelock("com.media.player", "AudioWakeLock", WakelockType::Partial);
        assert!(pm.is_cpu_awake());

        pm.release_wakelock("AudioWakeLock").unwrap();
        assert!(!pm.is_cpu_awake());
    }

    #[test]
    fn test_binder_ipc_looper() {
        let mut hub = AndroidBinderHub::new();

        let payload = BinderTransactionPayload {
            sender_pid: 1000,
            target_handle: 1,
            code: 10,
            data: vec![1, 2, 3, 4],
        };
        hub.send_binder_transaction(payload);
        assert_eq!(hub.pending_transactions.len(), 1);

        let msg = LooperMessage {
            what: 1,
            arg1: 10,
            arg2: 20,
            target_handler: "MainHandler".to_string(),
        };
        hub.post_looper_message(1000, msg);

        let msgs = hub.poll_looper(1000);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].what, 1);

        assert!(hub.poll_looper(1000).is_empty());
    }

    #[test]
    fn test_thread_scheduler() {
        let mut sched = AndroidThreadScheduler::new();
        sched.register_thread(2001, 1000, AndroidThreadType::MainUiThread);
        sched.register_thread(2002, 1000, AndroidThreadType::WorkerThread);

        let main_info = sched.get_thread_info(2001).unwrap();
        assert_eq!(main_info.linux_nice_priority, -10);
        assert_eq!(main_info.cgroup_path, "/sys/fs/cgroup/top-app");

        let worker_info = sched.get_thread_info(2002).unwrap();
        assert_eq!(worker_info.linux_nice_priority, 10);
        assert_eq!(worker_info.cgroup_path, "/sys/fs/cgroup/background");
    }
}
