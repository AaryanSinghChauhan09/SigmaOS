/// Systemd-Grade Init and Target State Engine for SigmaOS
/// Provides robust target dependency graphs, wants/requires properties,
/// and target states to defeat Fedora's Systemd initialization.
use core::sync::atomic::{AtomicUsize, Ordering};

pub type UnitID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Service,
    Target,
    Socket,
    Timer,
    Path,
    Mount,
    Device,
}

/// Alternative Init System types (Artix & Devuan parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitSystemType {
    SigmaInit, // Default SigmaOS Init
    Runit,
    S6,
    Dinit,
    Sysvinit,
    OpenRC,
}

/// FreeBSD & OpenBSD rc.d boot execution ordering level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsdRcOrder {
    EarlyBoot, // e.g. REQUIRE: mountcritlocal, BEFORE: DAEMON
    CoreBoot,  // e.g. REQUIRE: NETWORKING, BEFORE: LOGIN
    LateBoot,  // e.g. REQUIRE: LOGIN
}

/// Multi-init abstraction bridge allowing boot-time switching across Linux & BSD init models
pub struct InitSystemBridge {
    pub active_init: InitSystemType,
}

impl InitSystemBridge {
    pub fn new(init_type: InitSystemType) -> Self {
        Self {
            active_init: init_type,
        }
    }

    pub fn convert_runit_service_script(&self, service_name: &str) -> Vec<u8> {
        let mut script = Vec::new();
        for &b in b"#!/bin/sh\nexec " {
            script.push(b);
        }
        for &b in service_name.as_bytes() {
            script.push(b);
        }
        for &b in b" --foreground\n" {
            script.push(b);
        }
        script
    }

    /// Converts service configuration to OpenRC runlevel script format (Gentoo/Alpine parity)
    pub fn convert_openrc_service_script(&self, service_name: &str, runlevel: &str) -> Vec<u8> {
        let mut script = Vec::new();
        script.extend_from_slice(b"#!/sbin/openrc-run\n# OpenRC Service Script for ");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\nrunlevel=");
        script.extend_from_slice(runlevel.as_bytes());
        script.extend_from_slice(b"\ncommand=\"/usr/bin/");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\"\ncommand_args=\"--daemon\"\n");
        script
    }

    /// Converts service configuration to FreeBSD / OpenBSD rc.d script format (BSD rc.d parity)
    pub fn convert_bsd_rc_script(&self, service_name: &str, order: BsdRcOrder) -> Vec<u8> {
        let mut script = Vec::new();
        script.extend_from_slice(b"#!/bin/sh\n# PROVIDE: ");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\n");
        match order {
            BsdRcOrder::EarlyBoot => {
                script.extend_from_slice(b"# REQUIRE: mountcritlocal\n# BEFORE: DAEMON\n")
            }
            BsdRcOrder::CoreBoot => {
                script.extend_from_slice(b"# REQUIRE: NETWORKING\n# BEFORE: LOGIN\n")
            }
            BsdRcOrder::LateBoot => script.extend_from_slice(b"# REQUIRE: LOGIN\n"),
        }
        script.extend_from_slice(b"name=\"");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\"\nrcvar=\"");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"_enable\"\nload_rc_config $name\nrun_rc_command \"$1\"\n");
        script
    }
}

/// Socket kinds for Systemd socket activation (Linux systemd parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketKind {
    Stream,
    Datagram,
    SequentialPacket,
    UnixStream,
    UnixDatagram,
    Fifo,
}

/// Socket activation configuration for Systemd socket units
#[derive(Debug, Clone)]
pub struct SystemdSocketConfig {
    pub socket_id: UnitID,
    pub service_id: UnitID,
    pub listen_address: [u8; 64],
    pub listen_port: u16,
    pub socket_kind: SocketKind,
    pub accept_connection: bool,
    pub max_connections: usize,
    pub current_connections: usize,
    pub pass_fds: bool,
}

