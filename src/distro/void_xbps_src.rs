#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Void Linux xbps Source-Bootstrap & Transactional Rebuild Model
// Extends the existing Void `runit` parity with the *xbps-src* side: building
// packages from source against a `XBPS_HOSTDIR`, resolving the dependency DAG,
// and producing reproducible binary packages. This rounds out the Void Linux
// inspiration beyond the init system already modelled in compatibility/void_linux.rs.

use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XbpsTemplatePhase {
    Fetch,
    Extract,
    Configure,
    Build,
    Install,
    Pkg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XbpsTemplate {
    pub pkgname: String,
    pub version: String,
    pub build_depends: Vec<String>,
    pub hostmakedepends: Vec<String>,
    pub short_desc: String,
    pub current_phase: XbpsTemplatePhase,
}

impl XbpsTemplate {
    pub fn new(pkgname: &str, version: &str) -> Self {
        XbpsTemplate {
            pkgname: pkgname.to_string(),
            version: version.to_string(),
            build_depends: Vec::new(),
            hostmakedepends: Vec::new(),
            short_desc: String::new(),
            current_phase: XbpsTemplatePhase::Fetch,
        }
    }

    pub fn add_build_depend(&mut self, dep: &str) {
        if !self.build_depends.iter().any(|d| d == dep) {
            self.build_depends.push(dep.to_string());
        }
    }

    /// Advance the build to the next phase, returning the new phase.
    pub fn advance_phase(&mut self) -> XbpsTemplatePhase {
        let next = match self.current_phase {
            XbpsTemplatePhase::Fetch => XbpsTemplatePhase::Extract,
            XbpsTemplatePhase::Extract => XbpsTemplatePhase::Configure,
            XbpsTemplatePhase::Configure => XbpsTemplatePhase::Build,
            XbpsTemplatePhase::Build => XbpsTemplatePhase::Install,
            XbpsTemplatePhase::Install => XbpsTemplatePhase::Pkg,
            XbpsTemplatePhase::Pkg => XbpsTemplatePhase::Pkg,
        };
        self.current_phase = next;
        self.current_phase
    }

    pub fn is_complete(&self) -> bool {
        self.current_phase == XbpsTemplatePhase::Pkg
    }

    pub fn pkg_filename(&self) -> String {
        format!("{}-{}.xbps", self.pkgname, self.version)
    }
}

/// Topologically-ordered build planner over xbps templates. Detects cycles and
/// yields a build order where every dependency precedes its dependents.
pub struct XbpsBootstrapPlanner {
    pub templates: HashMap<String, XbpsTemplate>,
    pub order: Vec<String>,
}

impl XbpsBootstrapPlanner {
    pub fn new() -> Self {
        XbpsBootstrapPlanner {
            templates: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn add_template(&mut self, t: XbpsTemplate) {
        self.templates.insert(t.pkgname.clone(), t);
    }

    /// Produce a deterministic build order, or Err if the dependency graph has a cycle.
    ///
    /// A node's in-degree is the number of packages it depends on. We process
    /// zero-in-degree packages first and, when one is emitted, decrement the
    /// in-degree of every package that *depends on it* (its successors). This is
    /// the correct direction for Kahn's algorithm: a dependent can only build
    /// after all of its dependencies have been emitted.
    pub fn plan(&mut self) -> Result<&Vec<String>, String> {
        let mut indegree: HashMap<String, usize> = HashMap::new();
        // successors[dep] = packages that depend on `dep`.
        let mut successors: HashMap<String, Vec<String>> = HashMap::new();
        for (name, t) in self.templates.iter() {
            indegree.entry(name.clone()).or_insert(0);
            for dep in t.build_depends.iter().chain(t.hostmakedepends.iter()) {
                if self.templates.contains_key(dep) {
                    *indegree.entry(name.clone()).or_insert(0) += 1;
                    successors
                        .entry(dep.clone())
                        .or_default()
                        .push(name.clone());
                }
            }
        }
        // Seed queue with zero-in-degree nodes (deterministic order via sorted names).
        let mut queue: Vec<String> = indegree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(k, _)| k.clone())
            .collect();
        queue.sort();

        let mut produced: Vec<String> = Vec::new();
        while let Some(node) = queue.first().cloned() {
            queue.remove(0);
            produced.push(node.clone());
            // Decrement the in-degree of every package that depends on `node`.
            if let Some(succs) = successors.get(&node) {
                for s in succs {
                    if let Some(d) = indegree.get_mut(s) {
                        if *d > 0 {
                            *d -= 1;
                            if *d == 0 {
                                queue.push(s.clone());
                            }
                        }
                    }
                }
            }
            queue.sort();
        }

        if produced.len() != self.templates.len() {
            return Err("Dependency cycle detected in xbps bootstrap graph".to_string());
        }
        self.order = produced;
        Ok(&self.order)
    }

    pub fn build_order(&self) -> &Vec<String> {
        &self.order
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_template_phases_and_filename() {
        let mut t = XbpsTemplate::new("sovereign-libc", "2.38");
        t.add_build_depend("sovereign-base");
        assert_eq!(t.pkg_filename(), "sovereign-libc-2.38.xbps");
        assert_eq!(t.current_phase, XbpsTemplatePhase::Fetch);
        t.advance_phase();
        assert_eq!(t.current_phase, XbpsTemplatePhase::Extract);
        for _ in 0..4 {
            t.advance_phase();
        }
        assert!(t.is_complete());
    }

    #[test]
    fn test_topological_order() {
        let mut planner = XbpsBootstrapPlanner::new();
        planner.add_template(XbpsTemplate::new("base", "1.0"));
        let mut glibc = XbpsTemplate::new("libc", "2.38");
        glibc.add_build_depend("base");
        planner.add_template(glibc);
        let mut bash = XbpsTemplate::new("bash", "5.2");
        bash.add_build_depend("libc");
        planner.add_template(bash);

        let order = planner.plan().unwrap().clone();
        assert_eq!(order[0], "base");
        assert_eq!(order[1], "libc");
        assert_eq!(order[2], "bash");
        let pos = |n: &str| order.iter().position(|x| x == n).unwrap();
        assert!(pos("base") < pos("libc"));
        assert!(pos("libc") < pos("bash"));
    }

    #[test]
    fn test_cycle_detection() {
        let mut planner = XbpsBootstrapPlanner::new();
        let mut a = XbpsTemplate::new("a", "1");
        a.add_build_depend("b");
        let mut b = XbpsTemplate::new("b", "1");
        b.add_build_depend("a");
        planner.add_template(a);
        planner.add_template(b);
        assert!(planner.plan().is_err());
    }
}
