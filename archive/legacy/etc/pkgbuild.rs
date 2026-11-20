// SPDX-License-Identifier: MIT
// SigmaOS PKGBUILD System - Arch Linux Inspired
// Implements PKGBUILD-style package building similar to Arch's makepkg

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// PKGBUILD package sources
#[derive(Debug, Clone, PartialEq)]
pub enum PkgSource {
    Url(String),
    Git { url: String, branch: Option<String> },
    Hg { url: String, branch: Option<String> },
    Svn { url: String, revision: Option<String> },
    File(String),
    Local(String),
}

impl PkgSource {
    pub fn url(url: impl Into<String>) -> Self {
        PkgSource::Url(url.into())
    }

    pub fn git(url: impl Into<String>, branch: Option<impl Into<String>>) -> Self {
        PkgSource::Git {
            url: url.into(),
            branch: branch.map(|b| b.into()),
        }
    }

    pub fn hg(url: impl Into<String>, branch: Option<impl Into<String>>) -> Self {
        PkgSource::Hg {
            url: url.into(),
            branch: branch.map(|b| b.into()),
        }
    }

    pub fn svn(url: impl Into<String>, revision: Option<impl Into<String>>) -> Self {
        PkgSource::Svn {
            url: url.into(),
            revision: revision.map(|r| r.into()),
        }
    }

    pub fn file(file: impl Into<String>) -> Self {
        PkgSource::File(file.into())
    }

    pub fn local(path: impl Into<String>) -> Self {
        PkgSource::Local(path.into())
    }

    pub fn get_source_string(&self) -> String {
        match self {
            PkgSource::Url(url) => url.clone(),
            PkgSource::Git { url, branch } => {
                if let Some(branch) = branch {
                    format!("{}#branch={}", url, branch)
                } else {
                    url.clone()
                }
            }
            PkgSource::Hg { url, branch } => {
                if let Some(branch) = branch {
                    format!("{}#branch={}", url, branch)
                } else {
                    url.clone()
                }
            }
            PkgSource::Svn { url, revision } => {
                if let Some(rev) = revision {
                    format!("{}#revision={}", url, rev)
                } else {
                    url.clone()
                }
            }
            PkgSource::File(file) => file.clone(),
            PkgSource::Local(path) => path.clone(),
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, PkgSource::File(_) | PkgSource::Local(_))
    }

    pub fn is_vcs(&self) -> bool {
        matches!(self, PkgSource::Git { .. } | PkgSource::Hg { .. } | PkgSource::Svn { .. })
    }
}

/// PKGBUILD package information
#[derive(Debug, Clone)]
pub struct PkgInfo {
    pub pkgname: String,
    pub pkgbase: String,
    pub pkgver: String,
    pub pkgrel: u32,
    pub epoch: u32,
    pub pkgdesc: String,
    pub arch: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub makedepends: Vec<String>,
    pub checkdepends: Vec<String>,
    pub optdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub backup: Vec<String>,
    pub options: Vec<String>,
    pub install: Option<String>,
    pub changelog: Option<String>,
    pub source: Vec<PkgSource>,
    pub noextract: Vec<String>,
    pub md5sums: Vec<String>,
    pub sha1sums: Vec<String>,
    pub sha256sums: Vec<String>,
    pub sha512sums: Vec<String>,
    pub b2sums: Vec<String>,
}

impl PkgInfo {
    /// Create a new package info
    pub fn new(pkgname: impl Into<String>, pkgver: impl Into<String>) -> Self {
        Self {
            pkgname: pkgname.into(),
            pkgbase: String::new(),
            pkgver: pkgver.into(),
            pkgrel: 1,
            epoch: 0,
            pkgdesc: String::new(),
            arch: vec!["x86_64".to_string()],
            url: String::new(),
            license: Vec::new(),
            groups: Vec::new(),
            depends: Vec::new(),
            makedepends: Vec::new(),
            checkdepends: Vec::new(),
            optdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            options: Vec::new(),
            install: None,
            changelog: None,
            source: Vec::new(),
            noextract: Vec::new(),
            md5sums: Vec::new(),
            sha1sums: Vec::new(),
            sha256sums: Vec::new(),
            sha512sums: Vec::new(),
            b2sums: Vec::new(),
        }
    }

    /// Set package base
    pub fn pkgbase(mut self, pkgbase: impl Into<String>) -> Self {
        self.pkgbase = pkgbase.into();
        self
    }

    /// Set package release
    pub fn pkgrel(mut self, pkgrel: u32) -> Self {
        self.pkgrel = pkgrel;
        self
    }

    /// Set epoch
    pub fn epoch(mut self, epoch: u32) -> Self {
        self.epoch = epoch;
        self
    }

    /// Set package description
    pub fn pkgdesc(mut self, desc: impl Into<String>) -> Self {
        self.pkgdesc = desc.into();
        self
    }

    /// Set architecture
    pub fn arch(mut self, arch: impl Into<String>) -> Self {
        self.arch.push(arch.into());
        self
    }