impl SystemdSocketConfig {
    pub fn new(
        socket_id: UnitID,
        service_id: UnitID,
        listen_address: &[u8],
        listen_port: u16,
        socket_kind: SocketKind,
    ) -> Self {
        let mut addr = [0u8; 64];
        let len = listen_address.len().min(63);
        addr[..len].copy_from_slice(&listen_address[..len]);
        Self {
            socket_id,
            service_id,
            listen_address: addr,
            listen_port,
            socket_kind,
            accept_connection: false,
            max_connections: 128,
            current_connections: 0,
            pass_fds: true,
        }
    }
}

/// Event triggering socket activation when incoming traffic is received
#[derive(Debug, Clone)]
pub struct SocketActivationEvent {
    pub socket_id: UnitID,
    pub remote_address: [u8; 64],
    pub remote_port: u16,
    pub file_descriptor: i32,
}

impl SocketActivationEvent {
    pub fn new(socket_id: UnitID, remote_address: &[u8], remote_port: u16, file_descriptor: i32) -> Self {
        let mut addr = [0u8; 64];
        let len = remote_address.len().min(63);
        addr[..len].copy_from_slice(&remote_address[..len]);
        Self {
            socket_id,
            remote_address: addr,
            remote_port,
            file_descriptor,
        }
    }
}

/// Socket activation manager handling listening sockets and service activation triggers
pub struct SystemdSocketActivationManager {
    pub sockets: Vec<SystemdSocketConfig>,
    pub active_fds: Vec<(UnitID, i32)>,
}

impl Default for SystemdSocketActivationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemdSocketActivationManager {
    pub fn new() -> Self {
        Self {
            sockets: Vec::new(),
            active_fds: Vec::new(),
        }
    }

    pub fn register_socket(&mut self, config: SystemdSocketConfig) {
        self.sockets.push(config);
    }

