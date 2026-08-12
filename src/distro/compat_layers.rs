use std::collections::HashMap;

/// Represents Windows Registry Value Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryType {
    String,
    Dword,
    Binary,
}

/// Represents an individual Windows Registry Key Value
#[derive(Debug, Clone)]
pub struct RegistryValue {
    pub name: String,
    pub val_type: RegistryType,
    pub data: Vec<u8>,
}

/// Simulates Windows Registry hives, keys, values, and transaction logging (WINE/ReactOS style)
#[derive(Debug, Clone)]
pub struct WindowsRegistry {
    pub hive_name: String,
    pub database: HashMap<String, HashMap<String, RegistryValue>>, // Key path -> (Value Name -> RegistryValue)
    pub transaction_log: Vec<String>,
}

impl WindowsRegistry {
    pub fn new(hive_name: &str) -> Self {
        Self {
            hive_name: hive_name.to_string(),
            database: HashMap::new(),
            transaction_log: Vec::new(),
        }
    }

    pub fn set_value(
        &mut self,
        key_path: &str,
        value_name: &str,
        val_type: RegistryType,
        data: &[u8],
    ) -> Result<(), &'static str> {
        let key_str = key_path.to_string();
        let val_str = value_name.to_string();

        let values = self
            .database
            .entry(key_str.clone())
            .or_insert_with(HashMap::new);
        values.insert(
            val_str.clone(),
            RegistryValue {
                name: val_str.clone(),
                val_type,
                data: data.to_vec(),
            },
        );

        self.transaction_log.push(format!(
            "SET_VALUE key={} value={} type={:?}",
            key_str, val_str, val_type
        ));
        Ok(())
    }

    pub fn get_value(&self, key_path: &str, value_name: &str) -> Option<&RegistryValue> {
        self.database.get(key_path)?.get(value_name)
    }
}

/// Simulates Win32 GDI drawing objects (pen, brush, font)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GdiObjectType {
    Pen,
    Brush,
    Font,
}

/// Simulates a Win32 Device Context (DC) and object binding (GDI compatibility layer)
#[derive(Debug, Clone)]
pub struct Win32Gdi {
    pub active_objects: HashMap<GdiObjectType, u32>, // type -> handle ID
    pub current_color: u32,
    pub commands_executed: Vec<String>,
}

impl Win32Gdi {
    pub fn new() -> Self {
        Self {
            active_objects: HashMap::new(),
            current_color: 0x00000000,
            commands_executed: Vec::new(),
        }
    }

    pub fn select_object(&mut self, obj_type: GdiObjectType, handle_id: u32) {
        self.active_objects.insert(obj_type, handle_id);
        self.commands_executed.push(format!(
            "SELECT_OBJECT type={:?} handle={}",
            obj_type, handle_id
        ));
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.current_color = color;
        self.commands_executed
            .push(format!("SET_PIXEL x={} y={} color={:08X}", x, y, color));
    }

    pub fn draw_rectangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.commands_executed
            .push(format!("DRAW_RECT x1={} y1={} x2={} y2={}", x1, y1, x2, y2));
    }
}

impl Default for Win32Gdi {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulates a Dynamic Link Library (DLL) loadable module with symbol tables
#[derive(Debug, Clone)]
pub struct DllModule {
    pub name: String,
    pub exported_symbols: HashMap<String, u64>, // Symbol Name -> virtual address offset
    pub is_loaded: bool,
    pub dll_main_called: bool,
}

impl DllModule {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            exported_symbols: HashMap::new(),
            is_loaded: false,
            dll_main_called: false,
        }
    }

    pub fn register_export(&mut self, symbol: &str, address: u64) {
        self.exported_symbols.insert(symbol.to_string(), address);
    }
}

/// Dynamic Link Library (DLL) loader (Windows ABI compatibility)
#[derive(Debug, Clone)]
pub struct DllLoader {
    pub loaded_dlls: HashMap<String, DllModule>,
}

impl DllLoader {
    pub fn new() -> Self {
        Self {
            loaded_dlls: HashMap::new(),
        }
    }

    pub fn register_dll(&mut self, module: DllModule) {
        self.loaded_dlls.insert(module.name.clone(), module);
    }

    pub fn load_library(&mut self, name: &str) -> Result<&mut DllModule, &'static str> {
        let dll = self.loaded_dlls.get_mut(name).ok_or("DLL not found")?;
        dll.is_loaded = true;
        dll.dll_main_called = true; // DllMain entrypoint execution
        Ok(dll)
    }

    pub fn get_proc_address(
        &self,
        library_name: &str,
        symbol_name: &str,
    ) -> Result<u64, &'static str> {
        let dll = self
            .loaded_dlls
            .get(library_name)
            .ok_or("Library not loaded")?;
        if !dll.is_loaded {
            return Err("Library has not been loaded");
        }
        dll.exported_symbols
            .get(symbol_name)
            .copied()
            .ok_or("Symbol not found in exports table")
    }
}