    /// Set URL
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Add license
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license.push(license.into());
        self
    }

    /// Add group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.groups.push(group.into());
        self
    }

    /// Add dependency
    pub fn depends(mut self, dep: impl Into<String>) -> Self {
        self.depends.push(dep.into());
        self
    }

    /// Add makedependency
    pub fn makedepends(mut self, dep: impl Into<String>) -> Self {
        self.makedepends.push(dep.into());
        self
    }

    /// Add checkdependency
    pub fn checkdepends(mut self, dep: impl Into<String>) -> Self {
        self.checkdepends.push(dep.into());
        self
    }

    /// Add optdependency
    pub fn optdepends(mut self, dep: impl Into<String>) -> Self {
        self.optdepends.push(dep.into());
        self
    }

    /// Add provides
    pub fn provides(mut self, provides: impl Into<String>) -> Self {
        self.provides.push(provides.into());
        self
    }

    /// Add conflicts
    pub fn conflicts(mut self, conflict: impl Into<String>) -> Self {
        self.conflicts.push(conflict.into());
        self
    }

    /// Add replaces
    pub fn replaces(mut self, replaces: impl Into<String>) -> Self {
        self.replaces.push(replaces.into());
        self
    }

    /// Add backup file
    pub fn backup(mut self, file: impl Into<String>) -> Self {
        self.backup.push(file.into());
        self
    }

    /// Add option
    pub fn option(mut self, option: impl Into<String>) -> Self {
        self.options.push(option.into());
        self
    }

    /// Set install script
    pub fn install(mut self, script: impl Into<String>) -> Self {
        self.install = Some(script.into());
        self
    }

    /// Set changelog
    pub fn changelog(mut self, changelog: impl Into<String>) -> Self {
        self.changelog = Some(changelog.into());
        self
    }

    /// Add source
    pub fn source(mut self, source: PkgSource) -> Self {
        self.source.push(source);
        self
    }

    /// Add URL source
    pub fn source_url(mut self, url: impl Into<String>) -> Self {
        self.source.push(PkgSource::url(url));
        self
    }

    /// Add Git source
    pub fn source_git(mut self, url: impl Into<String>, branch: Option<impl Into<String>>) -> Self {
        self.source.push(PkgSource::git(url, branch));
        self
    }

    /// Add local source
    pub fn source_local(mut self, path: impl Into<String>) -> Self {
        self.source.push(PkgSource::local(path));
        self
    }

    /// Add noextract
    pub fn noextract(mut self, file: impl Into<String>) -> Self {
        self.noextract.push(file.into());
        self
    }

    /// Add MD5 sum
    pub fn md5sum(mut self, sum: impl Into<String>) -> Self {
        self.md5sums.push(sum.into());
        self
    }

    /// Add SHA1 sum
    pub fn sha1sum(mut self, sum: impl Into<String>) -> Self {
        self.sha1sums.push(sum.into());
        self
    }

    /// Add SHA256 sum
    pub fn sha256sum(mut self, sum: impl Into<String>) -> Self {
        self.sha256sums.push(sum.into());
        self
    }

    /// Add SHA512 sum
    pub fn sha512sum(mut self, sum: impl Into<String>) -> Self {
        self.sha512sums.push(sum.into());
        self
    }

    /// Add BLAKE2 sum
    pub fn b2sum(mut self, sum: impl Into<String>) -> Self {
        self.b2sums.push(sum.into());
        self
    }

    /// Get full package version
    pub fn get_full_version(&self) -> String {
        if self.epoch > 0 {
            format!("{}:{}-{}.{}", self.epoch, self.pkgver, self.pkgrel, self.pkgname)
        } else {
            format!("{}-{}.{}", self.pkgver, self.pkgrel, self.pkgname)
        }
    }

    /// Get package name with version
    pub fn get_pkgname_ver(&self) -> String {
        format!("{}={}", self.pkgname, self.get_full_version())
    }

    /// Check if package is split
    pub fn is_split(&self) -> bool {
        self.pkgbase != self.pkgname
    }

    /// Get package base name
    pub fn get_pkgbase(&self) -> String {
        if self.pkgbase.is_empty() {
            self.pkgname.clone()
        } else {
            self.pkgbase.clone()
        }
    }

    /// Convert to PKGBUILD format
    pub fn to_pkgbuild(&self) -> String {
        let mut content = String::new();

        content.push_str(&format!("# Maintainer: SigmaOS Packager\n"));
        content.push_str(&format!("pkgname={}\n", self.pkgname));
        if !self.pkgbase.is_empty() {
            content.push_str(&format!("pkgbase={}\n", self.pkgbase));
        }
        content.push_str(&format!("pkgver={}\n", self.pkgver));
        content.push_str(&format!("pkgrel={}\n", self.pkgrel));
        if self.epoch > 0 {
            content.push_str(&format!("epoch={}\n", self.epoch));
        }
        content.push_str(&format!("pkgdesc=\"{}\"\n", self.pkgdesc));
        content.push_str(&format!("arch=(\"{}\")\n", self.arch.join("\" \"")));
        content.push_str(&format!("url=\"{}\"\n", self.url));
        content.push_str(&format!("license=(\"{}\")\n", self.license.join("\" \"")));

        if !self.groups.is_empty() {
            content.push_str(&format!("groups=(\"{}\")\n", self.groups.join("\" \"")));
        }
        if !self.depends.is_empty() {
            content.push_str(&format!("depends=(\"{}\")\n", self.depends.join("\" \"")));
        }
        if !self.makedepends.is_empty() {
            content.push_str(&format!("makedepends=(\"{}\")\n", self.makedepends.join("\" \"")));
        }
        if !self.checkdepends.is_empty() {
            content.push_str(&format!("checkdepends=(\"{}\")\n", self.checkdepends.join("\" \"")));
        }
        if !self.optdepends.is_empty() {
            content.push_str(&format!("optdepends=(\"{}\")\n", self.optdepends.join("\" \"")));
        }
        if !self.provides.is_empty() {
            content.push_str(&format!("provides=(\"{}\")\n", self.provides.join("\" \"")));
        }
        if !self.conflicts.is_empty() {
            content.push_str(&format!("conflicts=(\"{}\")\n", self.conflicts.join("\" \"")));
        }
        if !self.replaces.is_empty() {
            content.push_str(&format!("replaces=(\"{}\")\n", self.replaces.join("\" \"")));
        }
        if !self.backup.is_empty() {
            content.push_str(&format!("backup=(\"{}\")\n", self.backup.join("\" \"")));
        }
        if !self.options.is_empty() {
            content.push_str(&format!("options=(\"{}\")\n", self.options.join("\" \"")));
        }
        if let Some(install) = &self.install {
            content.push_str(&format!("install={}\n", install));
        }
        if let Some(changelog) = &self.changelog {
            content.push_str(&format!("changelog={}\n", changelog));
        }

        content.push_str("\nsource=(\n");
        for source in &self.source {
            content.push_str(&format!("    \"{}\"\n", source.get_source_string()));
        }
        content.push_str(")\n");

        if !self.noextract.is_empty() {
            content.push_str(&format!("noextract=(\"{}\")\n", self.noextract.join("\" \"")));
        }

        if !self.md5sums.is_empty() {
            content.push_str(&format!("md5sums=(\"{}\")\n", self.md5sums.join("\" \"")));
        }
        if !self.sha1sums.is_empty() {
            content.push_str(&format!("sha1sums=(\"{}\")\n", self.sha1sums.join("\" \"")));
        }
        if !self.sha256sums.is_empty() {
            content.push_str(&format!("sha256sums=(\"{}\")\n", self.sha256sums.join("\" \"")));
        }
        if !self.sha512sums.is_empty() {
            content.push_str(&format!("sha512sums=(\"{}\")\n", self.sha512sums.join("\" \"")));
        }
        if !self.b2sums.is_empty() {
            content.push_str(&format!("b2sums=(\"{}\")\n", self.b2sums.join("\" \"")));
        }

        content.push_str("\n# Build functions\n");
        content.push_str("prepare() {\n");
        content.push_str("    :\n");
        content.push_str("}\n\n");
        content.push_str("build() {\n");
        content.push_str("    :\n");
        content.push_str("}\n\n");
        content.push_str("check() {\n");
        content.push_str("    :\n");
        content.push_str("}\n\n");
        content.push_str("package() {\n");
        content.push_str("    :\n");
        content.push_str("}\n");

        content
    }
}

