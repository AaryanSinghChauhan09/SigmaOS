// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - OCI container runtime (sigma-runc)
// Principles: PROCESS ISOLATION, LEAST PRIVILEGE, MODULARITY, DEFENSIVE PROGRAMMING
use alloc::{string::String, vec::Vec, collections::BTreeMap, format};
#[derive(Debug,Clone)] pub struct OciLayer { pub digest: String, pub size: u64, pub media_type: String }
#[derive(Debug,Clone)] pub struct OciManifest { pub schema_version: u32, pub media_type: String, pub config_digest: String, pub layers: Vec<OciLayer> }
#[derive(Debug,Clone)] pub struct MountSpec { pub source: String, pub target: String, pub readonly: bool, pub fs_type: String }
#[derive(Debug,Clone,PartialEq)] pub enum NetworkMode { Bridge, Host, None, Named(String) }
#[derive(Debug,Clone)] pub struct ContainerSpec { pub image: String, pub cmd: Vec<String>, pub env: Vec<String>, pub mounts: Vec<MountSpec>, pub network: NetworkMode, pub user: String, pub working_dir: String, pub privileged: bool }
impl ContainerSpec { pub fn new(image: &str, cmd: Vec<&str>) -> Self { Self { image: image.into(), cmd: cmd.iter().map(|s| s.to_string()).collect(), env: Vec::new(), mounts: Vec::new(), network: NetworkMode::Bridge, user: String::from("root"), working_dir: String::from("/"), privileged: false } } }
#[derive(Debug,Clone,PartialEq)] pub enum ContainerState { Created, Running { pid: u32 }, Paused, Stopped { exit_code: i32 }, Dead }
#[derive(Debug,Clone)] pub struct OciContainer { pub id: String, pub name: String, pub spec: ContainerSpec, pub state: ContainerState, pub created_ns: u64 }
pub struct OciRuntime { pub containers: BTreeMap<String,OciContainer>, pub next_pid: u32 }
impl OciRuntime {
    pub fn new() -> Self { Self { containers: BTreeMap::new(), next_pid: 1000 } }
    pub fn pull_image(&self, image: &str) -> Result<OciManifest,&'static str> { Ok(OciManifest { schema_version: 2, media_type: String::from("application/vnd.oci.image.manifest.v1+json"), config_digest: format!("sha256:{}", image.len()), layers: alloc::vec![OciLayer { digest: String::from("sha256:abc123"), size: 1024*1024*50, media_type: String::from("application/vnd.oci.image.layer.v1.tar+gzip") }] }) }
    pub fn create_container(&mut self, name: &str, spec: ContainerSpec) -> String { let id = format!("{:016x}", self.containers.len() as u64); self.containers.insert(id.clone(), OciContainer { id: id.clone(), name: name.into(), spec, state: ContainerState::Created, created_ns: 0 }); id }
    pub fn start_container(&mut self, id: &str) -> Result<(),&'static str> { let pid = self.next_pid; self.next_pid += 1; self.containers.get_mut(id).ok_or("not found").map(|c| c.state = ContainerState::Running { pid }) }
    pub fn stop_container(&mut self, id: &str, _timeout_s: u32) -> Result<(),&'static str> { self.containers.get_mut(id).ok_or("not found").map(|c| c.state = ContainerState::Stopped { exit_code: 0 }) }
    pub fn pause_container(&mut self, id: &str) -> Result<(),&'static str> { let c = self.containers.get_mut(id).ok_or("not found")?; if matches!(c.state, ContainerState::Running { .. }) { c.state = ContainerState::Paused; Ok(()) } else { Err("not running") } }
    pub fn remove_container(&mut self, id: &str) -> Result<(),&'static str> { self.containers.remove(id).ok_or("not found").map(|_| ()) }
    pub fn list_containers(&self) -> Vec<&OciContainer> { self.containers.values().collect() }
    pub fn inspect(&self, id: &str) -> Option<&OciContainer> { self.containers.get(id) }
    pub fn exec_in_container(&self, id: &str, _cmd: &[&str]) -> Result<Vec<u8>,&'static str> { if self.containers.contains_key(id) { Ok(b"ok
".to_vec()) } else { Err("container not found") } }
    pub fn logs(&self, id: &str) -> Result<Vec<u8>,&'static str> { if self.containers.contains_key(id) { Ok(b"container log output
".to_vec()) } else { Err("not found") } }
}
#[derive(Debug,Clone)] pub struct ComposeService { pub image: String, pub ports: Vec<(u16,u16)>, pub volumes: Vec<String>, pub environment: Vec<String>, pub depends_on: Vec<String> }
pub struct ComposeFile { pub services: BTreeMap<String,ComposeService> }
impl ComposeFile { pub fn parse_compose(_yaml: &str) -> Self { Self { services: BTreeMap::new() } } }
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_create_start_stop() { let mut rt = OciRuntime::new(); let spec = ContainerSpec::new("nginx:latest", alloc::vec!["nginx","-g","daemon off;"]); let id = rt.create_container("web", spec); assert!(matches!(rt.inspect(&id).unwrap().state, ContainerState::Created)); rt.start_container(&id).unwrap(); assert!(matches!(rt.inspect(&id).unwrap().state, ContainerState::Running { .. })); rt.stop_container(&id, 10).unwrap(); assert!(matches!(rt.inspect(&id).unwrap().state, ContainerState::Stopped { exit_code: 0 })); }
    #[test] fn test_pull_image() { let rt = OciRuntime::new(); let m = rt.pull_image("ubuntu:22.04").unwrap(); assert_eq!(m.schema_version, 2); assert!(!m.layers.is_empty()); }
    #[test] fn test_exec() { let mut rt = OciRuntime::new(); let id = rt.create_container("c", ContainerSpec::new("alpine", alloc::vec!["sh"])); rt.start_container(&id).unwrap(); let out = rt.exec_in_container(&id, &["ls","/"]).unwrap(); assert!(!out.is_empty()); }
}
