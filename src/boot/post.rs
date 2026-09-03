#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::string::{String, ToString};

// POST (Power-On Self-Test) Diagnostics
// BIOS/UEFI firmware POST implementation

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostStatus {
    Passed,
    Failed,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestType {
    Cpu,
    Memory,
    Storage,
    Network,
    Display,
}

#[derive(Debug, Clone)]
pub struct PostTest {
    pub test_type: TestType,
    pub status: PostStatus,
    pub description: String,
    pub error_code: Option<u32>,
}

impl PostTest {
    pub fn new(test_type: TestType, description: String) -> Self {
        Self {
            test_type,
            status: PostStatus::Passed,
            description,
            error_code: None,
        }
    }

    pub fn fail(&mut self, error_code: u32) {
        self.status = PostStatus::Failed;
        self.error_code = Some(error_code);
    }

    pub fn warn(&mut self) {
        self.status = PostStatus::Warning;
    }
}

pub struct PostDiagnostics {
    pub tests: Vec<PostTest>,
    pub overall_status: PostStatus,
}

impl PostDiagnostics {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
            overall_status: PostStatus::Passed,
        }
    }

    pub fn add_test(&mut self, test: PostTest) {
        self.tests.push(test);
        self.update_overall_status();
    }

    pub fn run_cpu_test(&mut self) -> Result<(), &'static str> {
        let mut test = PostTest::new(TestType::Cpu, "CPU Registers Test".to_string());

        // Simulated CPU test
        let cpu_ok = true; // In real implementation, would test CPU

        if cpu_ok {
            self.add_test(test);
            Ok(())
        } else {
            test.fail(0xC001);
            self.add_test(test);
            Err("CPU test failed")
        }
    }

    pub fn run_memory_test(&mut self) -> Result<(), &'static str> {
        let mut test = PostTest::new(TestType::Memory, "Memory Integrity Test".to_string());

        // Simulated memory test
        let memory_ok = true; // In real implementation, would test memory

        if memory_ok {
            self.add_test(test);
            Ok(())
        } else {
            test.fail(0xC002);
            self.add_test(test);
            Err("Memory test failed")
        }
    }

    pub fn run_storage_test(&mut self) -> Result<(), &'static str> {
        let mut test = PostTest::new(TestType::Storage, "Storage Controller Test".to_string());

        // Simulated storage test
        let storage_ok = true; // In real implementation, would test storage

        if storage_ok {
            self.add_test(test);
            Ok(())
        } else {
            test.fail(0xC003);
            self.add_test(test);
            Err("Storage test failed")
        }
    }

    pub fn run_all_tests(&mut self) -> PostStatus {
        self.run_cpu_test().ok();
        self.run_memory_test().ok();
        self.run_storage_test().ok();

        self.overall_status
    }

    pub fn get_failed_tests(&self) -> Vec<&PostTest> {
        self.tests
            .iter()
            .filter(|t| t.status == PostStatus::Failed)
            .collect()
    }

    pub fn get_warning_tests(&self) -> Vec<&PostTest> {
        self.tests
            .iter()
            .filter(|t| t.status == PostStatus::Warning)
            .collect()
    }

    fn update_overall_status(&mut self) {
        if self.tests.iter().any(|t| t.status == PostStatus::Failed) {
            self.overall_status = PostStatus::Failed;
        } else if self.tests.iter().any(|t| t.status == PostStatus::Warning) {
            self.overall_status = PostStatus::Warning;
        } else {
            self.overall_status = PostStatus::Passed;
        }
    }

    pub fn test_count(&self) -> usize {
        self.tests.len()
    }
}

impl Default for PostDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_test_creation() {
        let test = PostTest::new(TestType::Cpu, "Test".to_string());
        assert_eq!(test.status, PostStatus::Passed);
        assert_eq!(test.test_type, TestType::Cpu);
    }

    #[test]
    fn test_post_test_fail() {
        let mut test = PostTest::new(TestType::Memory, "Test".to_string());
        test.fail(0x1234);

        assert_eq!(test.status, PostStatus::Failed);
        assert_eq!(test.error_code, Some(0x1234));
    }

    #[test]
    fn test_post_test_warn() {
        let mut test = PostTest::new(TestType::Storage, "Test".to_string());
        test.warn();

        assert_eq!(test.status, PostStatus::Warning);
    }

    #[test]
    fn test_post_diagnostics() {
        let mut post = PostDiagnostics::new();
        let test = PostTest::new(TestType::Cpu, "Test".to_string());

        post.add_test(test);
        assert_eq!(post.test_count(), 1);
        assert_eq!(post.overall_status, PostStatus::Passed);
    }

    #[test]
    fn test_post_diagnostics_fail() {
        let mut post = PostDiagnostics::new();
        let mut test = PostTest::new(TestType::Cpu, "Test".to_string());
        test.fail(0xC001);

        post.add_test(test);
        assert_eq!(post.overall_status, PostStatus::Failed);
    }

    #[test]
    fn test_post_diagnostics_warning() {
        let mut post = PostDiagnostics::new();
        let mut test = PostTest::new(TestType::Cpu, "Test".to_string());
        test.warn();

        post.add_test(test);
        assert_eq!(post.overall_status, PostStatus::Warning);
    }

    #[test]
    fn test_get_failed_tests() {
        let mut post = PostDiagnostics::new();

        let mut test1 = PostTest::new(TestType::Cpu, "Test1".to_string());
        test1.fail(0xC001);
        post.add_test(test1);

        let test2 = PostTest::new(TestType::Memory, "Test2".to_string());
        post.add_test(test2);

        let failed = post.get_failed_tests();
        assert_eq!(failed.len(), 1);
    }

    #[test]
    fn test_run_all_tests() {
        let mut post = PostDiagnostics::new();
        let status = post.run_all_tests();

        assert_eq!(post.test_count(), 3);
        assert_eq!(status, PostStatus::Passed);
    }
}