/// PKGBUILD parser
#[derive(Debug, Clone)]
pub struct PkgbuildParser {
    pub pkgbuild_path: PathBuf,
    pub pkginfo: Option<PkgInfo>,
    pub build_functions: BTreeMap<String, String>,
}

impl PkgbuildParser {
    /// Create a new parser
    pub fn new(pkgbuild_path: impl Into<PathBuf>) -> Self {
        Self {
            pkgbuild_path: pkgbuild_path.into(),
            pkginfo: None,
            build_functions: BTreeMap::new(),
        }
    }

    /// Parse PKGBUILD file
    pub fn parse(&mut self) -> Result<(), String> {
        let content = fs::read_to_string(&self.pkgbuild_path)
            .map_err(|e| format!("Failed to read PKGBUILD: {}", e))?;

        self.parse_content(&content)?;
        Ok(())
    }

    /// Parse PKGBUILD content
    pub fn parse_content(&mut self, content: &str) -> Result<(), String> {
        let mut pkginfo = PkgInfo::new("unknown", "0.0.0");
        let mut in_function = false;
        let mut current_function = String::new();
        let mut function_content = String::new();

        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Check for function definitions
            if line.ends_with("() {") {
                let func_name = line.trim_end_matches("() {").trim();
                if ["prepare", "build", "check", "package"].contains(&func_name) {
                    in_function = true;
                    current_function = func_name.to_string();
                    function_content.clear();
                    continue;
                }
            }

            // Check for function end
            if in_function && line == "}" {
                in_function = false;
                self.build_functions.insert(current_function.clone(), function_content.clone());
                continue;
            }

            if in_function {
                function_content.push_str(line);
                function_content.push('\n');
                continue;
            }

            // Parse variable assignments
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "pkgname" => {
                        pkginfo.pkgname = self.parse_string_array(value)?.first().cloned().unwrap_or_default();
                    }
                    "pkgbase" => {
                        pkginfo.pkgbase = self.parse_string_array(value)?.first().cloned().unwrap_or_default();
                    }
                    "pkgver" => {
                        pkginfo.pkgver = self.parse_string(value)?;
                    }
                    "pkgrel" => {
                        pkginfo.pkgrel = self.parse_int(value)?;
                    }
                    "epoch" => {
                        pkginfo.epoch = self.parse_int(value)?;
                    }
                    "pkgdesc" => {
                        pkginfo.pkgdesc = self.parse_string(value)?;
                    }
                    "arch" => {
                        pkginfo.arch = self.parse_string_array(value)?;
                    }
                    "url" => {
                        pkginfo.url = self.parse_string(value)?;
                    }
                    "license" => {
                        pkginfo.license = self.parse_string_array(value)?;
                    }
                    "groups" => {
                        pkginfo.groups = self.parse_string_array(value)?;
                    }
                    "depends" => {
                        pkginfo.depends = self.parse_string_array(value)?;
                    }
                    "makedepends" => {
                        pkginfo.makedepends = self.parse_string_array(value)?;
                    }
                    "checkdepends" => {
                        pkginfo.checkdepends = self.parse_string_array(value)?;
                    }
                    "optdepends" => {
                        pkginfo.optdepends = self.parse_string_array(value)?;
                    }
                    "provides" => {
                        pkginfo.provides = self.parse_string_array(value)?;
                    }
                    "conflicts" => {
                        pkginfo.conflicts = self.parse_string_array(value)?;
                    }
                    "replaces" => {
                        pkginfo.replaces = self.parse_string_array(value)?;
                    }
                    "backup" => {
                        pkginfo.backup = self.parse_string_array(value)?;
                    }
                    "options" => {
                        pkginfo.options = self.parse_string_array(value)?;
                    }
                    "install" => {
                        pkginfo.install = Some(self.parse_string(value)?);
                    }
                    "changelog" => {
                        pkginfo.changelog = Some(self.parse_string(value)?);
                    }
                    "source" => {
                        pkginfo.source = self.parse_source_array(value)?;
                    }
                    "noextract" => {
                        pkginfo.noextract = self.parse_string_array(value)?;
                    }
                    "md5sums" => {
                        pkginfo.md5sums = self.parse_string_array(value)?;
                    }
                    "sha1sums" => {
                        pkginfo.sha1sums = self.parse_string_array(value)?;
                    }
                    "sha256sums" => {
                        pkginfo.sha256sums = self.parse_string_array(value)?;
                    }
                    "sha512sums" => {
                        pkginfo.sha512sums = self.parse_string_array(value)?;
                    }
                    "b2sums" => {
                        pkginfo.b2sums = self.parse_string_array(value)?;
                    }
                    _ => {}
                }
            }
        }

        self.pkginfo = Some(pkginfo);
        Ok(())
    }

    /// Parse string value
    fn parse_string(&self, value: &str) -> Result<String, String> {
        let value = value.trim();
        if value.starts_with('"') && value.ends_with('"') {
            Ok(value[1..value.len()-1].to_string())
        } else if value.starts_with('\'') && value.ends_with('\'') {
            Ok(value[1..value.len()-1].to_string())
        } else {
            Ok(value.to_string())
        }
    }

    /// Parse integer value
    fn parse_int(&self, value: &str) -> Result<u32, String> {
        let value = value.trim();
        value.parse::<u32>().map_err(|e| format!("Invalid integer: {}", e))
    }

    /// Parse string array
    fn parse_string_array(&self, value: &str) -> Result<Vec<String>, String> {
        let value = value.trim();
        if !value.starts_with('(') || !value.ends_with(')') {
            return Err("Invalid array format".to_string());
        }

        let inner = &value[1..value.len()-1];
        let mut items = Vec::new();

        // Simple parsing - could be improved for complex cases
        for item in inner.split_whitespace() {
            let item = item.trim_matches('"').trim_matches('\'');
            if !item.is_empty() {
                items.push(item.to_string());
            }
        }

        Ok(items)
    }

    /// Parse source array
    fn parse_source_array(&self, value: &str) -> Result<Vec<PkgSource>, String> {
        let value = value.trim();
        if !value.starts_with('(') || !value.ends_with(')') {
            return Err("Invalid source array format".to_string());
        }

        let inner = &value[1..value.len()-1];
        let mut sources = Vec::new();

        // Parse each source
        for item in inner.split('"') {
            let item = item.trim();
            if item.is_empty() || item == "(" || item == ")" {
                continue;
            }

            // Determine source type
            if item.starts_with("http://") || item.starts_with("https://") || item.starts_with("ftp://") {
                sources.push(PkgSource::url(item.to_string()));
            } else if item.contains(".git") {
                sources.push(PkgSource::git(item.to_string(), None));
            } else if item.ends_with(".patch") || item.ends_with(".diff") {
                sources.push(PkgSource::file(item.to_string()));
            } else {
                sources.push(PkgSource::local(item.to_string()));
            }
        }

        Ok(sources)
    }

    /// Get package info
    pub fn get_pkginfo(&self) -> Option<&PkgInfo> {
        self.pkginfo.as_ref()
    }

    /// Get build function
    pub fn get_build_function(&self, name: &str) -> Option<&String> {
        self.build_functions.get(name)
    }

    /// Get all build functions
    pub fn get_build_functions(&self) -> &BTreeMap<String, String> {
        &self.build_functions
    }
}

