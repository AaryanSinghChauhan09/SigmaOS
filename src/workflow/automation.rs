#![no_std]
#![no_main]

/// OOP-based Workflow Automation for SigmaOS
/// Based on Ideas-999-Structured: AI & Automation Item 396
/// Implements workflow engine with triggers and actions

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WorkflowID = usize;
pub type StepID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WorkflowState { Draft = 0, Active = 1, Paused = 2, Completed = 3, Failed = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WorkflowError { Success = 0, NotFound = 1, ExecutionFailed = 2, InvalidState = 3 }

pub trait WorkflowStep {
    fn id(&self) -> StepID;
    fn name(&self) -> &[u8];
    fn execute(&mut self) -> Result<Vec<u8>, WorkflowError>;
    fn is_complete(&self) -> bool;
}

#[repr(C)]
pub struct SimpleWorkflowStep {
    pub id: StepID,
    pub name: [u8; 64],
    pub completed: AtomicUsize,
}

impl SimpleWorkflowStep {
    pub fn new(id: StepID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleWorkflowStep {
            id,
            name: name_array,
            completed: AtomicUsize::new(0),
        }
    }
}

impl WorkflowStep for SimpleWorkflowStep {
    fn id(&self) -> StepID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn execute(&mut self) -> Result<Vec<u8>, WorkflowError> {
        self.completed.store(1, Ordering::SeqCst);
        let mut output = Vec::new();
        let name = self.name();
        for &byte in name { output.push(byte); }
        output.push(b':');
        output.push(b' ');
        output.push(b'd');
        output.push(b'o');
        output.push(b'n');
        output.push(b'e');
        Ok(output)
    }

    fn is_complete(&self) -> bool { self.completed.load(Ordering::SeqCst) == 1 }
}

pub trait Workflow {
    fn id(&self) -> WorkflowID;
    fn name(&self) -> &[u8];
    fn state(&self) -> WorkflowState;
    fn add_step(&mut self, step: Box<dyn WorkflowStep>) -> Result<(), WorkflowError>;
    fn execute(&mut self) -> Result<Vec<u8>, WorkflowError>;
}

#[repr(C)]
pub struct SimpleWorkflow {
    pub id: WorkflowID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub steps: Vec<Option<Box<dyn WorkflowStep>>>,
}

impl SimpleWorkflow {
    pub fn new(id: WorkflowID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleWorkflow {
            id,
            name: name_array,
            state: AtomicUsize::new(WorkflowState::Draft as usize),
            steps: Vec::new(),
        }
    }
}

impl Workflow for SimpleWorkflow {
    fn id(&self) -> WorkflowID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> WorkflowState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn add_step(&mut self, step: Box<dyn WorkflowStep>) -> Result<(), WorkflowError> {
        self.steps.push(Some(step));
        Ok(())
    }

    fn execute(&mut self) -> Result<Vec<u8>, WorkflowError> {
        self.state.store(WorkflowState::Active as usize, Ordering::SeqCst);
        let mut results = Vec::new();

        for step_option in &mut self.steps {
            if let Some(ref mut step) = *step_option {
                match step.execute() {
                    Ok(output) => {
                        for &byte in &output { results.push(byte); }
                        results.push(b'\n');
                    }
                    Err(e) => {
                        self.state.store(WorkflowState::Failed as usize, Ordering::SeqCst);
                        return Err(e);
                    }
                }
            }
        }

        self.state.store(WorkflowState::Completed as usize, Ordering::SeqCst);
        Ok(results)
    }
}

pub trait Trigger {
    fn id(&self) -> usize;
    fn check(&self) -> bool;
    fn fire(&mut self) -> Result<Vec<u8>, WorkflowError>;
}

#[repr(C)]
pub struct SimpleTrigger {
    pub id: usize,
    pub trigger_type: [u8; 32],
    pub condition: AtomicUsize,
}

impl SimpleTrigger {
    pub fn new(id: usize, trigger_type: &[u8]) -> Self {
        let mut type_array = [0u8; 32];
        let type_len = trigger_type.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(trigger_type.as_ptr(), type_array.as_mut_ptr(), type_len);
        }
        SimpleTrigger {
            id,
            trigger_type: type_array,
            condition: AtomicUsize::new(0),
        }
    }
}

impl Trigger for SimpleTrigger {
    fn id(&self) -> usize { self.id }
    fn check(&self) -> bool { self.condition.load(Ordering::SeqCst) == 1 }