    pub fn bind_socket_fd(&mut self, socket_id: UnitID, fd: i32) -> Result<(), &'static str> {
        if self.sockets.iter().any(|s| s.socket_id == socket_id) {
            self.active_fds.push((socket_id, fd));
            Ok(())
        } else {
            Err("Socket unit not found in activation manager")
        }
    }

    pub fn process_incoming_event(
        &mut self,
        engine: &mut SystemdEngine,
        event: &SocketActivationEvent,
    ) -> Result<UnitID, &'static str> {
        let mut target_service = None;
        for s in self.sockets.iter_mut() {
            if s.socket_id == event.socket_id {
                s.current_connections = s.current_connections.saturating_add(1);
                target_service = Some(s.service_id);
                break;
            }
        }

        if let Some(service_id) = target_service {
            engine.trigger_socket_activation(event.socket_id)?;
            Ok(service_id)
        } else {
            Err("No service associated with incoming socket event")
        }
    }

    pub fn get_socket_fd(&self, socket_id: UnitID) -> Option<i32> {
        for &(id, fd) in self.active_fds.iter() {
            if id == socket_id {
                return Some(fd);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitState {
    Active,
    Inactive,
    Activating,
    Deactivating,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure,
}

#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub unit_id: UnitID,
    pub message: [u8; 64],
    pub from_state: UnitState,
    pub to_state: UnitState,
}

impl JournalEntry {
    pub fn new(unit_id: UnitID, msg: &[u8], from_state: UnitState, to_state: UnitState) -> Self {
        let mut msg_arr = [0u8; 64];
        let len = msg.len().min(63);
        msg_arr[..len].copy_from_slice(&msg[..len]);
        JournalEntry {
            unit_id,
            message: msg_arr,
            from_state,
            to_state,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemdUnit {
    pub id: UnitID,
    pub name: [u8; 32],
    pub unit_type: UnitType,
    pub state: UnitState,
    pub requires: Vec<UnitID>,
    pub wants: Vec<UnitID>,
    pub before: Vec<UnitID>,
    pub after: Vec<UnitID>,
    pub conflicts: Vec<UnitID>,
    pub binds_to: Vec<UnitID>,
    pub part_of: Vec<UnitID>,
    pub restart_policy: RestartPolicy,
    pub restart_count: usize,
    pub startup_time_ms: u64,
    pub duration_ms: u64,
    pub is_enabled: bool,
    pub triggered_unit: Option<UnitID>,
}

impl SystemdUnit {
    pub fn new(id: UnitID, name: &[u8], unit_type: UnitType) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);
        SystemdUnit {
            id,
            name: name_arr,
            unit_type,
            state: UnitState::Inactive,
            requires: Vec::new(),
            wants: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            conflicts: Vec::new(),
            binds_to: Vec::new(),
            part_of: Vec::new(),
            restart_policy: RestartPolicy::No,
            restart_count: 0,
            startup_time_ms: 0,
            duration_ms: 0,
            is_enabled: true,
            triggered_unit: None,
        }
    }
}

pub struct SystemdEngine {
    pub units: Vec<SystemdUnit>,
    pub current_target: AtomicUsize, // stores UnitID of active target
    pub journal: Vec<JournalEntry>,
}

impl Default for SystemdEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemdEngine {
    pub fn new() -> Self {
        SystemdEngine {
            units: Vec::new(),
            current_target: AtomicUsize::new(0),
            journal: Vec::new(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
    }

    pub fn find_unit(&self, id: UnitID) -> Option<&SystemdUnit> {
        for unit in self.units.iter() {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn find_unit_mut(&mut self, id: UnitID) -> Option<&mut SystemdUnit> {
        for unit in self.units.iter_mut() {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn log_journal(
        &mut self,
        unit_id: UnitID,
        message: &[u8],
        from_state: UnitState,
        to_state: UnitState,
    ) {
        let entry = JournalEntry::new(unit_id, message, from_state, to_state);
        self.journal.push(entry);
    }

    pub fn isolate_target(&mut self, target_id: UnitID) -> Result<(), &'static str> {
        let mut target_idx = None;
        for (i, unit) in self.units.iter().enumerate() {
            if unit.id == target_id && unit.unit_type == UnitType::Target {
                target_idx = Some(i);
                break;
            }
        }

        let _idx = target_idx.ok_or("Target unit not found")?;

        self.units[_idx].state = UnitState::Activating;

        let mut active_set = Vec::new();
        self.collect_dependencies(target_id, &mut active_set);

        for unit in self.units.iter_mut() {
            if !active_set.contains(&unit.id) && unit.id != target_id {
                unit.state = UnitState::Inactive;
            } else if active_set.contains(&unit.id) {
                unit.state = UnitState::Active;
            }
        }

        self.units[_idx].state = UnitState::Active;
        self.current_target.store(target_id, Ordering::SeqCst);
        Ok(())
    }

    fn collect_dependencies(&self, unit_id: UnitID, set: &mut Vec<UnitID>) {
        if set.contains(&unit_id) {
            return;
        }

        for unit in self.units.iter() {
            if unit.id == unit_id {
                for &req in unit.requires.iter() {
                    set.push(req);
                    self.collect_dependencies(req, set);
                }
                for &want in unit.wants.iter() {
                    set.push(want);
                    self.collect_dependencies(want, set);
                }
            }
        }
    }

    pub fn get_active_target_id(&self) -> usize {
        self.current_target.load(Ordering::SeqCst)
    }

    pub fn topological_sort(
        &self,
        unit_ids: &SystemdVec<UnitID>,
    ) -> Result<SystemdVec<UnitID>, &'static str> {
        let mut sorted = Vec::new();
        let mut visiting = Vec::new();
        let mut visited = Vec::new();

        for &id in unit_ids.iter() {
            if !visited.contains(&id) {
                self.topo_visit(id, unit_ids.as_slice(), &mut sorted, &mut visiting, &mut visited)?;
            }
        }
        Ok(SystemdVec::from(sorted.as_slice()))
    }

    fn topo_visit(
        &self,
        id: UnitID,
        all_ids: &[UnitID],
        sorted: &mut Vec<UnitID>,
        visiting: &mut Vec<UnitID>,
        visited: &mut Vec<UnitID>,
    ) -> Result<(), &'static str> {
        if visiting.contains(&id) {
            return Err("Dependency cycle detected");
        }
        if visited.contains(&id) {
            return Ok(());
        }

        visiting.push(id);

        let mut prereqs = Vec::new();

        if let Some(unit) = self.find_unit(id) {
            for &after_id in unit.after.iter() {
                if all_ids.contains(&after_id) && !prereqs.contains(&after_id) {
                    prereqs.push(after_id);
                }
            }
        }

        for &other_id in all_ids.iter() {
            if let Some(other_unit) = self.find_unit(other_id) {
                if other_unit.before.contains(&id) && !prereqs.contains(&other_id) {
                    prereqs.push(other_id);
                }
            }
        }

        for p_id in prereqs {
            if !visited.contains(&p_id) {
                self.topo_visit(p_id, all_ids, sorted, visiting, visited)?;
            }
        }

        visiting.retain(|&x| x != id);
        visited.push(id);
        sorted.push(id);
        Ok(())
    }

    pub fn systemctl_start(&mut self, id: UnitID) -> Result<(), &'static str> {
        let (is_enabled, conflicts, requires, wants) = if let Some(u) = self.find_unit(id) {
            (
                u.is_enabled,
                u.conflicts.clone(),
                u.requires.clone(),
                u.wants.clone(),
            )
        } else {
            return Err("Unit not found");
        };

        if !is_enabled {
            return Err("Unit is disabled");
        }

        for &conflict_id in conflicts.iter() {
            if let Some(conf_unit) = self.find_unit(conflict_id) {
                if conf_unit.state == UnitState::Active {
                    self.systemctl_stop(conflict_id)?;
                }
            }
        }

        let mut units_to_stop = Vec::new();
        for u in self.units.iter() {
            if u.conflicts.contains(&id) && u.state == UnitState::Active {
                units_to_stop.push(u.id);
            }
        }
        for &stop_id in units_to_stop.iter() {
            self.systemctl_stop(stop_id)?;
        }

        for &req_id in requires.iter() {
            if self.systemctl_start(req_id).is_err() {
                if let Some(u) = self.find_unit_mut(id) {
                    u.state = UnitState::Failed;
                }
                self.log_journal(
                    id,
                    b"Required dependency failed to start",
                    UnitState::Inactive,
                    UnitState::Failed,
                );
                return Err("Required dependency failed to start");
            }
        }

        for &want_id in wants.iter() {
            let _ = self.systemctl_start(want_id);
        }

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Activating;
        }
        self.log_journal(
            id,
            b"Unit starting/activating",
            UnitState::Inactive,
            UnitState::Activating,
        );

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Active;
            u.startup_time_ms = 100;
            u.duration_ms = 150;
        }
        self.log_journal(
            id,
            b"Unit started successfully",
            UnitState::Activating,
            UnitState::Active,
        );

        Ok(())
    }

    pub fn systemctl_stop(&mut self, id: UnitID) -> Result<(), &'static str> {
        let exists = self.find_unit(id).is_some();
        if !exists {
            return Err("Unit not found");
        }

        if let Some(u) = self.find_unit_mut(id) {
            if u.state == UnitState::Inactive {
                return Ok(());
            }
            u.state = UnitState::Deactivating;
        }
        self.log_journal(
            id,
            b"Unit stopping/deactivating",
            UnitState::Active,
            UnitState::Deactivating,
        );

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Inactive;
        }
        self.log_journal(
            id,
            b"Unit stopped successfully",
            UnitState::Deactivating,
            UnitState::Inactive,
        );

        let mut bound_units = Vec::new();
        for u in self.units.iter() {
            if u.binds_to.contains(&id) && u.state == UnitState::Active {
                bound_units.push(u.id);
            }
        }

        for &b_id in bound_units.iter() {
            self.systemctl_stop(b_id)?;
        }

        Ok(())
    }

    pub fn systemctl_restart(&mut self, id: UnitID) -> Result<(), &'static str> {
        self.systemctl_stop(id)?;
        self.systemctl_start(id)?;
        Ok(())
    }

    pub fn systemctl_enable(&mut self, id: UnitID) -> Result<(), &'static str> {
        if let Some(u) = self.find_unit_mut(id) {
            u.is_enabled = true;
            Ok(())
        } else {
            Err("Unit not found")
        }
    }

    pub fn systemctl_disable(&mut self, id: UnitID) -> Result<(), &'static str> {
        if let Some(u) = self.find_unit_mut(id) {
            u.is_enabled = false;
            Ok(())
        } else {
            Err("Unit not found")
        }
    }

    pub fn systemctl_reload(&mut self, id: UnitID) -> Result<(), &'static str> {
        let (state, is_active) = if let Some(u) = self.find_unit(id) {
            (u.state, u.state == UnitState::Active)
        } else {
            return Err("Unit not found");
        };
        if !is_active {
            return Err("Cannot reload inactive unit");
        }

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Activating;
        }
        self.log_journal(
            id,
            b"Reloading unit configuration",
            state,
            UnitState::Activating,
        );

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Active;
        }
        self.log_journal(
            id,
            b"Unit reloaded successfully",
            UnitState::Activating,
            UnitState::Active,
        );
        Ok(())
    }

    pub fn systemctl_status(&self, id: UnitID) -> Option<UnitState> {
        self.find_unit(id).map(|u| u.state)
    }

    pub fn handle_unit_failure(&mut self, id: UnitID) -> Result<bool, &'static str> {
        let mut should_restart = false;
        let (policy, count) = if let Some(unit) = self.find_unit(id) {
            (unit.restart_policy, unit.restart_count)
        } else {
            return Err("Unit not found");
        };

        if policy == RestartPolicy::Always || policy == RestartPolicy::OnFailure {
            if count < 3 {
                should_restart = true;
            }
        }

        if should_restart {
            if let Some(unit) = self.find_unit_mut(id) {
                unit.restart_count += 1;
                unit.state = UnitState::Activating;
            }
            self.log_journal(
                id,
                b"Auto-restarting failed unit based on restart policy",
                UnitState::Failed,
                UnitState::Activating,
            );
            self.systemctl_start(id)?;
            Ok(true)
        } else {
            if let Some(unit) = self.find_unit_mut(id) {
                unit.state = UnitState::Failed;
            }
            self.log_journal(
                id,
                b"Unit entered Failed state, restart policy not met or limit exceeded",
                UnitState::Inactive,
                UnitState::Failed,
            );
            Ok(false)
        }
    }

    pub fn trigger_socket_activation(&mut self, socket_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(socket_id) {
            if unit.unit_type != UnitType::Socket {
                return Err("Unit is not a Socket");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(
                socket_id,
                b"Socket activation triggered",
                UnitState::Active,
                UnitState::Active,
            );
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this socket")
        }
    }

    pub fn trigger_path_activation(&mut self, path_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(path_id) {
            if unit.unit_type != UnitType::Path {
                return Err("Unit is not a Path");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(
                path_id,
                b"Path modification activation triggered",
                UnitState::Active,
                UnitState::Active,
            );
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this path")
        }
    }

    pub fn trigger_timer_activation(&mut self, timer_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(timer_id) {
            if unit.unit_type != UnitType::Timer {
                return Err("Unit is not a Timer");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(
                timer_id,
                b"Timer activation triggered",
                UnitState::Active,
                UnitState::Active,
            );
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this timer")
        }
    }

    pub fn systemd_analyze_blame(&self) -> SystemdVec<(UnitID, u64)> {
        let mut blame_list = Vec::new();
        for unit in self.units.iter() {
            if unit.state == UnitState::Active {
                blame_list.push((unit.id, unit.duration_ms));
            }
        }
        for i in 0..blame_list.len() {
            for j in 0..blame_list.len() - 1 - i {
                if blame_list[j].1 < blame_list[j + 1].1 {
                    let temp = blame_list[j].clone();
                    blame_list[j] = blame_list[j + 1].clone();
                    blame_list[j + 1] = temp;
                }
            }
        }
        SystemdVec::from(blame_list)
    }

    pub fn query_target_by_name(&self, name: &[u8]) -> Option<UnitID> {
        for unit in self.units.iter() {
            if unit.unit_type == UnitType::Target {
                let len = name.len().min(32);
                if unit.name[..len] == name[..len] {
                    return Some(unit.id);
                }
            }
        }
        None
    }
}

pub struct SystemdVec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> SystemdVec<T> {
    pub fn as_slice(&self) -> &[T] {
        if self.data.is_null() || self.len == 0 {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::Deref for SystemdVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SystemdVec<T> {
    fn from(slice: &'a [T]) -> Self {
        let mut vec = Self::new();
        for item in slice {
            vec.push(item.clone());
        }
        vec
    }
}

impl<T> From<Vec<T>> for SystemdVec<T> {
    fn from(v: Vec<T>) -> Self {
        let mut vec = Self::new();
        for item in v {
            vec.push(item);
        }
        vec
    }
}

impl<T: PartialEq> SystemdVec<T> {
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item {
                    return true;
                }
            }
        }
        false
    }
}

impl<T: Clone> Clone for SystemdVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = SystemdVec::new();
        for item in self.iter() {
            new_vec.push(item.clone());
        }
        new_vec
    }
}

