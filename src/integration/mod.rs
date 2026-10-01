//! Integration module for combining OS features
//!
//! This module provides integration points between different SigmaOS subsystems,
//! enabling features like HelenOS async IPC, Kuroko language runtime, and enhanced
//! terminal tabs to work together seamlessly.
use std::format;

use std::boxed::Box;
use std::string::String;
use std::vec::Vec;

use crate::ipc::helenos_async::{HelenAsyncSystem, HelenIpcError, HelenMessage};
use crate::lang::kuroko_lang::{KurokoError, KurokoVM, KurokoValue};
// Terminal module integration temporarily disabled pending terminal tab implementation
// use crate::desktop::terminal::{TabManager, TerminalTab, TerminalError};

/// Integration layer for OS subsystems
pub struct SigmaIntegration {
    pub async_system: HelenAsyncSystem,
    pub kuroko_vm: KurokoVM,
    // Terminal manager temporarily disabled pending terminal tab implementation
    // pub terminal_manager: TabManager,
}

impl SigmaIntegration {
    pub fn new() -> Self {
        SigmaIntegration {
            async_system: HelenAsyncSystem::new(),
            kuroko_vm: KurokoVM::new(),
            // Terminal manager temporarily disabled pending terminal tab implementation
            // terminal_manager: TabManager::new(32),
        }
    }

    /// Initialize integration for a new task/process
    pub fn initialize_task(
        &mut self,
        task_id: usize,
    ) -> Result<IntegrationHandle, IntegrationError> {
        // Initialize async IPC
        let (answerbox_id, phone_id) = self.async_system.initialize_task(task_id);

        // Terminal integration temporarily disabled
        let terminal_id = task_id;
        let tab_id = task_id; // Use task_id as fallback tab_id

        Ok(IntegrationHandle {
            task_id,
            answerbox_id,
            phone_id,
            terminal_id,
            tab_id,
        })
    }

    /// Execute Kuroko code with terminal integration
    pub fn execute_kuroko_with_terminal(
        &mut self,
        code: &str,
        _tab_id: usize,
    ) -> Result<String, IntegrationError> {
    pub fn execute_kuroko_with_terminal(&mut self, code: &str, _tab_id: usize)
        -> Result<String, IntegrationError> {

        // Compile and execute Kuroko code
        let mut compiler = crate::lang::kuroko_lang::KurokoCompiler::new();
        let code_object = compiler
            .compile(code)
            .map_err(|e| IntegrationError::LanguageError(e))?;

        let result = self
            .kuroko_vm
            .interpret(code_object)
        let result = self.kuroko_vm.interpret(code_object)
            .map_err(|e| IntegrationError::LanguageError(e))?;

        // Terminal output temporarily disabled
        let output = self.kuroko_vm.value_to_string(&result);
        Ok(output)
    }

    /// Send async message from terminal to another task
    pub fn send_terminal_message(
        &mut self,
        from_tab_id: usize,
        to_phone_id: usize,
        message: &str,
    ) -> Result<(), IntegrationError> {
        let call_id = self
            .async_system
            .ipc_manager
            .next_call_id
            .fetch_add(1, core::sync::atomic::Ordering::SeqCst);
    pub fn send_terminal_message(&mut self, from_tab_id: usize, to_phone_id: usize,
                                  message: &str) -> Result<(), IntegrationError> {

        let call_id = self.async_system.ipc_manager.next_call_id.fetch_add(
            1, core::sync::atomic::Ordering::SeqCst
        );

        let ipc_message = HelenMessage::new(100, call_id, from_tab_id);

        // Process the message content
        if !message.is_empty() {
            self.async_system
                .ipc_manager
                .send_async(to_phone_id, ipc_message)
                .map_err(|e| IntegrationError::IpcError(e))?;
        }

        Ok(())
    }

    /// Handle interrupt notification and update terminal
    pub fn handle_interrupt_for_terminal(
        &mut self,
        irq: u32,
        _tab_id: usize,
    ) -> Result<(), IntegrationError> {
        self.async_system
            .ipc_manager
            .handle_interrupt(irq)
    pub fn handle_interrupt_for_terminal(&mut self, irq: u32, _tab_id: usize)
        -> Result<(), IntegrationError> {

        self.async_system.ipc_manager.handle_interrupt(irq)
            .map_err(|e| IntegrationError::IpcError(e))?;

        // Terminal update temporarily disabled
        Ok(())
    }

    /// Create split terminal panes with async coordination
    pub fn create_split_terminal(
        &mut self,
        _parent_tab_id: usize,
        _direction: bool,
    ) -> Result<usize, IntegrationError> {
        // Terminal split functionality temporarily disabled
        Err(IntegrationError::TerminalError(
            "Terminal split not implemented".to_string(),
        ))
    }

    /// Run Kuroko script with async IPC capabilities
    pub fn run_async_kuroko_script(
        &mut self,
        script: &str,
        _task_id: usize,
    ) -> Result<KurokoValue, IntegrationError> {
    pub fn run_async_kuroko_script(&mut self, script: &str, _task_id: usize)
        -> Result<KurokoValue, IntegrationError> {

        // This would involve registering async functions in Kuroko
        // For now, just execute normally
        let mut compiler = crate::lang::kuroko_lang::KurokoCompiler::new();
        let code_object = compiler
            .compile(script)
            .map_err(|e| IntegrationError::LanguageError(e))?;

        self.kuroko_vm
            .interpret(code_object)
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

// Terminal error implementation temporarily disabled
// impl From<TerminalError> for IntegrationError {
//     fn from(_error: TerminalError) -> Self {
//         IntegrationError::TerminalError
//     }
// }

pub mod fedora_messaging;
pub use fedora_messaging::{
    AmqpQueueBinding, Bugzilla2FedmsgBridgeEngine, BugzillaEventRecord, BugzillaEventType,
    FedoraAmqpBusAdapter, FedoraMessageCategory, FedoraMessagePayload, FedoraMessageSchemaEngine,
    FedoraMessageSigner, FedoraMessageTopic, FedoraMessagingWebhookEngine, WebhookDeliveryJob,
    WebhookEndpoint,
};

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
            // Send to each integration's async system
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

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_initialization() {
        let mut integration = SigmaIntegration::new();
        let handle = integration.initialize_task(1).unwrap();

        assert_eq!(handle.task_id, 1);
        // Terminal tab test temporarily disabled
        // assert!(handle.tab_id > 0);
    }

    #[test]
    fn test_kuroko_terminal_integration() {
        let mut integration = SigmaIntegration::new();
        let handle = integration.initialize_task(1).unwrap();

        let result = integration.execute_kuroko_with_terminal("1 + 1", handle.tab_id);
        assert!(result.is_ok());
    }

    // Terminal split test temporarily disabled
    // #[test]
    // fn test_split_terminal_creation() {
    //     let mut integration = SigmaIntegration::new();
    //     let handle = integration.initialize_task(1).unwrap();
    //
    //     let new_tab_id = integration.create_split_terminal(handle.tab_id, true);
    //     assert!(new_tab_id.is_ok());
    // }

    #[test]
    fn test_os_integration_manager() {
        let mut manager = OSIntegrationManager::new();
        let id = manager.create_integration();

        assert_eq!(id, 0);
        assert!(manager.get_integration(id).is_some());
    }
}