/// Package builder
#[derive(Debug, Clone)]
pub struct PkgBuilder {
    pub pkgbuild_dir: PathBuf,
    pub src_dir: PathBuf,
    pub pkg_dir: PathBuf,
    pub build_dir: PathBuf,
    pub use_ccache: bool,
    pub use_sccache: bool,
    pub parallel_jobs: Option<u32>,
    pub skip_checks: bool,
    pub skip_source: bool,
    pub skip_build: bool,
    pub skip_package: bool,
    pub clean_build: bool,
    pub last_build: Option<u64>,
}

impl PkgBuilder {
    /// Create a new package builder
    pub fn new(pkgbuild_dir: impl Into<PathBuf>) -> Self {
        let pkgbuild_dir = pkgbuild_dir.into();
        let pkgname = pkgbuild_dir.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("package")
            .to_string();

        Self {
            pkgbuild_dir: pkgbuild_dir.clone(),
            src_dir: pkgbuild_dir.join("src"),
            pkg_dir: pkgbuild_dir.join("pkg"),
            build_dir: pkgbuild_dir.join("src"),
            use_ccache: true,
            use_sccache: false,
            parallel_jobs: None,
            skip_checks: false,
            skip_source: false,
            skip_build: false,
            skip_package: false,
            clean_build: false,
            last_build: None,
        }
    }

    /// Set parallel jobs
    pub fn jobs(mut self, jobs: u32) -> Self {
        self.parallel_jobs = Some(jobs);
        self
    }

    /// Enable/disable ccache
    pub fn ccache(mut self, enable: bool) -> Self {
        self.use_ccache = enable;
        self
    }

    /// Enable/disable sccache
    pub fn sccache(mut self, enable: bool) -> Self {
        self.use_sccache = enable;
        self
    }

    /// Skip checks
    pub fn skip_checks(mut self, skip: bool) -> Self {
        self.skip_checks = skip;
        self
    }

    /// Skip source download
    pub fn skip_source(mut self, skip: bool) -> Self {
        self.skip_source = skip;
        self
    }

    /// Skip build
    pub fn skip_build(mut self, skip: bool) -> Self {
        self.skip_build = skip;
        self
    }

    /// Skip package
    pub fn skip_package(mut self, skip: bool) -> Self {
        self.skip_package = skip;
        self
    }

    /// Clean build
    pub fn clean_build(mut self, clean: bool) -> Self {
        self.clean_build = clean;
        self
    }

    /// Parse PKGBUILD
    pub fn parse_pkgbuild(&self) -> Result<PkgInfo, String> {
        let pkgbuild_path = self.pkgbuild_dir.join("PKGBUILD");
        let mut parser = PkgbuildParser::new(pkgbuild_path);
        parser.parse()?;
        parser.pkginfo.ok_or_else(|| "Failed to parse PKGBUILD".to_string())
    }

    /// Download sources
    pub fn download_sources(&self, pkginfo: &PkgInfo) -> Result<(), String> {
        if self.skip_source {
            return Ok(());
        }

        // Create src directory
        fs::create_dir_all(&self.src_dir).map_err(|e| format!("Failed to create src dir: {}", e))?;

        for source in &pkginfo.source {
            match source {
                PkgSource::Url(url) => self.download_url(url)?,
                PkgSource::Git { url, branch } => self.download_git(url, branch.as_deref())?,
                PkgSource::Hg { url, branch } => self.download_hg(url, branch.as_deref())?,
                PkgSource::Svn { url, revision } => self.download_svn(url, revision.as_deref())?,
                PkgSource::File(file) => self.copy_local_file(file)?,
                PkgSource::Local(path) => self.copy_local_file(path)?,
            }
        }

        Ok(())
    }

