// SPDX-License-Identifier: MIT
// Integration test for Sovereign OS Documentation & GitHub Wiki Consolidation Engine

#[path = "../src/community/sovereign_documentation_consolidation_pr_engine.rs"]
mod sovereign_documentation_consolidation_pr_engine;

use sovereign_documentation_consolidation_pr_engine::*;
use std::fs;

#[test]
fn test_man_pages_consolidation_standalone() {
    let mut engine = SovereignManPagesConsolidationEngine::new();
    assert!(!engine.pages.is_empty());

    engine.register_page(SovereignManPage {
        title: "test-cmd".to_string(),
        section: ManSection::UserCommands,
        synopsis: "test-cmd [FLAGS]".to_string(),
        description: "Test command for man page engine.".to_string(),
        options: Default::default(),
        see_also: vec!["sigma-pkg(8)".to_string()],
        bsd_linux_source: "Linux test suite".to_string(),
    });

    let results = engine.search("test-cmd");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].section, ManSection::UserCommands);

    let md = results[0].to_markdown();
    assert!(md.contains("# test-cmd(1)"));
    assert!(md.contains("SYNOPSIS"));

    let catalog = engine.generate_catalog_markdown();
    assert!(catalog.contains("Consolidated Man Pages Catalog"));
    assert!(catalog.contains("sigma-sh"));
}

#[test]
fn test_info_pages_consolidation_standalone() {
    let mut engine = SovereignInfoPagesConsolidationEngine::new();
    assert!(engine.nodes.contains_key("Top"));

    engine.register_node(SovereignInfoNode {
        title: "Kernel Micro-architecture".to_string(),
        node_name: "Kernel Architecture".to_string(),
        up_node: "Top".to_string(),
        next_node: "Packaging System".to_string(),
        prev_node: "Top".to_string(),
        content: "Detailed info node regarding kernel micro-architecture.".to_string(),
        menu_items: vec![],
    });

    assert!(engine.nodes.contains_key("Kernel Architecture"));
    let node = &engine.nodes["Kernel Architecture"];
    let md = node.to_markdown();
    assert!(md.contains("Info Node: Kernel Micro-architecture"));
    assert!(md.contains("Up: `[Top]`"));

    let tree = engine.generate_tree_markdown();
    assert!(tree.contains("GNU Info Pages Texinfo Navigation Tree"));
}

#[test]
fn test_usr_share_doc_consolidation_standalone() {
    let engine = SovereignUsrShareDocConsolidationEngine::new();
    assert!(engine.docs.contains_key("sigma-core"));

    let doc = &engine.docs["sigma-core"];
    let md = doc.to_markdown();
    assert!(md.contains("# Package Documentation: sigma-core (v1.0.0)"));
    assert!(md.contains("CHANGELOG"));
    assert!(md.contains("COPYRIGHT & LICENSE"));

    let registry = engine.generate_registry_markdown();
    assert!(registry.contains("/usr/share/doc Offline Package Docs Registry"));
}

#[test]
fn test_wiki_handbook_consolidation_standalone() {
    let engine = SovereignWikiHandbookConsolidationEngine::new();
    assert!(engine.articles.contains_key("Installation-Guide"));

    let article = &engine.articles["Installation-Guide"];
    let md = article.to_markdown();
    assert!(md.contains("SigmaOS Installation & Deployment Guide"));
    assert!(md.contains("Troubleshooting & FAQ"));

    let handbook = engine.generate_handbook_markdown();
    assert!(handbook.contains("Consolidated OS Documentation & Handbook System"));
}

#[test]
fn test_pr_gateway_generation_standalone() {
    let gateway = SovereignDocumentationConsolidationPrGateway::new();
    let md = gateway.generate_consolidated_pr_markdown();

    assert!(md.contains("# Pull Request Proposal: Consolidated Operating System Documentation"));
    assert!(md.contains("Man Pages (Sections 1..9)"));
    assert!(md.contains("GNU Info Pages"));
    assert!(md.contains("/usr/share/doc"));
    assert!(md.contains("Arch Wiki & FreeBSD Handbook"));

    let temp_dir = std::env::temp_dir().join("sigma_doc_test_export_integration");
    let files = gateway.export_all_wiki_pages(&temp_dir).expect("Wiki export failed");
    assert_eq!(files.len(), 4);
    assert!(temp_dir.join("Man-Pages-Catalog.md").exists());
    assert!(temp_dir.join("Info-Pages-Tree.md").exists());
    assert!(temp_dir.join("Package-Docs-Registry.md").exists());
    assert!(temp_dir.join("Consolidated-OS-Documentation-System.md").exists());

    let _ = fs::remove_dir_all(&temp_dir);

    // Export to project wiki directory
    let wiki_files = gateway.export_all_wiki_pages("wiki").expect("Project wiki export failed");
    assert_eq!(wiki_files.len(), 4);
}
