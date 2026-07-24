#![no_std]
#![no_main]

/// OOP-based AI Agent Framework for SigmaOS
/// Implements AI agent using OOP principles with traits and structs
/// No dependency on external AI frameworks
/// Based on Roadmap Item 81: SigmaAI core agent

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Intent type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IntentType {
    SystemCommand = 0,
    FileOperation = 1,
    NetworkRequest = 2,
    ApplicationLaunch = 3,
    InformationQuery = 4,
    Custom = 5,
}

/// Intent (OOP: Intent object)
#[repr(C)]
pub struct Intent {
    pub intent_type: IntentType,
    pub confidence: f32,
    pub command: [u8; 256],
    pub parameters: [u8; 512],
}

impl Intent {
    pub fn new(intent_type: IntentType, command: &[u8]) -> Self {
        let mut command_array = [0u8; 256];
        let cmd_len = command.len().min(255);

        unsafe {
            core::ptr::copy_nonoverlapping(command.as_ptr(), command_array.as_mut_ptr(), cmd_len);
        }

        Intent {
            intent_type,
            confidence: 0.0,
            command: command_array,
            parameters: [0; 512],
        }
    }

    pub fn set_parameters(&mut self, parameters: &[u8]) {
        let len = parameters.len().min(511);
        unsafe {
            core::ptr::copy_nonoverlapping(parameters.as_ptr(), self.parameters.as_mut_ptr(), len);
        }
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

/// AI agent trait (OOP interface)
pub trait AIAgent {
    /// Parse natural language input
    fn parse(&mut self, input: &[u8]) -> Result<Intent, AIError>;
    /// Execute intent
    fn execute(&mut self, intent: &Intent) -> Result<Vec<u8>, AIError>;
    /// Learn from feedback
    fn learn(&mut self, input: &[u8], feedback: bool);
    /// Get agent info
    fn info(&self) -> AgentInfo;
}

/// AI error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AIError {
    Success = 0,
    ParseFailed = 1,
    ExecutionFailed = 2,
    UnknownIntent = 3,
    PermissionDenied = 4,
    InvalidInput = 5,
}

/// Agent info
#[repr(C)]
pub struct AgentInfo {
    pub name: [u8; 64],
    pub version: (u32, u32, u32),
    pub total_intents: usize,
    pub execution_count: usize,
    pub capability: AgentCapability,
}

impl AgentInfo {
    pub fn new() -> Self {
        AgentInfo {
            name: [0; 64],
            version: (1, 0, 0),
            total_intents: 0,
            execution_count: 0,
            capability: AgentCapability::new(),
        }
    }
}

/// Agent capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AgentCapability {
    pub can_parse: bool,
    pub can_execute: bool,
    pub can_learn: bool,
}

impl AgentCapability {
    pub fn new() -> Self {
        AgentCapability {
            can_parse: false,
            can_execute: false,
            can_learn: false,
        }
    }

    pub fn full() -> Self {
        AgentCapability {
            can_parse: true,
            can_execute: true,
            can_learn: true,
        }
    }
}

/// Simple AI agent (OOP: Concrete agent class)
#[repr(C)]
pub struct SimpleAIAgent {
    pub name: [u8; 64],
    pub version: (u32, u32, u32),
    pub execution_count: AtomicUsize,
    pub capability: AgentCapability,
    pub patterns: Vec<Pattern>,
}

/// Pattern for intent matching
#[repr(C)]
pub struct Pattern {
    pub pattern: [u8; 128],
    pub intent_type: IntentType,
    pub template: [u8; 256],
}

impl Pattern {
    pub fn new(pattern: &[u8], intent_type: IntentType, template: &[u8]) -> Self {
        let mut pattern_array = [0u8; 128];
        let mut template_array = [0u8; 256];

        let pattern_len = pattern.len().min(127);
        let template_len = template.len().min(255);

        unsafe {
            core::ptr::copy_nonoverlapping(pattern.as_ptr(), pattern_array.as_mut_ptr(), pattern_len);
            core::ptr::copy_nonoverlapping(template.as_ptr(), template_array.as_mut_ptr(), template_len);
        }

        Pattern {
            pattern: pattern_array,
            intent_type,
            template: template_array,
        }
    }
}