    /// Download URL source
    fn download_url(&self, url: &str) -> Result<(), String> {
        let filename = url.split('/').last().unwrap_or("download");
        let dest = self.src_dir.join(filename);

        // In real implementation, would use wget or curl
        // For now, just create a placeholder
        fs::write(&dest, format!("Downloaded from: {}", url))
            .map_err(|e| format!("Failed to download {}: {}", url, e))?;

        Ok(())
    }

    /// Download Git source
    fn download_git(&self, url: &str, branch: Option<&str>) -> Result<(), String> {
        let repo_name = url.split('/').last().unwrap_or("repo").trim_end_matches(".git");
        let dest = self.src_dir.join(repo_name);

        // In real implementation, would use git clone
        // For now, just create a placeholder
        fs::create_dir_all(&dest).map_err(|e| format!("Failed to create git dir: {}", e))?;
        fs::write(dest.join(".git_placeholder"), "Git repository placeholder")
            .map_err(|e| format!("Failed to create git placeholder: {}", e))?;

        Ok(())
    }

    /// Download Mercurial source
    fn download_hg(&self, url: &str, branch: Option<&str>) -> Result<(), String> {
        let repo_name = url.split('/').last().unwrap_or("repo").trim_end_matches(".hg");
        let dest = self.src_dir.join(repo_name);

        // In real implementation, would use hg clone
        // For now, just create a placeholder
        fs::create_dir_all(&dest).map_err(|e| format!("Failed to create hg dir: {}", e))?;
        fs::write(dest.join(".hg_placeholder"), "Mercurial repository placeholder")
            .map_err(|e| format!("Failed to create hg placeholder: {}", e))?;

        Ok(())
    }

    /// Download SVN source
    fn download_svn(&self, url: &str, revision: Option<&str>) -> Result<(), String> {
        let repo_name = url.split('/').last().unwrap_or("repo").trim_end_matches(".svn");
        let dest = self.src_dir.join(repo_name);

        // In real implementation, would use svn checkout
        // For now, just create a placeholder
        fs::create_dir_all(&dest).map_err(|e| format!("Failed to create svn dir: {}", e))?;
        fs::write(dest.join(".svn_placeholder"), "SVN repository placeholder")
            .map_err(|e| format!("Failed to create svn placeholder: {}", e))?;

        Ok(())
    }

    /// Copy local file
    fn copy_local_file(&self, path: &str) -> Result<(), String> {
        let src = Path::new(path);
        if !src.exists() {
            return Err(format!("Local file not found: {}", path));
        }

        let filename = src.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file");
        let dest = self.src_dir.join(filename);

        fs::copy(src, dest).map_err(|e| format!("Failed to copy {}: {}", path, e))?;
        Ok(())
    }

    /// Verify checksums
    pub fn verify_checksums(&self, pkginfo: &PkgInfo) -> Result<(), String> {
        // In real implementation, would verify checksums
        // For now, just check if checksums are provided
        if pkginfo.md5sums.is_empty() && pkginfo.sha256sums.is_empty() && pkginfo.sha512sums.is_empty() {
            return Ok(()); // No checksums to verify
        }

        // Verify each source has corresponding checksum
        for (i, source) in pkginfo.source.iter().enumerate() {
            if source.is_local() {
                continue; // Skip local files
            }

            let has_checksum = if !pkginfo.md5sums.is_empty() {
                pkginfo.md5sums.get(i).is_some()
            } else if !pkginfo.sha256sums.is_empty() {
                pkginfo.sha256sums.get(i).is_some()
            } else if !pkginfo.sha512sums.is_empty() {
                pkginfo.sha512sums.get(i).is_some()
            } else {
                true // No checksums provided for this source
            };

            if !has_checksum {
                return Err(format!("Missing checksum for source {}", i));
            }
        }

        Ok(())
    }

    /// Extract sources
    pub fn extract_sources(&self, pkginfo: &PkgInfo) -> Result<(), String> {
        // Create build directory
        fs::create_dir_all(&self.build_dir).map_err(|e| format!("Failed to create build dir: {}", e))?;

        for source in &pkginfo.source {
            if source.is_local() {
                continue; // Skip local files
            }

            // Extract archive
            self.extract_source(source)?;
        }

        Ok(())
    }

    /// Extract a single source
    fn extract_source(&self, source: &PkgSource) -> Result<(), String> {
        let source_str = source.get_source_string();
        let filename = source_str.split('/').last().unwrap_or("source");
        let src_path = self.src_dir.join(filename);

        if !src_path.exists() {
            return Err(format!("Source not found: {}", src_path.display()));
        }

        // Determine extraction command based on file type
        let extract_cmd = if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
            vec!["tar", "-xzf", src_path.to_str().unwrap(), "-C", self.build_dir.to_str().unwrap()]
        } else if filename.ends_with(".tar.bz2") || filename.ends_with(".tbz2") {
            vec!["tar", "-xjf", src_path.to_str().unwrap(), "-C", self.build_dir.to_str().unwrap()]
        } else if filename.ends_with(".tar.xz") || filename.ends_with(".txz") {
            vec!["tar", "-xJf", src_path.to_str().unwrap(), "-C", self.build_dir.to_str().unwrap()]
        } else if filename.ends_with(".zip") {
            vec!["unzip", "-q", src_path.to_str().unwrap(), "-d", self.build_dir.to_str().unwrap()]
        } else if filename.ends_with(".tar") {
            vec!["tar", "-xf", src_path.to_str().unwrap(), "-C", self.build_dir.to_str().unwrap()]
        } else {
            // Unknown format, just copy
            fs::copy(&src_path, self.build_dir.join(filename))
                .map_err(|e| format!("Failed to copy source: {}", e))?;
            return Ok(());
        };

        // Execute extraction
        let output = Command::new(extract_cmd[0])
            .args(&extract_cmd[1..])
            .output()
            .map_err(|e| format!("Failed to extract {}: {}", filename, e))?;

