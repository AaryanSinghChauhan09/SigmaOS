//! Integration module for combining OS features
//!
//! This module provides integration points between different SigmaOS subsystems,
//! enabling features like HelenOS async IPC, Kuroko language runtime, and enhanced
//! terminal tabs to work together seamlessly.
use std::format;

use std::vec::Vec;
use std::string::String;

use crate::ipc::helenos_async::{HelenAsyncSystem, HelenMessage, HelenIpcError};
use crate::lang::kuroko_lang::{KurokoVM, KurokoValue, KurokoError};

/// Integration layer for OS subsystems
pub struct SigmaIntegration {
    pub async_system: HelenAsyncSystem,
    pub kuroko_vm: KurokoVM,
}

impl SigmaIntegration {
    pub fn new() -> Self {
        SigmaIntegration {
            async_system: HelenAsyncSystem::new(),
            kuroko_vm: KurokoVM::new(),
        }
    }

    /// Initialize integration for a new task/process
    pub fn initialize_task(&mut self, task_id: usize) -> Result<IntegrationHandle, IntegrationError> {
        // Initialize async IPC
        let (answerbox_id, phone_id) = self.async_system.initialize_task(task_id);

        let terminal_id = task_id;
        let tab_id = task_id;

        Ok(IntegrationHandle {
            task_id,
            answerbox_id,
            phone_id,
            terminal_id,
            tab_id,
        })
    }

    /// Execute Kuroko code with terminal integration
    pub fn execute_kuroko_with_terminal(&mut self, code: &str, _tab_id: usize)
        -> Result<String, IntegrationError> {

        let mut compiler = crate::lang::kuroko_lang::KurokoCompiler::new();
        let code_object = compiler.compile(code)
            .map_err(|e| IntegrationError::LanguageError(e))?;

        let result = self.kuroko_vm.interpret(code_object)
            .map_err(|e| IntegrationError::LanguageError(e))?;

        let output = self.kuroko_vm.value_to_string(&result);
        Ok(output)
    }

    /// Send async message from terminal to another task
    pub fn send_terminal_message(&mut self, from_tab_id: usize, to_phone_id: usize,
                                  message: &str) -> Result<(), IntegrationError> {

        let call_id = self.async_system.ipc_manager.next_call_id.fetch_add(
            1, core::sync::atomic::Ordering::SeqCst
        );

        let ipc_message = HelenMessage::new(100, call_id, from_tab_id);

        if !message.is_empty() {
            self.async_system.ipc_manager.send_async(to_phone_id, ipc_message)
                .map_err(|e| IntegrationError::IpcError(e))?;
        }

        Ok(())
    }

    /// Handle interrupt notification and update terminal
    pub fn handle_interrupt_for_terminal(&mut self, irq: u32, _tab_id: usize)
        -> Result<(), IntegrationError> {

        self.async_system.ipc_manager.handle_interrupt(irq)
            .map_err(|e| IntegrationError::IpcError(e))?;

        Ok(())
    }

    /// Create split terminal panes with async coordination
    pub fn create_split_terminal(&mut self, _parent_tab_id: usize, _direction: bool)
        -> Result<usize, IntegrationError> {

        Err(IntegrationError::TerminalError("Terminal split not implemented".to_string()))
    }

    /// Run Kuroko script with async IPC capabilities
    pub fn run_async_kuroko_script(&mut self, script: &str, _task_id: usize)
        -> Result<KurokoValue, IntegrationError> {

        let mut compiler = crate::lang::kuroko_lang::KurokoCompiler::new();
        let code_object = compiler.compile(script)
            .map_err(|e| IntegrationError::LanguageError(e))?;

        self.kuroko_vm.interpret(code_object)
            .map_err(|e| IntegrationError::LanguageError(e))
    }
}

impl Default for SigmaIntegration {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle for integrated task resources
#[repr(C)]
pub struct IntegrationHandle {
    pub task_id: usize,
    pub answerbox_id: usize,
    pub phone_id: usize,
    pub terminal_id: usize,
    pub tab_id: usize,
}

/// Integration error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrationError {
    Success,
    IpcError(HelenIpcError),
    LanguageError(KurokoError),
    TerminalError(String),
    NotFound,
    PermissionDenied,
}

impl From<HelenIpcError> for IntegrationError {
    fn from(error: HelenIpcError) -> Self {
        IntegrationError::IpcError(error)
    }
}

impl From<KurokoError> for IntegrationError {
    fn from(error: KurokoError) -> Self {
        IntegrationError::LanguageError(error)
    }
}

pub mod fedora_messaging;
pub use fedora_messaging::{
    AmqpQueueBinding, Bugzilla2FedmsgBridgeEngine, BugzillaEventRecord, BugzillaEventType,
    FedoraAmqpBusAdapter, FedoraMessageCategory, FedoraMessagePayload, FedoraMessageSchemaEngine,
    FedoraMessageSigner, FedoraMessageTopic, FedoraMessagingWebhookEngine, WebhookDeliveryJob,
    WebhookEndpoint,
};

pub mod sigma_native;
pub use sigma_native::*;

/// OS-wide integration manager
pub struct OSIntegrationManager {
    pub integrations: Vec<SigmaIntegration>,
    pub global_async_system: HelenAsyncSystem,
}

impl OSIntegrationManager {
    pub fn new() -> Self {
        OSIntegrationManager {
            integrations: Vec::new(),
            global_async_system: HelenAsyncSystem::new(),
        }
    }

    /// Create new integration context
    pub fn create_integration(&mut self) -> usize {
        let id = self.integrations.len();
        self.integrations.push(SigmaIntegration::new());
        id
    }

    /// Get integration by ID
    pub fn get_integration(&mut self, id: usize) -> Option<&mut SigmaIntegration> {
        self.integrations.get_mut(id)
    }

    /// Broadcast message to all integrations
    pub fn broadcast_message(&mut self, message: HelenMessage) -> Result<(), IntegrationError> {
        for integration in &mut self.integrations {
            let _ = integration.async_system.ipc_manager.send_async(0, message);
        }
        Ok(())
    }
}

impl Default for OSIntegrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_initialization() {
        let mut integration = SigmaIntegration::new();
        let handle = integration.initialize_task(1).unwrap();

        assert_eq!(handle.task_id, 1);
    }

    #[test]
    fn test_kuroko_terminal_integration() {
        let mut integration = SigmaIntegration::new();
        let handle = integration.initialize_task(1).unwrap();

        let result = integration.execute_kuroko_with_terminal("1 + 1", handle.tab_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_os_integration_manager() {
        let mut manager = OSIntegrationManager::new();
        let id = manager.create_integration();

        assert_eq!(id, 0);
        assert!(manager.get_integration(id).is_some());
    }
}