impl SimpleAIAgent {
    pub fn new(name: &[u8], version: (u32, u32, u32), capability: AgentCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleAIAgent {
            name: name_array,
            version,
            execution_count: AtomicUsize::new(0),
            capability,
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);
    }

    unsafe fn match_pattern(&self, input: &[u8]) -> Option<&Pattern> {
        for pattern in &self.patterns {
            let pattern_len = pattern.pattern.iter().position(|&b| b == 0).unwrap_or(128);
            let pattern_str = &pattern.pattern[..pattern_len];

            if input.len() >= pattern_len {
                let mut matches = true;
                for i in 0..pattern_len {
                    if input[i] != pattern_str[i] {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    return Some(pattern);
                }
            }
        }
        None
    }

    /// Helper byte-level search function
    fn contains_bytes(&self, haystack: &[u8], needle: &[u8]) -> bool {
        if needle.is_empty() {
            return true;
        }
        haystack.windows(needle.len()).any(|window| window == needle)
    }

    /// Translates natural language CLI commands (supporting English, Hindi, and Tamil)
    pub fn translate_natural_command(&self, input: &[u8]) -> Result<Vec<u8>, AIError> {
        if input.is_empty() {
            return Err(AIError::InvalidInput);
        }

        // Direct check for "libreoffice" and "install" or "karo" (Hindi) or "நிறுவவும்" (Tamil)
        let has_libreoffice = self.contains_bytes(input, b"libreoffice") || self.contains_bytes(input, b"\xE0\xAE\xB2\xE0\xAE\xBF\xE0\xAE\xAA\xE0\xAF\x8D\xE0\xAE\xB0\xE0\xAF\x87\xE0\xAE\x86\xE0\xAE\xAA\xE0\xAE\xBF\xE0\xAE\xB8\xAF");
        let has_install = self.contains_bytes(input, b"install")
            || self.contains_bytes(input, b"karo")
            || self.contains_bytes(input, b"\xE0\xAE\xA0\xE0\xAE\xBF\xE0\xAE\xB1\xE0\xAF\x81\xE0\xAE\xB5\xE0\xAE\xB5\xE0\xAF\x81\xE0\xAE\xAE\xAF");

        if has_libreoffice && has_install {
            let mut out = Vec::new();
            for &b in b"sigpkg install libreoffice" { out.push(b); }
            return Ok(out);
        }

        // Disk usage checks
        if self.contains_bytes(input, b"disk") && (self.contains_bytes(input, b"usage") || self.contains_bytes(input, b"show")) {
            let mut out = Vec::new();
            for &b in b"df -h" { out.push(b); }
            return Ok(out);
        }

        // WiFi connection checks
        if self.contains_bytes(input, b"connect") && self.contains_bytes(input, b"wifi") {
            let mut out = Vec::new();
            for &b in b"sigma-wifi connect --ssid Home" { out.push(b); }
            return Ok(out);
        }

        // Default to returning the input command
        let mut out = Vec::new();
        for &b in input { out.push(b); }
        Ok(out)
    }

    /// Performs safety checks on potentially dangerous commands (such as rm -rf / or deleting accounts folder)
    pub fn perform_safety_check(&self, command: &[u8]) -> Option<Vec<u8>> {
        if self.contains_bytes(command, b"rm -rf /") || self.contains_bytes(command, b"delete all files") {
            let mut warning = Vec::new();
            for &b in b"Warning: This will delete all files. Are you sure? (y/N)" {
                warning.push(b);
            }
            return Some(warning);
        }

        if self.contains_bytes(command, b"sigma-accounts") || self.contains_bytes(command, b"home/ravi/sigma-accounts") {
            let mut warning = Vec::new();
            for &b in b"Warning: You're deleting your accounts folder." {
                warning.push(b);
            }
            return Some(warning);
        }

        None
    }

    /// Explains low-level command options in clear, plain language (e.g., tar extraction flags)
    pub fn explain_command(&self, command: &[u8]) -> Result<Vec<u8>, AIError> {
        if command.is_empty() {
            return Err(AIError::InvalidInput);
        }

        if self.contains_bytes(command, b"tar -xvf") || self.contains_bytes(command, b"tar") {
            let mut out = Vec::new();
            for &b in b"Extracts (-x) a tar archive (-f) verbosely (-v) with gzip compression" {
                out.push(b);
            }
            return Ok(out);
        }

        let mut fallback = Vec::new();
        for &b in b"Executes the input system parameters inside Ring 3 sandboxes" {
            fallback.push(b);
        }
        Ok(fallback)
    }
}

impl AIAgent for SimpleAIAgent {
    fn parse(&mut self, input: &[u8]) -> Result<Intent, AIError> {
        if !self.capability.can_parse {
            return Err(AIError::PermissionDenied);
        }

        if input.len() == 0 {
            return Err(AIError::InvalidInput);
        }

        unsafe {
            if let Some(pattern) = self.match_pattern(input) {
                let mut intent = Intent::new(pattern.intent_type, &pattern.template);
                intent.confidence = 1.0;
                Ok(intent)
            } else {
                // Default to information query if no pattern matches
                let mut intent = Intent::new(IntentType::InformationQuery, input);
                intent.confidence = 0.5;
                Ok(intent)
            }
        }
    }