        if !output.status.success() {
            return Err(format!("Extraction failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Run prepare function
    pub fn run_prepare(&self, pkginfo: &PkgInfo) -> Result<(), String> {
        if self.skip_build {
            return Ok(());
        }

        // Execute prepare() function if defined
        if let Some(prepare_func) = self.get_prepare_function(pkginfo) {
            self.execute_build_function("prepare", &prepare_func)?;
        }

        Ok(())
    }

    /// Run build function
    pub fn run_build(&self, pkginfo: &PkgInfo) -> Result<(), String> {
        if self.skip_build {
            return Ok(());
        }

        // Execute build() function
        if let Some(build_func) = self.get_build_function(pkginfo) {
            self.execute_build_function("build", &build_func)?;
        }

        Ok(())
    }

    /// Run check function
    pub fn run_check(&self, pkginfo: &PkgInfo) -> Result<(), String> {
        if self.skip_checks || self.skip_build {
            return Ok(());
        }

        // Execute check() function if defined
        if let Some(check_func) = self.get_check_function(pkginfo) {
            self.execute_build_function("check", &check_func)?;
        }

        Ok(())
    }

    /// Run package function
    pub fn run_package(&self, pkginfo: &PkgInfo) -> Result<(), String> {
        if self.skip_package {
            return Ok(());
        }

        // Create package directory
        fs::create_dir_all(&self.pkg_dir).map_err(|e| format!("Failed to create pkg dir: {}", e))?;

        // Execute package() function
        if let Some(package_func) = self.get_package_function(pkginfo) {
            self.execute_build_function("package", &package_func)?;
        }

        Ok(())
    }

    /// Get prepare function
    fn get_prepare_function(&self, pkginfo: &PkgInfo) -> Option<String> {
        // In real implementation, would parse PKGBUILD for prepare()
        // For now, return None
        None
    }

    /// Get build function
    fn get_build_function(&self, pkginfo: &PkgInfo) -> Option<String> {
        // In real implementation, would parse PKGBUILD for build()
        // For now, return None
        None
    }

    /// Get check function
    fn get_check_function(&self, pkginfo: &PkgInfo) -> Option<String> {
        // In real implementation, would parse PKGBUILD for check()
        // For now, return None
        None
    }

    /// Get package function
    fn get_package_function(&self, pkginfo: &PkgInfo) -> Option<String> {
        // In real implementation, would parse PKGBUILD for package()
        // For now, return None
        None
    }

    /// Execute build function
    fn execute_build_function(&self, name: &str, func: &str) -> Result<(), String> {
        // In real implementation, would execute the function in a shell
        // For now, just log the execution
        println!("Executing {}() function", name);
        println!("{}", func);
        Ok(())
    }

    /// Create package archive
    pub fn create_package_archive(&self, pkginfo: &PkgInfo) -> Result<PathBuf, String> {
        let pkgname = &pkginfo.pkgname;
        let pkgver = pkginfo.get_full_version();
        let arch = pkginfo.arch.first().cloned().unwrap_or_else(|| "x86_64".to_string());
        let filename = format!("{}-{}-{}.pkg.tar.zst", pkgname, pkgver, arch);

        let output_path = self.pkgbuild_dir.join(&filename);

        // In real implementation, would create actual package archive
        // For now, create a placeholder
        fs::write(&output_path, format!("Package: {}\nVersion: {}\nArch: {}\n", pkgname, pkgver, arch))
            .map_err(|e| format!("Failed to create package archive: {}", e))?;

        Ok(output_path)
    }

    /// Build package
    pub fn build(&mut self) -> Result<PathBuf, String> {
        // Parse PKGBUILD
        let pkginfo = self.parse_pkgbuild()?;

        // Clean build if requested
        if self.clean_build {
            self.clean_build_dirs()?;
        }

        // Download sources
        self.download_sources(&pkginfo)?;

        // Verify checksums
        self.verify_checksums(&pkginfo)?;

        // Extract sources
        self.extract_sources(&pkginfo)?;

        // Run build functions
        self.run_prepare(&pkginfo)?;
        self.run_build(&pkginfo)?;
        self.run_check(&pkginfo)?;
        self.run_package(&pkginfo)?;

        // Create package archive
        let package_path = self.create_package_archive(&pkginfo)?;

        self.last_build = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        Ok(package_path)
    }

    /// Clean build directories
    fn clean_build_dirs(&self) -> Result<(), String> {
        if self.src_dir.exists() {
            fs::remove_dir_all(&self.src_dir).map_err(|e| format!("Failed to clean src dir: {}", e))?;
        }
        if self.pkg_dir.exists() {
            fs::remove_dir_all(&self.pkg_dir).map_err(|e| format!("Failed to clean pkg dir: {}", e))?;
        }
        Ok(())
    }

    /// Get build statistics
    pub fn get_statistics(&self) -> BuildStatistics {
        let mut src_size = 0;
        let mut pkg_size = 0;

        if self.src_dir.exists() {
            src_size = self.calculate_dir_size(&self.src_dir);
        }
        if self.pkg_dir.exists() {
            pkg_size = self.calculate_dir_size(&self.pkg_dir);
        }

        BuildStatistics {
            src_size,
            pkg_size,
            last_build: self.last_build,
            use_ccache: self.use_ccache,
            use_sccache: self.use_sccache,
            parallel_jobs: self.parallel_jobs,
        }
    }

    /// Calculate directory size
    fn calculate_dir_size(&self, dir: &Path) -> u64 {
        let mut size = 0;
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    size += self.calculate_dir_size(&path);
                } else if path.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        size += metadata.len();
                    }
                }
            }
        }
        size
    }
}

impl Default for PkgBuilder {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}

/// Build statistics
#[derive(Debug, Clone)]
pub struct BuildStatistics {
    pub src_size: u64,
    pub pkg_size: u64,
    pub last_build: Option<u64>,
    pub use_ccache: bool,
    pub use_sccache: bool,
    pub parallel_jobs: Option<u32>,
}

/// Package build manager
#[derive(Debug, Clone)]
pub struct PkgBuildManager {
    pub builders: BTreeMap<String, PkgBuilder>,
    pub build_queue: Vec<String>,
    pub build_history: Vec<BuildResult>,
    pub default_options: BuildOptions,
}

