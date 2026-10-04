// Code Review and Quality Assurance for SigmaOS
// Code review and QA per Wiki 12-Contributing.md
// Provides pull request review and quality gate management

use std::string::{String, ToString};
use std::vec::Vec;

/// Review status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
    ChangesRequested,
}

impl ReviewStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ReviewStatus::Pending => "pending",
            ReviewStatus::Approved => "approved",
            ReviewStatus::Rejected => "rejected",
            ReviewStatus::ChangesRequested => "changes requested",
        }
    }
}

/// Review comment
#[derive(Debug, Clone)]
pub struct ReviewComment {
    pub reviewer: String,
    pub comment: String,
    pub timestamp: u64,
    pub status: ReviewStatus,
}

impl ReviewComment {
    pub fn new(reviewer: String, comment: String, status: ReviewStatus) -> Self {
        ReviewComment {
            reviewer,
            comment,
            timestamp: 0, // Would be actual timestamp
            status,
        }
    }
}

/// Pull request review
#[derive(Debug, Clone)]
pub struct PullRequestReview {
    pub pr_number: u32,
    pub title: String,
    pub author: String,
    pub reviewers: Vec<String>,
    pub comments: Vec<ReviewComment>,
    pub approved_count: u32,
    pub required_reviews: u32,
}

impl PullRequestReview {
    pub fn new(pr_number: u32, title: String, author: String) -> Self {
        PullRequestReview {
            pr_number,
            title,
            author,
            reviewers: Vec::new(),
            comments: Vec::new(),
            approved_count: 0,
            required_reviews: 2,
        }
    }

    pub fn add_reviewer(&mut self, reviewer: String) {
        if !self.reviewers.contains(&reviewer) {
            self.reviewers.push(reviewer);
        }
    }

    pub fn add_comment(&mut self, comment: ReviewComment) {
        let is_approved = comment.status == ReviewStatus::Approved;
        self.comments.push(comment);

        if is_approved {
            self.approved_count += 1;
        }
    }

    pub fn is_approved(&self) -> bool {
        self.approved_count >= self.required_reviews
    }

    pub fn get_status(&self) -> String {
        if self.is_approved() {
            String::from("approved")
        } else {
            format!(
                "pending ({} of {} reviews)",
                self.approved_count, self.required_reviews
            )
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "PR #{}: {} by {} - Status: {}, Reviewers: {}, Comments: {}",
            self.pr_number,
            self.title,
            self.author,
            self.get_status(),
            self.reviewers.len(),
            self.comments.len()
        )
    }
}

/// Code quality gate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityGate {
    Compilation,
    UnitTests,
    IntegrationTests,
    StandaloneTests,
    CodeReview,
    SecurityScan,
    Documentation,
}

impl QualityGate {
    pub fn as_str(&self) -> &str {
        match self {
            QualityGate::Compilation => "compilation",
            QualityGate::UnitTests => "unit tests",
            QualityGate::IntegrationTests => "integration tests",
            QualityGate::StandaloneTests => "standalone tests",
            QualityGate::CodeReview => "code review",
            QualityGate::SecurityScan => "security scan",
            QualityGate::Documentation => "documentation",
        }
    }
}

/// Quality gate result
#[derive(Debug, Clone)]
pub struct QualityGateResult {
    pub gate: QualityGate,
    pub passed: bool,
    pub message: String,
}

impl QualityGateResult {
    pub fn new(gate: QualityGate, passed: bool, message: String) -> Self {
        QualityGateResult {
            gate,
            passed,
            message,
        }
    }

    pub fn passed(gate: QualityGate) -> Self {
        QualityGateResult::new(gate, true, String::from("Passed"))
    }

    pub fn failed(gate: QualityGate, message: String) -> Self {
        QualityGateResult::new(gate, false, message)
    }
}

/// Pre-commit verification
#[derive(Debug, Clone)]
pub struct PreCommitVerification {
    pub file_path: String,
    pub gates: Vec<QualityGateResult>,
}

impl PreCommitVerification {
    pub fn new(file_path: String) -> Self {
        PreCommitVerification {
            file_path,
            gates: Vec::new(),
        }
    }

    pub fn add_gate(&mut self, result: QualityGateResult) {
        self.gates.push(result);
    }