    fn fire(&mut self) -> Result<Vec<u8>, WorkflowError> {
        self.condition.store(0, Ordering::SeqCst);
        let mut output = Vec::new();
        let trigger_type = &self.trigger_type;
        let len = trigger_type.iter().position(|&b| b == 0).unwrap_or(32);
        for &byte in &trigger_type[..len] { output.push(byte); }
        output.push(b' ');
        output.push(b'f');
        output.push(b'i');
        output.push(b'r');
        output.push(b'e');
        output.push(b'd');
        Ok(output)
    }
}

pub trait WorkflowEngine {
    fn register_workflow(&mut self, workflow: Box<dyn Workflow>) -> Result<WorkflowID, WorkflowError>;
    fn add_trigger(&mut self, workflow_id: WorkflowID, trigger: Box<dyn Trigger>) -> Result<(), WorkflowError>;
    fn process_triggers(&mut self) -> Vec<WorkflowID>;
    fn execute_workflow(&mut self, workflow_id: WorkflowID) -> Result<Vec<u8>, WorkflowError>;
}

#[repr(C)]
pub struct SimpleWorkflowEngine {
    pub workflows: Vec<Option<Box<dyn Workflow>>>,
    pub triggers: Vec<(WorkflowID, Option<Box<dyn Trigger>>)>,
    pub next_id: AtomicUsize,
}

impl SimpleWorkflowEngine {
    pub fn new() -> Self {
        SimpleWorkflowEngine {
            workflows: Vec::new(),
            triggers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WorkflowEngine for SimpleWorkflowEngine {
    fn register_workflow(&mut self, workflow: Box<dyn Workflow>) -> Result<WorkflowID, WorkflowError> {
        let id = workflow.id();
        self.workflows.push(Some(workflow));
        Ok(id)
    }

    fn add_trigger(&mut self, workflow_id: WorkflowID, trigger: Box<dyn Trigger>) -> Result<(), WorkflowError> {
        self.triggers.push((workflow_id, Some(trigger)));
        Ok(())
    }

    fn process_triggers(&mut self) -> Vec<WorkflowID> {
        let mut triggered_workflows = Vec::new();

        for &(workflow_id, ref trigger_option) in &mut self.triggers {
            if let Some(ref mut trigger) = *trigger_option {
                if trigger.check() {
                    triggered_workflows.push(workflow_id);
                }
            }
        }

        triggered_workflows
    }

    fn execute_workflow(&mut self, workflow_id: WorkflowID) -> Result<Vec<u8>, WorkflowError> {
        for workflow_option in &mut self.workflows {
            if let Some(ref mut workflow) = *workflow_option {
                if workflow.id() == workflow_id {
                    return workflow.execute();
                }
            }
        }
        Err(WorkflowError::NotFound)
    }
}

pub trait Scheduler {
    fn schedule_workflow(&mut self, workflow_id: WorkflowID, delay_ms: u64) -> Result<(), WorkflowError>;
    fn check_scheduled(&mut self) -> Vec<WorkflowID>;
    fn cancel_schedule(&mut self, workflow_id: WorkflowID) -> Result<(), WorkflowError>;
}

#[repr(C)]
pub struct SimpleScheduler {
    pub scheduled: Vec<(WorkflowID, u64, u64)>,
}

impl SimpleScheduler {
    pub fn new() -> Self {
        SimpleScheduler {
            scheduled: Vec::new(),
        }
    }
}

impl Scheduler for SimpleScheduler {
    fn schedule_workflow(&mut self, workflow_id: WorkflowID, delay_ms: u64) -> Result<(), WorkflowError> {
        let current_time = 1000000u64;
        let execute_time = current_time + delay_ms;
        self.scheduled.push((workflow_id, current_time, execute_time));
        Ok(())
    }

    fn check_scheduled(&mut self) -> Vec<WorkflowID> {
        let mut ready = Vec::new();
        let current_time = 1000000u64;

        let mut i = 0;
        while i < self.scheduled.len() {
            if self.scheduled[i].2 <= current_time {
                ready.push(self.scheduled[i].0);
                self.scheduled.remove(i);
            } else {
                i += 1;
            }
        }

        ready
    }

    fn cancel_schedule(&mut self, workflow_id: WorkflowID) -> Result<(), WorkflowError> {
        for i in 0..self.scheduled.len() {
            if self.scheduled[i].0 == workflow_id {
                self.scheduled.remove(i);
                return Ok(());
            }
        }
        Err(WorkflowError::NotFound)
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