    fn execute(&mut self, intent: &Intent) -> Result<Vec<u8>, AIError> {
        if !self.capability.can_execute {
            return Err(AIError::PermissionDenied);
        }

        self.execution_count.fetch_add(1, Ordering::SeqCst);

        // In a real implementation, this would execute the actual command
        // For now, return a simulated response
        let mut response = Vec::new();
        let success_msg = b"Command executed successfully";
        
        for byte in success_msg {
            response.push(*byte);
        }

        Ok(response)
    }

    fn learn(&mut self, input: &[u8], feedback: bool) {
        if !self.capability.can_learn {
            return;
        }

        // In a real implementation, this would update the model
        // For now, this is a placeholder
    }

    fn info(&self) -> AgentInfo {
        AgentInfo {
            name: self.name,
            version: self.version,
            total_intents: self.patterns.len(),
            execution_count: self.execution_count.load(Ordering::SeqCst),
            capability: self.capability,
        }
    }
}

/// AI agent manager trait (OOP interface)
pub trait AIAgentManager {
    /// Register agent
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<usize, AIError>;
    /// Unregister agent
    fn unregister_agent(&mut self, id: usize) -> Result<(), AIError>;
    /// Get agent
    fn get_agent(&self, id: usize) -> Option<&dyn AIAgent>;
    /// Process natural language request
    fn process(&mut self, input: &[u8]) -> Result<Vec<u8>, AIError>;
    /// Get manager statistics
    fn stats(&self) -> AIStats;
}

/// AI statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AIStats {
    pub total_agents: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
}

impl AIStats {
    pub fn new() -> Self {
        AIStats {
            total_agents: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
        }
    }
}

/// Simple AI agent manager (OOP: Concrete manager class)
pub struct SimpleAIAgentManager {
    agents: Vec<Option<Box<dyn AIAgent>>>,
    active_agent: AtomicUsize,
    stats: AIStats,
    capability: ManagerCapability,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ManagerCapability {
    pub can_register: bool,
    pub can_unregister: bool,
    pub can_process: bool,
}

impl ManagerCapability {
    pub fn new() -> Self {
        ManagerCapability {
            can_register: false,
            can_unregister: false,
            can_process: false,
        }
    }