impl<T> SystemdVec<T> {
    pub fn new() -> Self {
        SystemdVec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut write_idx = 0;
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    if write_idx != i {
                        core::ptr::copy_nonoverlapping(
                            self.data.add(i),
                            self.data.add(write_idx),
                            1,
                        );
                    }
                    write_idx += 1;
                } else {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
        }
        self.len = write_idx;
    }

    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            if self.capacity > 0 && !self.data.is_null() {
                for i in 0..self.len {
                    core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
                }
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for SystemdVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for SystemdVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for SystemdVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

pub struct VecIter<'a, T> {
    vec: &'a SystemdVec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_engine_dependency_isolation() {
        let mut engine = SystemdEngine::new();

        let mut graphical = SystemdUnit::new(100, b"graphical.target", UnitType::Target);
        graphical.requires.push(200);
        graphical.wants.push(300);

        let multi_user = SystemdUnit::new(200, b"multi-user.target", UnitType::Target);
        let network = SystemdUnit::new(300, b"network.target", UnitType::Target);
        let service = SystemdUnit::new(400, b"apache.service", UnitType::Service);

        engine.register_unit(graphical);
        engine.register_unit(multi_user);
        engine.register_unit(network);
        engine.register_unit(service);

        engine.isolate_target(100).unwrap();

        assert_eq!(engine.get_active_target_id(), 100);

        for unit in engine.units.iter() {
            if unit.id == 200 || unit.id == 300 || unit.id == 100 {
                assert_eq!(unit.state, UnitState::Active);
            } else if unit.id == 400 {
                assert_eq!(unit.state, UnitState::Inactive);
            }
        }
    }

    #[test]
    fn test_systemd_topological_sort_and_cycle_detection() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.before.push(2);

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.before.push(3);

        let c = SystemdUnit::new(3, b"c.service", UnitType::Service);

        engine.register_unit(a);
        engine.register_unit(b);
        engine.register_unit(c);

        let mut ids = SystemdVec::new();
        ids.push(3);
        ids.push(2);
        ids.push(1);

        let sorted = engine.topological_sort(&ids).unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 3);

        let mut engine_cycle = SystemdEngine::new();
        let mut u1 = SystemdUnit::new(10, b"u1.service", UnitType::Service);
        u1.before.push(20);
        let mut u2 = SystemdUnit::new(20, b"u2.service", UnitType::Service);
        u2.before.push(10);

        engine_cycle.register_unit(u1);
        engine_cycle.register_unit(u2);

        let mut cycle_ids = SystemdVec::new();
        cycle_ids.push(10);
        cycle_ids.push(20);

        assert!(engine_cycle.topological_sort(&cycle_ids).is_err());
    }

    #[test]
    fn test_systemd_conflicts_and_binds_to() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.conflicts.push(2);

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.state = UnitState::Active;

        let mut c = SystemdUnit::new(3, b"c.service", UnitType::Service);
        c.binds_to.push(1);
        c.state = UnitState::Active;

        engine.register_unit(a);
        engine.register_unit(b);
        engine.register_unit(c);

        engine.systemctl_start(1).unwrap();

        assert_eq!(engine.systemctl_status(1), Some(UnitState::Active));
        assert_eq!(engine.systemctl_status(2), Some(UnitState::Inactive));

        engine.systemctl_stop(1).unwrap();
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Inactive));
        assert_eq!(engine.systemctl_status(3), Some(UnitState::Inactive));
    }

    #[test]
    fn test_systemd_activation_triggers() {
        let mut engine = SystemdEngine::new();

        let mut socket = SystemdUnit::new(1, b"test.socket", UnitType::Socket);
        socket.triggered_unit = Some(10);

        let srv = SystemdUnit::new(10, b"test.service", UnitType::Service);

        engine.register_unit(socket);
        engine.register_unit(srv);

        assert_eq!(engine.systemctl_status(10), Some(UnitState::Inactive));

        engine.trigger_socket_activation(1).unwrap();
        assert_eq!(engine.systemctl_status(10), Some(UnitState::Active));
    }

    #[test]
    fn test_systemd_restart_policy() {
        let mut engine = SystemdEngine::new();

        let mut fail_srv = SystemdUnit::new(1, b"fail.service", UnitType::Service);
        fail_srv.restart_policy = RestartPolicy::Always;

        engine.register_unit(fail_srv);

        let restarted = engine.handle_unit_failure(1).unwrap();
        assert!(restarted);
        assert_eq!(engine.find_unit(1).unwrap().restart_count, 1);
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Active));
    }

    #[test]
    fn test_systemd_analyze_blame() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.state = UnitState::Active;
        a.duration_ms = 500;

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.state = UnitState::Active;
        b.duration_ms = 1200;

        engine.register_unit(a);
        engine.register_unit(b);

        let blame = engine.systemd_analyze_blame();
        assert_eq!(blame.len(), 2);
        assert_eq!(blame[0].0, 2);
        assert_eq!(blame[0].1, 1200);
        assert_eq!(blame[1].0, 1);
        assert_eq!(blame[1].1, 500);
    }

    #[test]
    fn test_advanced_dependency_sorting() {
        let mut engine = SystemdEngine::new();

        let mut basic = SystemdUnit::new(1, b"basic.target", UnitType::Target);
        basic.before.push(2);

        let mut network = SystemdUnit::new(2, b"network.target", UnitType::Target);
        network.before.push(3);

        let multi_user = SystemdUnit::new(3, b"multi-user.target", UnitType::Target);

        engine.register_unit(basic);
        engine.register_unit(network);
        engine.register_unit(multi_user);

        let mut ids = SystemdVec::new();
        ids.push(3);
        ids.push(2);
        ids.push(1);

        let sorted = engine.topological_sort(&ids).unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 3);
    }

    #[test]
    fn test_systemctl_management() {
        let mut engine = SystemdEngine::new();
        let target = SystemdUnit::new(99, b"multi-user.target", UnitType::Target);
        engine.register_unit(target);

        let queried = engine.query_target_by_name(b"multi-user.target");
        assert_eq!(queried, Some(99));

        let non_existent = engine.query_target_by_name(b"non-existent.target");
        assert_eq!(non_existent, None);
    }

    #[test]
    fn test_alternative_init_bridge() {
        let bridge = InitSystemBridge::new(InitSystemType::Runit);
        assert_eq!(bridge.active_init, InitSystemType::Runit);

        let runit_script = bridge.convert_runit_service_script("apache2");
        assert!(&runit_script[..].starts_with(b"#!/bin/sh\nexec apache2 --foreground\n"));
    }

    #[test]
    fn test_socket_activation_manager() {
        let mut engine = SystemdEngine::new();
        let mut socket_mgr = SystemdSocketActivationManager::new();

        let mut socket_unit = SystemdUnit::new(10, b"httpd.socket", UnitType::Socket);
        socket_unit.triggered_unit = Some(20);
        let srv_unit = SystemdUnit::new(20, b"httpd.service", UnitType::Service);

        engine.register_unit(socket_unit);
        engine.register_unit(srv_unit);

        let socket_cfg = SystemdSocketConfig::new(
            10,
            20,
            b"0.0.0.0",
            80,
            SocketKind::Stream,
        );
        socket_mgr.register_socket(socket_cfg);

        socket_mgr.bind_socket_fd(10, 5).unwrap();
        assert_eq!(socket_mgr.get_socket_fd(10), Some(5));

        let event = SocketActivationEvent::new(10, b"192.168.1.50", 54321, 5);
        let triggered_srv = socket_mgr.process_incoming_event(&mut engine, &event).unwrap();
        assert_eq!(triggered_srv, 20);
        assert_eq!(engine.systemctl_status(20), Some(UnitState::Active));
    }
}
