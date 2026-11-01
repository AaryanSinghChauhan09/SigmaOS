// tests/test_secondary_memory_lifecycle.rs
// Verification suite for SigmaOS Secondary Memory Subsystem Policies, Rules, and Lifecycles.

use std::fs;
use std::path::Path;

#[test]
fn test_storage_rules_file_exists_and_valid() {
    assert!(
        Path::new("STORAGE_RULES.toml").exists(),
        "STORAGE_RULES.toml must exist in repo root"
    );
    let content = fs::read_to_string("STORAGE_RULES.toml").expect("Failed to read STORAGE_RULES.toml");

    assert!(content.contains("[rule.storage_completeness]"));
    assert!(content.contains("[rule.no_leak]"));
    assert!(content.contains("[rule.oom_metric]"));
    assert!(content.contains("[rule.mount_integrity]"));
    assert!(content.contains("[rule.cache_semantics]"));
    assert!(content.contains("[rule.no_model_only_status]"));
    assert!(content.contains("[rule.runtime_evidence]"));
}

#[test]
fn test_secondary_memory_documentation_exists_and_valid() {
    let policy_path = Path::new("docs/storage/SECONDARY_MEMORY_POLICY.md");
    let adr_path = Path::new("docs/storage/SECONDARY_MEMORY_ADR.md");
    let block_path = Path::new("docs/storage/BLOCK_POLICY.md");
    let cache_path = Path::new("docs/filesystem/CACHE_POLICY.md");
    let mount_path = Path::new("docs/filesystem/MOUNT_POLICY.md");

    assert!(policy_path.exists(), "SECONDARY_MEMORY_POLICY.md must exist");
    assert!(adr_path.exists(), "SECONDARY_MEMORY_ADR.md must exist");
    assert!(block_path.exists(), "BLOCK_POLICY.md must exist");
    assert!(cache_path.exists(), "CACHE_POLICY.md must exist");
    assert!(mount_path.exists(), "MOUNT_POLICY.md must exist");

    let policy_content = fs::read_to_string(policy_path).unwrap();
    assert!(policy_content.contains("probe -> identify -> block layer -> cache -> filesystem -> mount ->"));
    assert!(policy_content.contains("read/write -> flush -> unmount -> detach -> error/recovery"));
    assert!(policy_content.contains("Block Layer Class"));
    assert!(policy_content.contains("Device Class"));
    assert!(policy_content.contains("Cache Class"));
    assert!(policy_content.contains("Filesystem Class"));
    assert!(policy_content.contains("Mount Policy"));
    assert!(policy_content.contains("Flush Policy"));
    assert!(policy_content.contains("Reclaim Policy"));
    assert!(policy_content.contains("Error Policy"));
    assert!(policy_content.contains("NUMA Behavior"));
    assert!(policy_content.contains("Behavior Under Storage Hotplug"));

    let adr_content = fs::read_to_string(adr_path).unwrap();
    assert!(adr_content.contains("Secondary Memory Lifecycle Subsystem"));
    assert!(adr_content.contains("Decision"));
    assert!(adr_content.contains("Consequences"));

    let block_content = fs::read_to_string(block_path).unwrap();
    assert!(block_content.contains("Race-Free Request Enqueue and Dequeue"));
    assert!(block_content.contains("Request Reuse Policy"));
    assert!(block_content.contains("Request Exhaustion Handling"));
    assert!(block_content.contains("Exactly-Once Request Completion"));
    assert!(block_content.contains("Lost-Completion Prevention"));
    assert!(block_content.contains("Double-Completion Prevention"));

    let cache_content = fs::read_to_string(cache_path).unwrap();
    assert!(cache_content.contains("Race-Free Cache Eviction"));
    assert!(cache_content.contains("Cache Reuse Policy"));
    assert!(cache_content.contains("Bounded Cache Metadata"));
    assert!(cache_content.contains("Lost-Writeback Prevention"));
    assert!(cache_content.contains("Double-Eviction Prevention"));
    assert!(cache_content.contains("Observable OOM and Watermark Metrics"));

    let mount_content = fs::read_to_string(mount_path).unwrap();
    assert!(mount_content.contains("Race-Free Mount Operations"));
    assert!(mount_content.contains("Bounded Filesystem Capacity"));
    assert!(mount_content.contains("Failure-Safe Mount / Unmount and Rollback"));
    assert!(mount_content.contains("Lost-Update Prevention"));
    assert!(mount_content.contains("Double-Unmount Prevention"));
    assert!(mount_content.contains("Observable Health and OOM Enforcement"));
}

#[test]
fn test_pull_request_template_storage_sections() {
    let pr_template_path = Path::new(".github/PULL_REQUEST_TEMPLATE.md");
    assert!(pr_template_path.exists());
    let template = fs::read_to_string(pr_template_path).unwrap();

    assert!(template.contains("## Storage component"));
    assert!(template.contains("- [ ] Block device manager"));
    assert!(template.contains("- [ ] virtio / NVMe / virtio"));
    assert!(template.contains("- [ ] Block cache manager"));
    assert!(template.contains("- [ ] Filesystem manager"));
    assert!(template.contains("## Storage safety review"));
    assert!(template.contains("Double-completion prevented"));
    assert!(template.contains("Resource limits enforced"));
}