impl Default for DllLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents translated POSIX Linux syscall codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxSyscall {
    Fork = 57,
    Execve = 59,
    Mmap = 9,
}

/// Linux POSIX syscall compatibility translation mapping layer (WSL-style)
#[derive(Debug, Clone)]
pub struct PosixTranslation {
    pub translation_log: Vec<String>,
}

impl PosixTranslation {
    pub fn new() -> Self {
        Self {
            translation_log: Vec::new(),
        }
    }

    /// Maps a Linux syscall and its params into SigmaOS capability checks
    pub fn translate_syscall(
        &mut self,
        syscall: LinuxSyscall,
        arguments: &[u64],
    ) -> Result<&'static str, &'static str> {
        match syscall {
            LinuxSyscall::Fork => {
                self.translation_log
                    .push("TRANSLATE fork() -> spawn process capability".to_string());
                Ok("SigmaOS SpawnProcessCapability granted")
            }
            LinuxSyscall::Execve => {
                if arguments.is_empty() {
                    return Err("Execve requires executable path argument");
                }
                self.translation_log
                    .push(format!("TRANSLATE execve() path_ptr={}", arguments[0]));
                Ok("SigmaOS ExecuteProgramCapability granted")
            }
            LinuxSyscall::Mmap => {
                self.translation_log
                    .push("TRANSLATE mmap() -> virtual page mapping capability".to_string());
                Ok("SigmaOS VirtualMemoryCapability granted")
            }
        }
    }
}

impl Default for PosixTranslation {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// NetBSD/FreeBSD kqueue & kevent event notification framework
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KFilter {
    Read,
    Write,
    Signal,
    Vnode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KEvent {
    pub ident: uptr,     // File descriptor, process ID, or signal number
    pub filter: KFilter, // Event filter
    pub flags: u16,      // Event flags (e.g., EV_ADD, EV_DELETE, EV_ENABLE, EV_DISABLE)
    pub fflags: u32,     // Filter-specific flags
    pub data: iptr,      // Filter-specific data value
    pub udata: uptr,     // Opaque user-defined data
}

pub type uptr = usize;
pub type iptr = isize;

/// BSD kqueue event notifications manager
pub struct KQueue {
    pub registry: HashMap<(uptr, KFilter), KEvent>,
    pub active_events: Vec<KEvent>,
}

impl KQueue {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            active_events: Vec::new(),
        }
    }

    /// Registers a new event query to watch
    pub fn kevent_register(&mut self, event: KEvent) {
        let key = (event.ident, event.filter);
        self.registry.insert(key, event);
    }

    /// Triggers a matched notification (used by kernel triggers like socket rx/tx or file modifications)
    pub fn trigger_event(&mut self, ident: uptr, filter: KFilter, data: iptr) -> bool {
        let key = (ident, filter);
        if let Some(event) = self.registry.get(&key) {
            let mut active_event = *event;
            active_event.data = data;
            self.active_events.push(active_event);
            true
        } else {
            false
        }
    }

    /// Polls/reaps next active event
    pub fn kevent_poll(&mut self) -> Option<KEvent> {
        if self.active_events.is_empty() {
            None
        } else {
            Some(self.active_events.remove(0))
        }
    }
}

impl Default for KQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================================
// FreeBSD GEOM block storage layered topology framework
// ==========================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomType {
    Disk,
    PartitionTable, // e.g. GPT/MBR
    Mirror,         // e.g. RAID1
    Encryption,     // e.g. geli / crypt
    Label,          // e.g. ufs label
}

#[derive(Debug, Clone)]
pub struct GeomProvider {
    pub name: String,
    pub geom_type: GeomType,
    pub sector_size: u32,
    pub total_sectors: u64,
}

#[derive(Debug, Clone)]
pub struct GeomConsumer {
    pub name: String,
    pub attached_provider_name: String,
}

/// Dynamic stackable GEOM storage controller
pub struct GeomTopology {
    pub providers: HashMap<String, GeomProvider>,
    pub consumers: Vec<GeomConsumer>,
}

impl GeomTopology {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            consumers: Vec::new(),
        }
    }

    /// Register a virtual or hardware storage provider node
    pub fn register_provider(&mut self, provider: GeomProvider) {
        self.providers.insert(provider.name.clone(), provider);
    }

    /// Attaches a consumer layer to a provider to stack virtualization
    pub fn attach_consumer(&mut self, consumer: GeomConsumer) -> Result<(), &'static str> {
        if !self.providers.contains_key(&consumer.attached_provider_name) {
            return Err("GEOM: Target provider not found in active topology");
        }
        self.consumers.push(consumer);
        Ok(())
    }

    /// Checks if a provider has any attached consumers (stacked layering)
    pub fn is_provider_stacked(&self, provider_name: &str) -> bool {
        self.consumers.iter().any(|c| c.attached_provider_name == provider_name)
    }
}