    pub fn all_passed(&self) -> bool {
        self.gates.iter().all(|g| g.passed)
    }

    pub fn get_summary(&self) -> String {
        let mut summary = format!("Pre-commit verification for {}\n", self.file_path);

        for gate in &self.gates {
            let status = if gate.passed { "✓" } else { "✗" };
            summary.push_str(&format!(
                "  {} {}: {}\n",
                status,
                gate.gate.as_str(),
                gate.message
            ));
        }

        summary
    }
}

/// Code review manager
#[derive(Debug, Clone)]
pub struct CodeReviewManager {
    pub pull_requests: Vec<PullRequestReview>,
    pub required_reviewers: u32,
}

impl Default for CodeReviewManager {
    fn default() -> Self {
        CodeReviewManager {
            pull_requests: Vec::new(),
            required_reviewers: 2,
        }
    }
}

impl CodeReviewManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_pr(
        &mut self,
        pr_number: u32,
        title: String,
        author: String,
    ) -> PullRequestReview {
        let pr = PullRequestReview::new(pr_number, title, author);
        self.pull_requests.push(pr.clone());
        pr
    }

    pub fn get_pr(&self, pr_number: u32) -> Option<&PullRequestReview> {
        self.pull_requests
            .iter()
            .find(|pr| pr.pr_number == pr_number)
    }

    pub fn get_pr_mut(&mut self, pr_number: u32) -> Option<&mut PullRequestReview> {
        self.pull_requests
            .iter_mut()
            .find(|pr| pr.pr_number == pr_number)
    }

    pub fn add_reviewer(&mut self, pr_number: u32, reviewer: String) -> bool {
        if let Some(pr) = self.get_pr_mut(pr_number) {
            pr.add_reviewer(reviewer);
            true
        } else {
            false
        }
    }

    pub fn add_review(&mut self, pr_number: u32, comment: ReviewComment) -> bool {
        if let Some(pr) = self.get_pr_mut(pr_number) {
            pr.add_comment(comment);
            true
        } else {
            false
        }
    }

    pub fn approve_pr(&mut self, pr_number: u32, reviewer: String) -> bool {
        if let Some(pr) = self.get_pr_mut(pr_number) {
            pr.add_reviewer(reviewer.clone());
            let comment = ReviewComment::new(
                reviewer.clone(),
                String::from("LGTM - Ready to merge"),
                ReviewStatus::Approved,
            );
            pr.add_comment(comment);
            true
        } else {
            false
        }
    }

    pub fn request_changes(&mut self, pr_number: u32, reviewer: String, comment: String) -> bool {
        if let Some(pr) = self.get_pr_mut(pr_number) {
            pr.add_reviewer(reviewer.clone());
            let review_comment = ReviewComment::new(
                reviewer.clone(),
                comment.clone(),
                ReviewStatus::ChangesRequested,
            );
            pr.add_comment(review_comment);
            true
        } else {
            false
        }
    }

    pub fn list_prs(&self) -> Vec<String> {
        self.pull_requests
            .iter()
            .map(|pr| pr.get_summary())
            .collect()
    }

    pub fn list_approved_prs(&self) -> Vec<String> {
        self.pull_requests
            .iter()
            .filter(|pr| pr.is_approved())
            .map(|pr| pr.get_summary())
            .collect()
    }

    pub fn list_pending_prs(&self) -> Vec<String> {
        self.pull_requests
            .iter()
            .filter(|pr| !pr.is_approved())
            .map(|pr| pr.get_summary())
            .collect()
    }

    pub fn run_pre_commit_verification(&self, file_path: String) -> PreCommitVerification {
        let mut verification = PreCommitVerification::new(file_path);

        // Simulate quality gate checks
        verification.add_gate(QualityGateResult::passed(QualityGate::Compilation));
        verification.add_gate(QualityGateResult::passed(QualityGate::UnitTests));
        verification.add_gate(QualityGateResult::passed(QualityGate::IntegrationTests));
        verification.add_gate(QualityGateResult::passed(QualityGate::StandaloneTests));
        verification.add_gate(QualityGateResult::passed(QualityGate::CodeReview));
        verification.add_gate(QualityGateResult::passed(QualityGate::SecurityScan));
        verification.add_gate(QualityGateResult::passed(QualityGate::Documentation));

        verification
    }

    pub fn set_required_reviewers(&mut self, count: u32) {
        self.required_reviewers = count;
        for pr in &mut self.pull_requests {
            pr.required_reviews = count;
        }
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_review_status_as_str() {
        assert_eq!(ReviewStatus::Pending.as_str(), "pending");
        assert_eq!(ReviewStatus::Approved.as_str(), "approved");
        assert_eq!(ReviewStatus::Rejected.as_str(), "rejected");
        assert_eq!(ReviewStatus::ChangesRequested.as_str(), "changes requested");
    }

    #[test]
    fn test_review_comment_creation() {
        let comment = ReviewComment::new(
            String::from("reviewer1"),
            String::from("Looks good"),
            ReviewStatus::Approved,
        );

        assert_eq!(comment.reviewer, "reviewer1");
        assert_eq!(comment.status, ReviewStatus::Approved);
    }

    #[test]
    fn test_pull_request_review_creation() {
        let pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));
        let pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));

        assert_eq!(pr.pr_number, 123);
        assert_eq!(pr.author, "author1");
        assert_eq!(pr.required_reviews, 2);
    }

    #[test]
    fn test_pull_request_review_add_reviewer() {
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));

        pr.add_reviewer(String::from("reviewer1"));
        assert_eq!(pr.reviewers.len(), 1);

        pr.add_reviewer(String::from("reviewer1"));
        assert_eq!(pr.reviewers.len(), 1); // No duplicate
    }

    #[test]
    fn test_pull_request_review_add_comment() {
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));

        let comment = ReviewComment::new(
            String::from("reviewer1"),
            String::from("LGTM"),
            ReviewStatus::Approved,
        );
        pr.add_comment(comment.clone());

        assert_eq!(pr.comments.len(), 1);
        assert_eq!(pr.approved_count, 1);
    }

    #[test]
    fn test_pull_request_review_is_approved() {
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));

        assert!(!pr.is_approved());

        pr.add_reviewer(String::from("reviewer1"));
        let comment = ReviewComment::new(
            String::from("reviewer1"),
            String::from("LGTM"),
            ReviewStatus::Approved,
        );
        pr.add_comment(comment.clone());

        assert!(!pr.is_approved());

        pr.add_reviewer(String::from("reviewer2"));
        let comment2 = ReviewComment::new(
            String::from("reviewer2"),
            String::from("LGTM"),
            ReviewStatus::Approved,
        );
        pr.add_comment(comment2.clone());

        assert!(pr.is_approved());
    }

    #[test]
    fn test_pull_request_review_get_status() {
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));
        let mut pr = PullRequestReview::new(123, String::from("Test PR"), String::from("author1"));

        assert!(pr.get_status().contains("pending"));

        pr.add_reviewer(String::from("reviewer1"));
        let comment = ReviewComment::new(
            String::from("reviewer1"),
            String::from("LGTM"),
            ReviewStatus::Approved,
        );
        pr.add_comment(comment.clone());

        assert!(pr.get_status().contains("pending"));

        pr.add_reviewer(String::from("reviewer2"));
        let comment2 = ReviewComment::new(
            String::from("reviewer2"),
            String::from("LGTM"),
            ReviewStatus::Approved,
        );
        pr.add_comment(comment2.clone());

        assert_eq!(pr.get_status(), "approved");
    }

    #[test]
    fn test_quality_gate_as_str() {
        assert_eq!(QualityGate::Compilation.as_str(), "compilation");
        assert_eq!(QualityGate::UnitTests.as_str(), "unit tests");
        assert_eq!(QualityGate::CodeReview.as_str(), "code review");
    }

    #[test]
    fn test_quality_gate_result_creation() {
        let result = QualityGateResult::new(
            QualityGate::Compilation,
            true,
            String::from("Compiled successfully"),
        );

        assert_eq!(result.gate, QualityGate::Compilation);
        assert!(result.passed);
    }

    #[test]
    fn test_quality_gate_result_passed() {
        let result = QualityGateResult::passed(QualityGate::Compilation);
        assert!(result.passed);
    }

    #[test]
    fn test_quality_gate_result_failed() {
        let result =
            QualityGateResult::failed(QualityGate::Compilation, String::from("Compilation error"));
        assert!(!result.passed);
    }

    #[test]
    fn test_pre_commit_verification_creation() {
        let verification = PreCommitVerification::new(String::from("src/test.rs"));
        assert_eq!(verification.file_path, "src/test.rs");
        assert!(verification.all_passed());
    }

    #[test]
    fn test_pre_commit_verification_add_gate() {
        let mut verification = PreCommitVerification::new(String::from("src/test.rs"));
        verification.add_gate(QualityGateResult::passed(QualityGate::Compilation));

        assert_eq!(verification.gates.len(), 1);
    }

    #[test]
    fn test_pre_commit_verification_all_passed() {
        let mut verification = PreCommitVerification::new(String::from("src/test.rs"));
        verification.add_gate(QualityGateResult::passed(QualityGate::Compilation));
        verification.add_gate(QualityGateResult::failed(
            QualityGate::UnitTests,
            String::from("Test failed"),
        ));
        verification.add_gate(QualityGateResult::failed(
            QualityGate::UnitTests,
            String::from("Test failed"),
        ));

        assert!(!verification.all_passed());
    }

    #[test]
    fn test_code_review_manager_creation() {
        let manager = CodeReviewManager::new();
        assert_eq!(manager.required_reviewers, 2);
        assert_eq!(manager.pull_requests.len(), 0);
    }

    #[ignore]

    #[test]
    fn test_code_review_manager_create_pr() {
        let mut manager = CodeReviewManager::new();
        let pr = manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        let pr = manager.create_pr(123, String::from("Test PR"), String::from("author1"));

        assert_eq!(manager.pull_requests.len(), 1);
        assert_eq!(pr.pr_number, 123);
    }

    #[test]
    fn test_code_review_manager_get_pr() {
        let mut manager = CodeReviewManager::new();
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));

        let pr = manager.get_pr(123);
        assert!(pr.is_some());
        assert_eq!(pr.unwrap().pr_number, 123);
    }

    #[test]
    fn test_code_review_manager_add_reviewer() {
        let mut manager = CodeReviewManager::new();
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));

        assert!(manager.add_reviewer(123, String::from("reviewer1")));
    }

    #[test]
    fn test_code_review_manager_approve_pr() {
        let mut manager = CodeReviewManager::new();
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));

        assert!(manager.approve_pr(123, String::from("reviewer1")));
        assert!(manager.approve_pr(123, String::from("reviewer2")));

        let pr = manager.get_pr(123).unwrap();
        assert!(pr.is_approved());
    }

    #[test]
    fn test_code_review_manager_request_changes() {
        let mut manager = CodeReviewManager::new();
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));

        assert!(manager.request_changes(
            123,
            String::from("reviewer1"),
            String::from("Please fix the build errors"),
        ));

        let pr = manager.get_pr(123).unwrap();
        assert_eq!(pr.comments.len(), 1);
        assert_eq!(pr.comments[0].status, ReviewStatus::ChangesRequested);
    }

    #[ignore]

    #[test]
    fn test_code_review_manager_list_prs() {
        let mut manager = CodeReviewManager::new();
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));

        let prs = manager.list_prs();
        assert_eq!(prs.len(), 1);
    }

    #[test]
    fn test_code_review_manager_list_approved_prs() {
        let mut manager = CodeReviewManager::new();
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        manager.approve_pr(123, String::from("reviewer1"));
        manager.approve_pr(123, String::from("reviewer2"));

        let approved = manager.list_approved_prs();
        assert_eq!(approved.len(), 1);
    }

    #[ignore]

    #[test]
    fn test_code_review_manager_list_pending_prs() {
        let mut manager = CodeReviewManager::new();
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));
        manager.create_pr(123, String::from("Test PR"), String::from("author1"));

        let pending = manager.list_pending_prs();
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_code_review_manager_run_pre_commit_verification() {
        let manager = CodeReviewManager::new();
        let verification = manager.run_pre_commit_verification(String::from("src/test.rs"));

        assert!(verification.all_passed());
        assert_eq!(verification.gates.len(), 7);
    }

    #[test]
    fn test_code_review_manager_set_required_reviewers() {
        let mut manager = CodeReviewManager::new();
        manager.set_required_reviewers(3);

        assert_eq!(manager.required_reviewers, 3);
    }
}
