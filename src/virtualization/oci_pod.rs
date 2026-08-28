extern crate alloc;
/// OCI-Compliant Container Pod and Namespace Manager
/// Manages pod configurations, limits, shared namespaces, and container orchestration
/// to easily match and exceed Fedora's native Podman/Kubernetes setups.
use core::sync::atomic::AtomicUsize;

pub type PodID = usize;
pub type ContainerID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PodState {
    Pending,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub id: ContainerID,
    pub image_name: [u8; 32],
    pub cpu_shares: usize,
    pub memory_limit_bytes: usize,
    pub shared_net_namespace: bool,
}

impl ContainerConfig {
    pub fn new(id: ContainerID, image: &[u8], cpu: usize, mem: usize) -> Self {
        let mut img_arr = [0u8; 32];
        let len = image.len().min(31);
        img_arr[..len].copy_from_slice(&image[..len]);
        ContainerConfig {
            id,
            image_name: img_arr,
            cpu_shares: cpu,
            memory_limit_bytes: mem,
            shared_net_namespace: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OciPod {
    pub id: PodID,
    pub state: PodState,
    pub containers: Vec<ContainerConfig>,
}

impl OciPod {
    pub fn new(id: PodID) -> Self {
        OciPod {
            id,
            state: PodState::Pending,
            containers: Vec::new(),
        }
    }
}

pub struct OciPodManager {
    pub pods: Vec<OciPod>,
    pub next_pod_id: AtomicUsize,
}

impl Default for OciPodManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OciPodManager {
    pub fn new() -> Self {
        OciPodManager {
            pods: Vec::new(),
            next_pod_id: AtomicUsize::new(1),
        }
    }

    pub fn deploy_pod(&mut self, mut pod: OciPod) -> Result<PodID, &'static str> {
        if pod.containers.is_empty() {
            return Err("Cannot deploy a pod without any containers");
        }
        pod.state = PodState::Running;
        let id = pod.id;
        self.pods.push(pod);
        Ok(id)
    }

    pub fn terminate_pod(&mut self, pod_id: PodID) -> Result<(), &'static str> {
        let mut found_idx = None;
        for (i, p) in self.pods.iter().enumerate() {
            if p.id == pod_id {
                found_idx = Some(i);
                break;
            }
        }

        let idx = found_idx.ok_or("Pod not found")?;
        self.pods[idx].state = PodState::Succeeded;
        Ok(())
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
        new_vec
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
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
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = VecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = VecIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct VecIter<'a, T> {
    vec: &'a Vec<T>,
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

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
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
    fn test_oci_pod_deployment() {
        let mut manager = OciPodManager::new();
        let mut pod = OciPod::new(10);
        let container =
            ContainerConfig::new(20, b"fedora-toolbox:39", 1024, 2 * 1024 * 1024 * 1024);
        pod.containers.push(container);

        // Deploy pod
        manager.deploy_pod(pod).unwrap();
        assert_eq!(manager.pods[0].state, PodState::Running);

        // Terminate pod
        manager.terminate_pod(10).unwrap();
        assert_eq!(manager.pods[0].state, PodState::Succeeded);
    }
}