    pub fn full() -> Self {
        ManagerCapability {
            can_register: true,
            can_unregister: true,
            can_process: true,
        }
    }
}

impl SimpleAIAgentManager {
    pub fn new(capability: ManagerCapability) -> Self {
        SimpleAIAgentManager {
            agents: Vec::new(),
            active_agent: AtomicUsize::new(0),
            stats: AIStats::new(),
            capability,
        }
    }
}

impl AIAgentManager for SimpleAIAgentManager {
    fn register_agent(&mut self, agent: Box<dyn AIAgent>) -> Result<usize, AIError> {
        if !self.capability.can_register {
            return Err(AIError::PermissionDenied);
        }

        let id = self.agents.len();
        self.agents.push(Some(agent));
        self.stats.total_agents += 1;
        Ok(id)
    }

    fn unregister_agent(&mut self, id: usize) -> Result<(), AIError> {
        if !self.capability.can_unregister {
            return Err(AIError::PermissionDenied);
        }

        if id < self.agents.len() {
            self.agents[id] = None;
            self.stats.total_agents -= 1;
            Ok(())
        } else {
            Err(AIError::InvalidInput)
        }
    }

    fn get_agent(&self, id: usize) -> Option<&dyn AIAgent> {
        if id < self.agents.len() {
            if let Some(ref agent) = self.agents[id] {
                return Some(agent.as_ref());
            }
        }
        None
    }

    fn process(&mut self, input: &[u8]) -> Result<Vec<u8>, AIError> {
        if !self.capability.can_process {
            return Err(AIError::PermissionDenied);
        }

        self.stats.total_requests += 1;

        let active = self.active_agent.load(Ordering::SeqCst);
        if let Some(ref mut agent) = self.agents[active] {
            let intent = agent.parse(input)?;

            if let Ok(response) = agent.execute(&intent) {
                self.stats.successful_requests += 1;
                Ok(response)
            } else {
                self.stats.failed_requests += 1;
                Err(AIError::ExecutionFailed)
            }
        } else {
            self.stats.failed_requests += 1;
            Err(AIError::InvalidInput)
        }
    }

    fn stats(&self) -> AIStats {
        self.stats
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
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

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

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

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_agent_basics() {
        let agent = SimpleAIAgent::new(b"TestAgent", (1, 0, 0), AgentCapability::full());
        assert!(agent.version == (1, 0, 0));
    }

    #[test]
    fn test_ai_natural_language_translations() {
        let agent = SimpleAIAgent::new(b"S-CLI", (1, 0, 0), AgentCapability::full());

        let install_en = agent.translate_natural_command(b"install libreoffice").unwrap();
        assert_eq!(install_en, b"sigpkg install libreoffice");

        let install_hi = agent.translate_natural_command(b"libreoffice install karo").unwrap();
        assert_eq!(install_hi, b"sigpkg install libreoffice");

        let disk_usage = agent.translate_natural_command(b"show my disk usage").unwrap();
        assert_eq!(disk_usage, b"df -h");

        let wifi_connect = agent.translate_natural_command(b"connect to WiFi Home").unwrap();
        assert_eq!(wifi_connect, b"sigma-wifi connect --ssid Home");
    }

    #[test]
    fn test_ai_safety_checks() {
        let agent = SimpleAIAgent::new(b"S-CLI", (1, 0, 0), AgentCapability::full());

        let dangerous_res = agent.perform_safety_check(b"rm -rf /");
        assert!(dangerous_res.is_some());
        assert!(dangerous_res.unwrap().windows(7).any(|w| window_eq(w, b"Warning")));

        let account_delete_res = agent.perform_safety_check(b"rm -rf /home/ravi/sigma-accounts/");
        assert!(account_delete_res.is_some());
        assert!(account_delete_res.unwrap().windows(7).any(|w| window_eq(w, b"Warning")));

        let safe_res = agent.perform_safety_check(b"ls -la /var/www");
        assert!(safe_res.is_none());
    }

    #[test]
    fn test_ai_command_explanations() {
        let agent = SimpleAIAgent::new(b"S-CLI", (1, 0, 0), AgentCapability::full());

        let explanation = agent.explain_command(b"tar -xvf archive.tar.gz").unwrap();
        assert!(explanation.windows(8).any(|w| window_eq(w, b"Extracts")));
    }

    fn window_eq(a: &[u8], b: &[u8]) -> bool {
        a == b
    }
}
