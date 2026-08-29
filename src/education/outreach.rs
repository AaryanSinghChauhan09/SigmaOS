extern crate alloc;
use alloc::vec;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS Education & Outreach Framework
// Training ecosystem, university partnerships, and standardized documentation validation

use crate::klib::HashMap;

/// Training path / certification details
#[derive(Debug, Clone)]
pub struct LearningPath {
    pub id: String,
    pub title: String,
    pub syllabus: Vec<String>,
    pub cert_name: String,
    pub active_students: u32,
}

/// Academic partnerships details
#[derive(Debug, Clone)]
pub struct UniversityPartnership {
    pub institution: String,
    pub department: String,
    pub course_name: String,
    pub research_focus: Option<String>,
    pub dynamic_collaboration_active: bool,
}

/// Documentation standards categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocFormat {
    ManPage,
    MarkdownGuide,
    WikiArticle,
}

/// A documentation asset to audit for compliance with Linux-equivalent HOWTO/wiki standards
#[derive(Debug, Clone)]
pub struct DocAsset {
    pub filename: String,
    pub format: DocFormat,
    pub has_standard_frontmatter: bool,
    pub code_examples_count: usize,
    pub is_fully_localized: bool,
}

impl DocAsset {
    pub fn passes_lint(&self) -> bool {
        self.has_standard_frontmatter
            && (self.code_examples_count > 0 || self.format == DocFormat::ManPage)
    }
}

/// Manager of educational outreach and documentation quality assurance
pub struct EducationOutreachManager {
    pub learning_paths: HashMap<String, LearningPath>,
    pub university_partners: Vec<UniversityPartnership>,
    pub documentation_pool: Vec<DocAsset>,
}

impl EducationOutreachManager {
    pub fn new() -> Self {
        Self {
            learning_paths: HashMap::new(),
            university_partners: Vec::new(),
            documentation_pool: Vec::new(),
        }
    }

    pub fn register_path(
        &mut self,
        id: String,
        title: String,
        syllabus: Vec<String>,
        cert: String,
    ) {
        let path = LearningPath {
            id: id.clone(),
            title,
            syllabus,
            cert_name: cert,
            active_students: 0,
        };
        self.learning_paths.insert(id, path);
    }

    pub fn enroll_student(&mut self, path_id: &str) -> bool {
        if let Some(path) = self.learning_paths.get_mut(path_id) {
            path.active_students += 1;
            true
        } else {
            false
        }
    }

    pub fn add_university_partner(&mut self, partner: UniversityPartnership) {
        self.university_partners.push(partner);
    }

    pub fn audit_document(&mut self, doc: DocAsset) {
        self.documentation_pool.push(doc);
    }

    pub fn get_compliant_doc_ratio(&self) -> f64 {
        if self.documentation_pool.is_empty() {
            return 1.0;
        }
        let passing = self
            .documentation_pool
            .iter()
            .filter(|d| d.passes_lint())
            .count();
        passing as f64 / self.documentation_pool.len() as f64
    }
}

impl Default for EducationOutreachManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_paths_and_enrollment() {
        let mut manager = EducationOutreachManager::new();
        manager.register_path(
            "SCSA-101".to_string(),
            "SigmaOS Certified Systems Administrator".to_string(),
            vec![
                "Capabilities".to_string(),
                "Post-Quantum Cryptography".to_string(),
            ],
            "SCSA".to_string(),
        );

        assert_eq!(
            manager
                .learning_paths
                .get("SCSA-101")
                .unwrap()
                .active_students,
            0
        );
        assert!(manager.enroll_student("SCSA-101"));
        assert_eq!(
            manager
                .learning_paths
                .get("SCSA-101")
                .unwrap()
                .active_students,
            1
        );
        assert!(!manager.enroll_student("NONEXISTENT"));
    }

    #[test]
    fn test_university_partnership() {
        let mut manager = EducationOutreachManager::new();
        let partner = UniversityPartnership {
            institution: "IIT Bombay".to_string(),
            department: "Computer Science & Engineering".to_string(),
            course_name: "CS-403: Sovereign Microkernel Operating Systems".to_string(),
            research_focus: Some("Capabilities verification".to_string()),
            dynamic_collaboration_active: true,
        };
        manager.add_university_partner(partner);
        assert_eq!(manager.university_partners.len(), 1);
        assert_eq!(manager.university_partners[0].institution, "IIT Bombay");
    }

    #[test]
    fn test_documentation_auditor() {
        let mut manager = EducationOutreachManager::new();
        let good_doc = DocAsset {
            filename: "INSTALL.md".to_string(),
            format: DocFormat::MarkdownGuide,
            has_standard_frontmatter: true,
            code_examples_count: 5,
            is_fully_localized: true,
        };
        let bad_doc = DocAsset {
            filename: "hacks.wiki".to_string(),
            format: DocFormat::WikiArticle,
            has_standard_frontmatter: false,
            code_examples_count: 0,
            is_fully_localized: false,
        };

        manager.audit_document(good_doc);
        manager.audit_document(bad_doc);

        assert_eq!(manager.documentation_pool.len(), 2);
        assert_eq!(manager.get_compliant_doc_ratio(), 0.5);
    }
}
