#![no_std]
#![no_main]

/// OOP-based Desktop Workspace for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 716
/// Implements virtual desktops and workspace management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WorkspaceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceLayout { Tiling = 0, Stacking = 1, Tabbed = 2, Floating = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WorkspaceError { Success = 0, NotFound = 1 }

pub trait Workspace {
    fn id(&self) -> WorkspaceID;
    fn name(&self) -> &[u8];
    fn layout(&self) -> WorkspaceLayout;
    fn set_layout(&mut self, layout: WorkspaceLayout);
}

#[repr(C)]
pub struct SimpleWorkspace {
    pub id: WorkspaceID,
    pub name: [u8; 64],
    pub layout: AtomicUsize,
}

impl SimpleWorkspace {
    pub fn new(id: WorkspaceID, name: &[u8], layout: WorkspaceLayout) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleWorkspace {
            id,
            name: name_array,
            layout: AtomicUsize::new(layout as usize),
        }
    }
}

impl Workspace for SimpleWorkspace {
    fn id(&self) -> WorkspaceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn layout(&self) -> WorkspaceLayout { unsafe { core::mem::transmute(self.layout.load(Ordering::SeqCst)) } }
    
    fn set_layout(&mut self, layout: WorkspaceLayout) {
        self.layout.store(layout as usize, Ordering::SeqCst);
    }
}

pub trait WorkspaceManager {
    fn create_workspace(&mut self, name: &[u8], layout: WorkspaceLayout) -> Result<WorkspaceID, WorkspaceError>;
    fn destroy_workspace(&mut self, id: WorkspaceID) -> Result<(), WorkspaceError>;
    fn get_workspace(&self, id: WorkspaceID) -> Option<&dyn Workspace>;
    fn switch_to(&mut self, id: WorkspaceID) -> Result<(), WorkspaceError>;
}

#[repr(C)]
pub struct SimpleWorkspaceManager {
    pub workspaces: Vec<Option<Box<dyn Workspace>>>,
    pub active: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleWorkspaceManager {
    pub fn new() -> Self {
        SimpleWorkspaceManager {
            workspaces: Vec::new(),
            active: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WorkspaceManager for SimpleWorkspaceManager {
    fn create_workspace(&mut self, name: &[u8], layout: WorkspaceLayout) -> Result<WorkspaceID, WorkspaceError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let workspace = SimpleWorkspace::new(id, name, layout);
        self.workspaces.push(Some(Box::new(workspace)));
        Ok(id)
    }
    
    fn destroy_workspace(&mut self, id: WorkspaceID) -> Result<(), WorkspaceError> {
        for workspace_option in &mut self.workspaces {
            if let Some(ref workspace) = *workspace_option {
                if workspace.id() == id {
                    return Ok(());
                }
            }
        }
        Err(WorkspaceError::NotFound)
    }
    
    fn get_workspace(&self, id: WorkspaceID) -> Option<&dyn Workspace> {
        for workspace_option in &self.workspaces {
            if let Some(ref workspace) = *workspace_option {
                if workspace.id() == id { return Some(workspace.as_ref()); }
            }
        }
        None
    }
    
    fn switch_to(&mut self, id: WorkspaceID) -> Result<(), WorkspaceError> {
        if self.get_workspace(id).is_some() {
            self.active.store(id, Ordering::SeqCst);
            Ok(())
        } else {
            Err(WorkspaceError::NotFound)
        }
    }
}

pub trait VirtualDesktop {
    fn add_desktop(&mut self) -> Result<WorkspaceID, WorkspaceError>;
    fn remove_desktop(&mut self, id: WorkspaceID) -> Result<(), WorkspaceError>;
    fn get_active_desktop(&self) -> WorkspaceID;
}

#[repr(C)]
pub struct SimpleVirtualDesktop {
    pub manager: SimpleWorkspaceManager,
}

impl SimpleVirtualDesktop {
    pub fn new(manager: SimpleWorkspaceManager) -> Self {
        SimpleVirtualDesktop { manager }
    }
}

impl VirtualDesktop for SimpleVirtualDesktop {
    fn add_desktop(&mut self) -> Result<WorkspaceID, WorkspaceError> {
        self.manager.create_workspace(b"Desktop", WorkspaceLayout::Tiling)
    }
    
    fn remove_desktop(&mut self, id: WorkspaceID) -> Result<(), WorkspaceError> {
        self.manager.destroy_workspace(id)
    }
    
    fn get_active_desktop(&self) -> WorkspaceID {
        self.manager.active.load(Ordering::SeqCst)
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