impl Default for GeomTopology {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows_registry_hives() {
        let mut reg = WindowsRegistry::new("HKLM");

        let path = "Software\\SigmaOS\\VFX";
        assert!(reg
            .set_value(
                path,
                "RenderEngine",
                RegistryType::String,
                b"SovereignCompositor"
            )
            .is_ok());

        let val = reg.get_value(path, "RenderEngine").unwrap();
        assert_eq!(val.val_type, RegistryType::String);
        assert_eq!(val.data, b"SovereignCompositor".to_vec());
        assert_eq!(reg.transaction_log.len(), 1);
    }

    #[test]
    fn test_win32_gdi_device_contexts() {
        let mut dc = Win32Gdi::new();
        dc.select_object(GdiObjectType::Pen, 1001);
        dc.set_pixel(100, 200, 0x00FF0000);
        dc.draw_rectangle(0, 0, 50, 50);

        assert_eq!(dc.active_objects.get(&GdiObjectType::Pen), Some(&1001));
        assert_eq!(dc.current_color, 0x00FF0000);
        assert_eq!(dc.commands_executed.len(), 3);
        assert!(dc.commands_executed[1].contains("SET_PIXEL"));
    }

    #[test]
    fn test_dll_module_loader() {
        let mut loader = DllLoader::new();

        let mut user32 = DllModule::new("user32.dll");
        user32.register_export("CreateWindowExA", 0x7FFE0010);
        user32.register_export("MessageBoxA", 0x7FFE0040);
        loader.register_dll(user32);

        // Fail to resolve before library load
        assert!(loader
            .get_proc_address("user32.dll", "MessageBoxA")
            .is_err());

        // Load library
        let loaded = loader.load_library("user32.dll").unwrap();
        assert!(loaded.is_loaded);
        assert!(loaded.dll_main_called);

        // Resolve symbol
        let addr = loader
            .get_proc_address("user32.dll", "MessageBoxA")
            .unwrap();
        assert_eq!(addr, 0x7FFE0040);
    }

    #[test]
    fn test_posix_syscall_translations() {
        let mut translation = PosixTranslation::new();

        let fork_res = translation.translate_syscall(LinuxSyscall::Fork, &[]);
        assert_eq!(fork_res, Ok("SigmaOS SpawnProcessCapability granted"));

        let mmap_res = translation.translate_syscall(LinuxSyscall::Mmap, &[]);
        assert_eq!(mmap_res, Ok("SigmaOS VirtualMemoryCapability granted"));

        let exec_fail = translation.translate_syscall(LinuxSyscall::Execve, &[]);
        assert!(exec_fail.is_err());

        let exec_success = translation.translate_syscall(LinuxSyscall::Execve, &[0x401000]);
        assert!(exec_success.is_ok());
    }

    #[test]
    fn test_bsd_kqueue_and_kevent() {
        let mut kq = KQueue::new();
        let ev = KEvent {
            ident: 10, // file descriptor 10
            filter: KFilter::Read,
            flags: 1, // EV_ADD
            fflags: 0,
            data: 0,
            udata: 0xDEADBEEF,
        };

        // Register event to watch
        kq.kevent_register(ev);

        // Trigger action representing file descriptor becoming readable with 12 bytes of data
        assert!(kq.trigger_event(10, KFilter::Read, 12));
        assert!(!kq.trigger_event(11, KFilter::Read, 12)); // unregistered

        // Poll and verify reaped event
        let reaped = kq.kevent_poll().unwrap();
        assert_eq!(reaped.ident, 10);
        assert_eq!(reaped.filter, KFilter::Read);
        assert_eq!(reaped.data, 12);
        assert_eq!(reaped.udata, 0xDEADBEEF);
        assert!(kq.kevent_poll().is_none());
    }

    #[test]
    fn test_freebsd_geom_storage() {
        let mut geom = GeomTopology::new();

        // Register base disk
        geom.register_provider(GeomProvider {
            name: "ada0".to_string(),
            geom_type: GeomType::Disk,
            sector_size: 512,
            total_sectors: 1000000,
        });

        // Try attaching consumer to nonexistent base provider - fails
        let bad_consumer = GeomConsumer {
            name: "gpt_part1".to_string(),
            attached_provider_name: "ada1".to_string(),
        };
        assert!(geom.attach_consumer(bad_consumer).is_err());

        // Attach consumer to active provider - succeeds
        let consumer = GeomConsumer {
            name: "gpt_part1".to_string(),
            attached_provider_name: "ada0".to_string(),
        };
        assert!(geom.attach_consumer(consumer).is_ok());

        // Check topology stacked properties
        assert!(geom.is_provider_stacked("ada0"));
        assert!(!geom.is_provider_stacked("ada1"));
    }
}