impl PkgBuildManager {
    /// Create a new build manager
    pub fn new() -> Self {
        Self {
            builders: BTreeMap::new(),
            build_queue: Vec::new(),
            build_history: Vec::new(),
            default_options: BuildOptions::new(),
        }
    }

    /// Add package builder
    pub fn add_builder(&mut self, name: String, builder: PkgBuilder) {
        self.builders.insert(name, builder);
    }

    /// Remove package builder
    pub fn remove_builder(&mut self, name: &str) -> Option<PkgBuilder> {
        self.builders.remove(name)
    }

    /// Get package builder
    pub fn get_builder(&self, name: &str) -> Option<&PkgBuilder> {
        self.builders.get(name)
    }

    /// Get package builder mutably
    pub fn get_builder_mut(&mut self, name: &str) -> Option<&mut PkgBuilder> {
        self.builders.get_mut(name)
    }

    /// Queue package for build
    pub fn queue_build(&mut self, name: &str) -> Result<(), String> {
        if !self.builders.contains_key(name) {
            return Err(format!("Package builder '{}' not found", name));
        }
        if !self.build_queue.contains(&name.to_string()) {
            self.build_queue.push(name.to_string());
        }
        Ok(())
    }

    /// Remove from build queue
    pub fn dequeue_build(&mut self, name: &str) -> bool {
        if let Some(pos) = self.build_queue.iter().position(|n| n == name) {
            self.build_queue.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get build queue
    pub fn get_build_queue(&self) -> &[String] {
        &self.build_queue
    }

    /// Build queued packages
    pub fn build_queue(&mut self) -> Result<Vec<BuildResult>, String> {
        let mut results = Vec::new();
        let queue = self.build_queue.clone();

        for name in queue {
            if let Some(builder) = self.builders.get_mut(&name) {
                match builder.build() {
                    Ok(package_path) => {
                        let result = BuildResult {
                            package_name: name.clone(),
                            success: true,
                            package_path: Some(package_path),
                            error: None,
                            build_time: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs(),
                        };
                        results.push(result.clone());
                        self.build_history.push(result);
                    }
                    Err(e) => {
                        let result = BuildResult {
                            package_name: name.clone(),
                            success: false,
                            package_path: None,
                            error: Some(e.clone()),
                            build_time: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs(),
                        };
                        results.push(result.clone());
                        self.build_history.push(result);
                        return Err(e);
                    }
                }
            }
        }

        self.build_queue.clear();
        Ok(results)
    }

    /// Get build history
    pub fn get_build_history(&self) -> &[BuildResult] {
        &self.build_history
    }

    /// Clear build history
    pub fn clear_build_history(&mut self) {
        self.build_history.clear();
    }

    /// Get build statistics
    pub fn get_statistics(&self) -> BuildManagerStatistics {
        let total_builds = self.build_history.len();
        let successful_builds = self.build_history.iter().filter(|r| r.success).count();
        let failed_builds = total_builds - successful_builds;

        BuildManagerStatistics {
            total_packages: self.builders.len(),
            queued_packages: self.build_queue.len(),
            total_builds,
            successful_builds,
            failed_builds,
            success_rate: if total_builds > 0 {
                (successful_builds as f64 / total_builds as f64) * 100.0
            } else {
                0.0
            },
        }
    }

    /// Set default build options
    pub fn set_default_options(&mut self, options: BuildOptions) {
        self.default_options = options;
    }

    /// Get default build options
    pub fn get_default_options(&self) -> &BuildOptions {
        &self.default_options
    }
}

impl Default for PkgBuildManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Build options
#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub use_ccache: bool,
    pub use_sccache: bool,
    pub parallel_jobs: Option<u32>,
    pub skip_checks: bool,
    pub skip_source: bool,
    pub skip_build: bool,
    pub skip_package: bool,
    pub clean_build: bool,
    pub optimize: bool,
    pub debug: bool,
}

impl BuildOptions {
    /// Create new build options
    pub fn new() -> Self {
        Self {
            use_ccache: true,
            use_sccache: false,
            parallel_jobs: None,
            skip_checks: false,
            skip_source: false,
            skip_build: false,
            skip_package: false,
            clean_build: false,
            optimize: true,
            debug: false,
        }
    }

    /// Set parallel jobs
    pub fn jobs(mut self, jobs: u32) -> Self {
        self.parallel_jobs = Some(jobs);
        self
    }

    /// Enable/disable ccache
    pub fn ccache(mut self, enable: bool) -> Self {
        self.use_ccache = enable;
        self
    }

    /// Enable/disable sccache
    pub fn sccache(mut self, enable: bool) -> Self {
        self.use_sccache = enable;
        self
    }

    /// Skip checks
    pub fn skip_checks(mut self, skip: bool) -> Self {
        self.skip_checks = skip;
        self
    }

    /// Skip source
    pub fn skip_source(mut self, skip: bool) -> Self {
        self.skip_source = skip;
        self
    }

    /// Skip build
    pub fn skip_build(mut self, skip: bool) -> Self {
        self.skip_build = skip;
        self
    }

    /// Skip package
    pub fn skip_package(mut self, skip: bool) -> Self {
        self.skip_package = skip;
        self
    }

    /// Clean build
    pub fn clean_build(mut self, clean: bool) -> Self {
        self.clean_build = clean;
        self
    }

    /// Enable/disable optimization
    pub fn optimize(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }

