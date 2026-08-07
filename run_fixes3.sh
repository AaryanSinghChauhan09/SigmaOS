#!/bin/bash
sed -i 's/for i in 0..16/for _ in 0..16/g' src/debugger/breakpoint.rs
sed -i 's/pub fn execute_action(&mut self, pid: u32, action: ProcessAction)/pub fn execute_action(\&mut self, _pid: u32, _action: ProcessAction)/g' src/dashboard/process.rs
sed -i 's/fn collect_coredump(&mut self, pid: usize)/fn collect_coredump(\&mut self, _pid: usize)/g' src/crash/reporting.rs
sed -i 's/fn load(&mut self, data: &\[u8\])/fn load(\&mut self, _data: \&[u8])/g' src/config/manager.rs
sed -i 's/fn set_config(&mut self, key: &\[u8\], value: &\[u8\])/fn set_config(\&mut self, _key: \&[u8], _value: \&[u8])/g' src/config/loader.rs
sed -i 's/pub fn replay_legacy_build(&self, recipe_name: &str)/pub fn replay_legacy_build(\&self, _recipe_name: \&str)/g' src/compatibility/constellation.rs
sed -i 's/fn upload(&mut self, local_path: &\[u8\], remote_path: &\[u8\])/fn upload(\&mut self, _local_path: \&[u8], _remote_path: \&[u8])/g' src/cloud/storage.rs
sed -i 's/for &(id, _, snapshot_type) in/for \&(_id, _, _snapshot_type) in/g' src/backup/snapshot.rs
sed -i 's/fn execute(&mut self, action: &SystemAction)/fn execute(\&mut self, _action: \&SystemAction)/g' src/automation/orchestrator.rs