    /// Enable/disable debug
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Apply to builder
    pub fn apply_to_builder(&self, builder: &mut PkgBuilder) {
        builder.use_ccache = self.use_ccache;
        builder.use_sccache = self.use_sccache;
        builder.parallel_jobs = self.parallel_jobs;
        builder.skip_checks = self.skip_checks;
        builder.skip_source = self.skip_source;
        builder.skip_build = self.skip_build;
        builder.skip_package = self.skip_package;
        builder.clean_build = self.clean_build;
    }
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub package_name: String,
    pub success: bool,
    pub package_path: Option<PathBuf>,
    pub error: Option<String>,
    pub build_time: u64,
}

/// Build manager statistics
#[derive(Debug, Clone)]
pub struct BuildManagerStatistics {
    pub total_packages: usize,
    pub queued_packages: usize,
    pub total_builds: usize,
    pub successful_builds: usize,
    pub failed_builds: usize,
    pub success_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkg_source() {
        let url_source = PkgSource::url("https://example.com/package.tar.gz");
        assert_eq!(url_source.get_source_string(), "https://example.com/package.tar.gz");
        assert!(!url_source.is_local());
        assert!(!url_source.is_vcs());

        let git_source = PkgSource::git("https://github.com/user/repo.git", Some("main"));
        assert_eq!(git_source.get_source_string(), "https://github.com/user/repo.git#branch=main");
        assert!(git_source.is_vcs());

        let local_source = PkgSource::local("/path/to/local");
        assert!(local_source.is_local());
    }

    #[test]
    fn test_pkg_info_builder() {
        let pkginfo = PkgInfo::new("test-package", "1.0.0")
            .pkgdesc("Test package description")
            .url("https://example.com")
            .license("MIT")
            .depends("dep1")
            .depends("dep2")
            .source_url("https://example.com/package.tar.gz")
            .sha256sum("abc123");

        assert_eq!(pkginfo.pkgname, "test-package");
        assert_eq!(pkginfo.pkgver, "1.0.0");
        assert_eq!(pkginfo.pkgdesc, "Test package description");
        assert_eq!(pkginfo.license.len(), 1);
        assert_eq!(pkginfo.depends.len(), 2);
        assert_eq!(pkginfo.source.len(), 1);
        assert_eq!(pkginfo.get_full_version(), "1.0.0-1.test-package");
    }

    #[test]
    fn test_pkginfo_to_pkgbuild() {
        let pkginfo = PkgInfo::new("test-package", "1.0.0")
            .pkgdesc("Test package")
            .url("https://example.com")
            .license("MIT")
            .depends("dep1")
            .source_url("https://example.com/package.tar.gz");

        let pkgbuild = pkginfo.to_pkgbuild();
        assert!(pkgbuild.contains("pkgname=test-package"));
        assert!(pkgbuild.contains("pkgver=1.0.0"));
        assert!(pkgbuild.contains("pkgdesc=\"Test package\""));
        assert!(pkgbuild.contains("url=\"https://example.com\""));
        assert!(pkgbuild.contains("license=(\"MIT\")"));
        assert!(pkgbuild.contains("depends=(\"dep1\")"));
        assert!(pkgbuild.contains("source=("));
    }

    #[test]
    fn test_pkgbuild_parser() {
        let content = r#"
pkgname=test-package
pkgver=1.0.0
pkgrel=1
pkgdesc="Test package"
arch=("x86_64")
url="https://example.com"
license=("MIT")
depends=("dep1" "dep2")
source=("https://example.com/package.tar.gz")
sha256sums=("abc123")

build() {
    cd package-1.0.0
    make
}

package() {
    cd package-1.0.0
    make install DESTDIR=$pkgdir
}
"#;

        let mut parser = PkgbuildParser::new(PathBuf::from("PKGBUILD"));
        parser.parse_content(content).unwrap();

        let pkginfo = parser.get_pkginfo().unwrap();
        assert_eq!(pkginfo.pkgname, "test-package");
        assert_eq!(pkginfo.pkgver, "1.0.0");
        assert_eq!(pkginfo.pkgdesc, "Test package");
        assert_eq!(pkginfo.depends.len(), 2);

        assert!(parser.get_build_function("build").is_some());
        assert!(parser.get_build_function("package").is_some());
    }

    #[test]
    fn test_pkg_builder() {
        let builder = PkgBuilder::new(PathBuf::from("/tmp/test-package"))
            .jobs(4)
            .ccache(true)
            .clean_build(true);

        assert_eq!(builder.parallel_jobs, Some(4));
        assert!(builder.use_ccache);
        assert!(builder.clean_build);
    }

    #[test]
    fn test_build_options() {
        let options = BuildOptions::new()
            .jobs(8)
            .ccache(true)
            .sccache(false)
            .skip_checks(true)
            .optimize(true)
            .debug(false);

        assert_eq!(options.parallel_jobs, Some(8));
        assert!(options.use_ccache);
        assert!(!options.use_sccache);
        assert!(options.skip_checks);
        assert!(options.optimize);
        assert!(!options.debug);
    }

    #[test]
    fn test_build_manager() {
        let mut manager = PkgBuildManager::new();

        let builder = PkgBuilder::new(PathBuf::from("/tmp/test1"));
        manager.add_builder("test1".to_string(), builder);

        let builder2 = PkgBuilder::new(PathBuf::from("/tmp/test2"));
        manager.add_builder("test2".to_string(), builder2);

        assert_eq!(manager.builders.len(), 2);
        assert!(manager.get_builder("test1").is_some());
        assert!(manager.get_builder("test2").is_some());
    }

    #[test]
    fn test_build_queue() {
        let mut manager = PkgBuildManager::new();

        let builder = PkgBuilder::new(PathBuf::from("/tmp/test"));
        manager.add_builder("test".to_string(), builder);

        assert!(manager.queue_build("test").is_ok());
        assert_eq!(manager.get_build_queue().len(), 1);

        assert!(manager.dequeue_build("test"));
        assert_eq!(manager.get_build_queue().len(), 0);
    }

    #[test]
    fn test_build_statistics() {
        let mut manager = PkgBuildManager::new();

        // Add some build results
        manager.build_history.push(BuildResult {
            package_name: "test1".to_string(),
            success: true,
            package_path: Some(PathBuf::from("/tmp/test1.pkg.tar.zst")),
            error: None,
            build_time: 100,
        });

        manager.build_history.push(BuildResult {
            package_name: "test2".to_string(),
            success: false,
            package_path: None,
            error: Some("Build failed".to_string()),
            build_time: 200,
        });

        let stats = manager.get_statistics();
        assert_eq!(stats.total_builds, 2);
        assert_eq!(stats.successful_builds, 1);
        assert_eq!(stats.failed_builds, 1);
        assert_eq!(stats.success_rate, 50.0);
    }
}
