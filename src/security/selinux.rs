// SELinux-style Security Policy Framework
// Linux-style mandatory access control with policy enforcement

#![no_std]

extern crate alloc;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityContext {
    Unconfined,
    System,
    User,
    Guest,
    Container,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SelinuxPermission {
    Read,
    Write,
    Execute,
    Create,
    Delete,
    Connect,
    Bind,
    Accept,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectType {
    File,
    Directory,
    Socket,
    Process,
    Network,
    Device,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityLabel {
    pub user: String,
    pub role: String,
    pub type_: String,
    pub level: String,
}

impl SecurityLabel {
    /// Parse a security context string (format user:role:type:level)
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() < 3 {
            return Err("Invalid context format. Expected at least user:role:type");
        }
        let user = parts[0].to_string();
        let role = parts[1].to_string();
        let type_ = parts[2].to_string();
        let level = if parts.len() >= 4 {
            parts[3..].join(":")
        } else {
            "s0".to_string()
        };

        Ok(Self {
            user,
            role,
            type_,
            level,
        })
    }

    /// Convert security label to standard context string representation
    pub fn to_context_string(&self) -> String {
        if self.level.is_empty() {
            alloc::format!("{}:{}:{}", self.user, self.role, self.type_)
        } else {
            alloc::format!("{}:{}:{}:{}", self.user, self.role, self.type_, self.level)
        }
    }

    /// Parse the level string into (sensitivity_numeric, categories_set)
    pub fn parse_level(&self) -> (u32, BTreeSet<u16>) {
        let level_str = self.level.trim();
        if level_str.is_empty() {
            return (0, BTreeSet::new());
        }

        // Handle sensitivity ranges, e.g. s0-s15:c0.c1023
        // We use the high level (the last part after '-') for dominance comparison
        let main_level = level_str.split('-').last().unwrap_or(level_str);
        let parts: Vec<&str> = main_level.split(':').collect();

        let sens_str = parts[0].trim();
        let mut sensitivity = 0;
        if sens_str.starts_with('s') {
            if let Ok(val) = sens_str[1..].parse::<u32>() {
                sensitivity = val;
            }
        }

        let mut categories = BTreeSet::new();
        if parts.len() > 1 {
            let cats_str = parts[1];
            for part in cats_str.split(',') {
                let part = part.trim();
                if part.contains('.') {
                    let range_parts: Vec<&str> = part.split('.').collect();
                    if range_parts.len() == 2 {
                        let start_str = range_parts[0].trim_start_matches('c');
                        let end_str = range_parts[1].trim_start_matches('c');
                        if let (Ok(start), Ok(end)) =
                            (start_str.parse::<u16>(), end_str.parse::<u16>())
                        {
                            for cat in start..=end {
                                categories.insert(cat);
                            }
                        }
                    }
                } else {
                    let cat_val = part.trim_start_matches('c');
                    if let Ok(val) = cat_val.parse::<u16>() {
                        categories.insert(val);
                    }
                }
            }
        }

        (sensitivity, categories)
    }

    /// Check if this security label dominates another (MLS/MCS dominance)
    /// L1 dominates L2 iff sensitivity(L1) >= sensitivity(L2) and categories(L1) is a superset of categories(L2)
    pub fn dominates(&self, other: &Self) -> bool {
        let (self_sens, self_cats) = self.parse_level();
        let (other_sens, other_cats) = other.parse_level();

        if self_sens < other_sens {
            return false;
        }

        for cat in &other_cats {
            if !self_cats.contains(cat) {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for SecurityLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_context_string())
    }
}

#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub source: SecurityLabel,
    pub target: SecurityLabel,
    pub object_type: ObjectType,
    pub permissions: Vec<SelinuxPermission>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct SelinuxBoolean {
    pub name: String,
    pub value: bool,
    pub default_value: bool,
}

#[derive(Debug, Clone)]
pub struct TypeTransitionRule {
    pub source_type: String,
    pub target_type: String,
    pub object_type: ObjectType,
    pub new_type: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AvcKey {
    pub source_type: String,
    pub target_type: String,
    pub object_type: ObjectType,
    pub permission: SelinuxPermission,
}

#[derive(Debug, Clone)]
pub struct AccessVectorCache {
    cache: BTreeMap<AvcKey, bool>,
    hits: usize,
    misses: usize,
}

impl AccessVectorCache {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    pub fn get(&mut self, key: &AvcKey) -> Option<bool> {
        if let Some(&allowed) = self.cache.get(key) {
            self.hits += 1;
            Some(allowed)
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn insert(&mut self, key: AvcKey, allowed: bool) {
        self.cache.insert(key, allowed);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn stats(&self) -> (usize, usize) {
        (self.hits, self.misses)
    }
}

impl Default for AccessVectorCache {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SecurityPolicy {
    rules: Vec<SecurityRule>,
    conditional_rules: Vec<(SecurityRule, String, bool)>, // (Rule, Boolean name, Expected boolean value)
    booleans: BTreeMap<String, SelinuxBoolean>,
    type_transitions: Vec<TypeTransitionRule>,
    avc: AccessVectorCache,
    audit_logs: Vec<String>,
    default_context: SecurityContext,
    enforcing_mode: bool,
}

impl SecurityPolicy {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            conditional_rules: Vec::new(),
            booleans: BTreeMap::new(),
            type_transitions: Vec::new(),
            avc: AccessVectorCache::new(),
            audit_logs: Vec::new(),
            default_context: SecurityContext::Unconfined,
            enforcing_mode: true,
        }
    }

    /// Add a security rule
    pub fn add_rule(&mut self, rule: SecurityRule) -> Result<(), &'static str> {
        self.rules.push(rule);
        self.avc.clear(); // invalidate cache on rule change
        Ok(())
    }

    /// Add a conditional security rule
    pub fn add_conditional_rule(
        &mut self,
        rule: SecurityRule,
        boolean_name: &str,
        expected_value: bool,
    ) -> Result<(), &'static str> {
        self.conditional_rules
            .push((rule, boolean_name.to_string(), expected_value));
        self.avc.clear();
        Ok(())
    }

    /// Add a domain type transition rule
    pub fn add_type_transition(
        &mut self,
        transition: TypeTransitionRule,
    ) -> Result<(), &'static str> {
        self.type_transitions.push(transition);
        Ok(())
    }

    /// Query if a type transition exists
    pub fn find_transition(
        &self,
        source_type: &str,
        target_type: &str,
        object_type: ObjectType,
    ) -> Option<String> {
        for rule in &self.type_transitions {
            if rule.source_type == source_type
                && rule.target_type == target_type
                && rule.object_type == object_type
            {
                return Some(rule.new_type.clone());
            }
        }
        None
    }

    /// Check if a permission is allowed (integrated with MLS/MCS checks, AVC, and Audit logging)
    pub fn check_permission(
        &mut self,
        source: &SecurityLabel,
        target: &SecurityLabel,
        object_type: ObjectType,
        permission: SelinuxPermission,
    ) -> bool {
        // Construct AVC Key
        let avc_key = AvcKey {
            source_type: source.type_.clone(),
            target_type: target.type_.clone(),
            object_type,
            permission,
        };

        // AVC cache lookup
        if let Some(allowed) = self.avc.get(&avc_key) {
            if !allowed && self.enforcing_mode {
                return false;
            }
            return true;
        }

        // Core enforcement evaluation
        let allowed = self.evaluate_permission(source, target, object_type, permission);

        // Cache the computed result
        self.avc.insert(avc_key, allowed);

        // Log to Audit if denied
        if !allowed {
            let log = alloc::format!(
                "type=AVC msg=audit(1620000000.000:0): avc: denied {{ {:?} }} for scontext={} tcontext={} tclass={:?} permissive={}",
                permission,
                source.to_context_string(),
                target.to_context_string(),
                object_type,
                if self.enforcing_mode { 0 } else { 1 }
            );
            self.audit_logs.push(log);
        }

        if !self.enforcing_mode {
            return true; // Permissive mode allows everything at runtime
        }

        allowed
    }

    /// Evaluate security policy without caching/enforcing modifiers
    fn evaluate_permission(
        &self,
        source: &SecurityLabel,
        target: &SecurityLabel,
        object_type: ObjectType,
        permission: SelinuxPermission,
    ) -> bool {
        // Multi-Level Security / Multi-Category Security (MLS/MCS) dominance check
        // The source must dominate target to access it
        if !source.dominates(target) {
            return false;
        }

        // 1. Check standard rules
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            if self.labels_match(&rule.source, source)
                && self.labels_match(&rule.target, target)
                && rule.object_type == object_type
                && rule.permissions.contains(&permission)
            {
                return true;
            }
        }

        // 2. Check conditional rules
        for (rule, bool_name, expected) in &self.conditional_rules {
            if !rule.enabled {
                continue;
            }

            let bool_val = self.get_boolean(bool_name).unwrap_or(false);
            if bool_val == *expected {
                if self.labels_match(&rule.source, source)
                    && self.labels_match(&rule.target, target)
                    && rule.object_type == object_type
                    && rule.permissions.contains(&permission)
                {
                    return true;
                }
            }
        }

        // Fallback for wildcards in rules where we check if a basic match exists
        false
    }

    /// Check if security labels match
    fn labels_match(&self, rule_label: &SecurityLabel, check_label: &SecurityLabel) -> bool {
        let user_match = rule_label.user.is_empty() || rule_label.user == check_label.user;
        let role_match = rule_label.role.is_empty() || rule_label.role == check_label.role;
        let type_match = rule_label.type_.is_empty() || rule_label.type_ == check_label.type_;
        let level_match = rule_label.level.is_empty() || rule_label.level == check_label.level;

        user_match && role_match && type_match && level_match
    }

    /// Set an SELinux Boolean
    pub fn set_boolean(&mut self, name: &str, value: bool) {
        if let Some(b) = self.booleans.get_mut(name) {
            b.value = value;
        } else {
            self.booleans.insert(
                name.to_string(),
                SelinuxBoolean {
                    name: name.to_string(),
                    value,
                    default_value: value,
                },
            );
        }
        self.avc.clear(); // invalidate cache when booleans toggle
    }

    /// Get value of an SELinux Boolean
    pub fn get_boolean(&self, name: &str) -> Option<bool> {
        self.booleans.get(name).map(|b| b.value)
    }

    /// Fetch all registered Booleans
    pub fn get_booleans(&self) -> &BTreeMap<String, SelinuxBoolean> {
        &self.booleans
    }

    /// Fetch AVC statistics
    pub fn avc_stats(&self) -> (usize, usize) {
        self.avc.stats()
    }

    /// Fetch recorded audit logs
    pub fn get_audit_logs(&self) -> &[String] {
        &self.audit_logs
    }

    /// Clear recorded audit logs
    pub fn clear_audit_logs(&mut self) {
        self.audit_logs.clear();
    }

    /// Enable or disable enforcing mode
    pub fn set_enforcing(&mut self, enforcing: bool) {
        self.enforcing_mode = enforcing;
        self.avc.clear();
    }

    /// Get enforcing mode
    pub fn is_enforcing(&self) -> bool {
        self.enforcing_mode
    }

    /// Set default security context
    pub fn set_default_context(&mut self, context: SecurityContext) {
        self.default_context = context;
    }

    /// Get default security context
    pub fn default_context(&self) -> SecurityContext {
        self.default_context
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Delete a rule by index
    pub fn delete_rule(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.rules.len() {
            return Err("Rule index out of bounds");
        }
        self.rules.remove(index);
        self.avc.clear();
        Ok(())
    }

    /// Get all rules
    pub fn get_rules(&self) -> &[SecurityRule] {
        &self.rules
    }

    /// Parse and compile a complete security policy from a declarative policy definition string (distro policy load)
    pub fn load_policy(&mut self, policy_text: &str) -> Result<(), &'static str> {
        for line in policy_text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line_clean = if line.ends_with(';') {
                &line[..line.len() - 1]
            } else {
                line
            };

            let parts: Vec<&str> = line_clean.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "bool" => {
                    if parts.len() < 3 {
                        return Err("Malformed bool statement. Expected: bool <name> <value>");
                    }
                    let name = parts[1].to_string();
                    let value = parts[2]
                        .parse::<bool>()
                        .map_err(|_| "Invalid boolean value")?;
                    self.set_boolean(&name, value);
                }
                "type_transition" => {
                    if parts.len() < 4 {
                        return Err("Malformed type_transition statement. Expected: type_transition <source> <target>:<class> <new_type>");
                    }
                    let source_type = parts[1].to_string();
                    let target_class: Vec<&str> = parts[2].split(':').collect();
                    if target_class.len() != 2 {
                        return Err("Malformed target:class in type_transition");
                    }
                    let target_type = target_class[0].to_string();
                    let object_type = parse_object_type(target_class[1])?;
                    let new_type = parts[3].to_string();

                    self.add_type_transition(TypeTransitionRule {
                        source_type,
                        target_type,
                        object_type,
                        new_type,
                    })?;
                }
                "allow" => {
                    if parts.len() < 4 {
                        return Err("Malformed allow statement");
                    }
                    let src_type = parts[1];
                    let target_class: Vec<&str> = parts[2].split(':').collect();
                    if target_class.len() != 2 {
                        return Err("Malformed target:class in allow statement");
                    }
                    let tgt_type = target_class[0];
                    let object_type = parse_object_type(target_class[1])?;

                    let rest = parts[3..].join(" ");
                    let perms = parse_permissions(&rest)?;

                    let source_label = SecurityLabel {
                        user: String::new(),
                        role: String::new(),
                        type_: src_type.to_string(),
                        level: String::new(),
                    };
                    let target_label = SecurityLabel {
                        user: String::new(),
                        role: String::new(),
                        type_: tgt_type.to_string(),
                        level: String::new(),
                    };

                    let rule = SecurityRule {
                        source: source_label,
                        target: target_label,
                        object_type,
                        permissions: perms,
                        enabled: true,
                    };
                    self.add_rule(rule)?;
                }
                "allowif" => {
                    if parts.len() < 8 {
                        return Err("Malformed allowif statement");
                    }
                    let bool_name = parts[1];
                    let op = parts[2];
                    if op != "==" {
                        return Err("Unsupported operator in allowif. Only '==' is supported");
                    }
                    let expected_val = parts[3]
                        .parse::<bool>()
                        .map_err(|_| "Invalid boolean in allowif")?;

                    if parts[4] != "allow" {
                        return Err("Expected 'allow' keyword after condition in allowif");
                    }

                    let src_type = parts[5];
                    let target_class: Vec<&str> = parts[6].split(':').collect();
                    if target_class.len() != 2 {
                        return Err("Malformed target:class in allowif statement");
                    }
                    let tgt_type = target_class[0];
                    let object_type = parse_object_type(target_class[1])?;

                    let rest = parts[7..].join(" ");
                    let perms = parse_permissions(&rest)?;

                    let source_label = SecurityLabel {
                        user: String::new(),
                        role: String::new(),
                        type_: src_type.to_string(),
                        level: String::new(),
                    };
                    let target_label = SecurityLabel {
                        user: String::new(),
                        role: String::new(),
                        type_: tgt_type.to_string(),
                        level: String::new(),
                    };

                    let rule = SecurityRule {
                        source: source_label,
                        target: target_label,
                        object_type,
                        permissions: perms,
                        enabled: true,
                    };
                    self.add_conditional_rule(rule, bool_name, expected_val)?;
                }
                _ => return Err("Unknown statement in policy"),
            }
        }
        self.avc.clear();
        Ok(())
    }
}

fn parse_object_type(s: &str) -> Result<ObjectType, &'static str> {
    match s.trim().to_lowercase().as_str() {
        "file" => Ok(ObjectType::File),
        "dir" | "directory" => Ok(ObjectType::Directory),
        "socket" => Ok(ObjectType::Socket),
        "process" => Ok(ObjectType::Process),
        "network" => Ok(ObjectType::Network),
        "device" => Ok(ObjectType::Device),
        _ => Err("Unknown object type"),
    }
}

fn parse_permissions(s: &str) -> Result<Vec<SelinuxPermission>, &'static str> {
    let s_trimmed = s.trim();
    let tokens = if s_trimmed.starts_with('{') && s_trimmed.ends_with('}') {
        let content = &s_trimmed[1..s_trimmed.len() - 1];
        content.split_whitespace().collect::<Vec<&str>>()
    } else {
        s_trimmed.split_whitespace().collect::<Vec<&str>>()
    };

    let mut perms = Vec::new();
    for token in tokens {
        let clean_token = token.trim_matches(',').trim();
        if clean_token.is_empty() {
            continue;
        }
        let perm = match clean_token.to_lowercase().as_str() {
            "read" => SelinuxPermission::Read,
            "write" => SelinuxPermission::Write,
            "execute" => SelinuxPermission::Execute,
            "create" => SelinuxPermission::Create,
            "delete" => SelinuxPermission::Delete,
            "connect" => SelinuxPermission::Connect,
            "bind" => SelinuxPermission::Bind,
            "accept" => SelinuxPermission::Accept,
            _ => return Err("Unknown permission"),
        };
        perms.push(perm);
    }
    Ok(perms)
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppArmorMode {
    Enforce,
    Complain,
    Audit,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct AppArmorPathRule {
    pub path_pattern: String,
    pub permissions: String, // e.g. "rw", "rx", "r"
}

#[derive(Debug, Clone)]
pub struct AppArmorCapabilityRule {
    pub capability: String, // e.g. "sys_admin", "net_bind_service"
}

#[derive(Debug, Clone)]
pub struct AppArmorNetworkRule {
    pub domain: String,   // e.g. "inet", "inet6"
    pub protocol: String, // e.g. "tcp", "udp"
}

#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: String,
    pub path: String, // legacy backward-compatible path prefix
    pub permissions: Vec<SelinuxPermission>, // legacy backward-compatible permissions
    pub enabled: bool, // legacy backward-compatible status
    pub mode: AppArmorMode, // advanced profile mode (Ubuntu/Debian style)
    pub path_rules: Vec<AppArmorPathRule>, // advanced file path matching rules with globbing
    pub capability_rules: Vec<AppArmorCapabilityRule>, // advanced capabilities restrictions
    pub network_rules: Vec<AppArmorNetworkRule>, // advanced network socket creation controls
}

impl AppArmorProfile {
    /// Create new backward-compatible profile (legacy)
    pub fn new_legacy(
        name: String,
        path: String,
        permissions: Vec<SelinuxPermission>,
        enabled: bool,
    ) -> Self {
        Self {
            name,
            path,
            permissions,
            enabled,
            mode: if enabled {
                AppArmorMode::Enforce
            } else {
                AppArmorMode::Disabled
            },
            path_rules: Vec::new(),
            capability_rules: Vec::new(),
            network_rules: Vec::new(),
        }
    }

    /// Create new AppArmor profile with advanced rules (modern distros style)
    pub fn new(name: String, mode: AppArmorMode) -> Self {
        Self {
            name,
            path: String::new(),
            permissions: Vec::new(),
            enabled: mode != AppArmorMode::Disabled,
            mode,
            path_rules: Vec::new(),
            capability_rules: Vec::new(),
            network_rules: Vec::new(),
        }
    }

    pub fn with_path_rule(mut self, pattern: String, permissions: String) -> Self {
        self.path_rules.push(AppArmorPathRule {
            path_pattern: pattern,
            permissions,
        });
        self
    }

    pub fn with_capability_rule(mut self, capability: String) -> Self {
        self.capability_rules
            .push(AppArmorCapabilityRule { capability });
        self
    }

    pub fn with_network_rule(mut self, domain: String, protocol: String) -> Self {
        self.network_rules
            .push(AppArmorNetworkRule { domain, protocol });
        self
    }
}

/// Helper to match path patterns with simple globbing
pub fn match_path_pattern(pattern: &str, requested: &str) -> bool {
    if pattern == requested {
        return true;
    }

    fn glob_match(pattern_chars: &[char], req_chars: &[char]) -> bool {
        match (pattern_chars, req_chars) {
            ([], []) => true,
            (['*', '*', tail @ ..], _) => {
                for i in 0..=req_chars.len() {
                    if glob_match(tail, &req_chars[i..]) {
                        return true;
                    }
                }
                false
            }
            (['*', tail @ ..], _) => {
                if req_chars.is_empty() {
                    return glob_match(tail, req_chars);
                }
                for i in 0..=req_chars.len() {
                    if i > 0 && req_chars[i - 1] == '/' {
                        break;
                    }
                    if glob_match(tail, &req_chars[i..]) {
                        return true;
                    }
                }
                false
            }
            ([p, p_tail @ ..], [r, r_tail @ ..]) if *p == *r => glob_match(p_tail, r_tail),
            _ => false,
        }
    }

    let p_chars: Vec<char> = pattern.chars().collect();
    let r_chars: Vec<char> = requested.chars().collect();
    glob_match(&p_chars, &r_chars)
}

#[derive(Debug, Clone)]
pub struct AppArmorAuditLog {
    pub profile_name: String,
    pub mode: AppArmorMode,
    pub action: String,
    pub target: String,
    pub allowed: bool,
}

pub struct AppArmorManager {
    profiles: BTreeMap<String, AppArmorProfile>,
    enforcing_mode: bool,
    pub audit_logs: Vec<AppArmorAuditLog>,
}

impl AppArmorManager {
    pub fn new() -> Self {
        Self {
            profiles: BTreeMap::new(),
            enforcing_mode: true,
            audit_logs: Vec::new(),
        }
    }

    /// Add an AppArmor profile
    pub fn add_profile(&mut self, profile: AppArmorProfile) -> Result<(), &'static str> {
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    /// Check if a path is allowed by its profile (backward-compatible legacy fallback)
    pub fn check_path(&self, path: &str, permission: SelinuxPermission) -> bool {
        if !self.enforcing_mode {
            return true;
        }

        for profile in self.profiles.values() {
            if path.starts_with(&profile.path)
                && profile.enabled
                && profile.permissions.contains(&permission)
            {
                return true;
            }
        }

        false
    }

    /// Check file path access under a specific profile with globbing support (Ubuntu style)
    pub fn check_file_access(
        &mut self,
        profile_name: &str,
        requested_path: &str,
        requested_perm: char,
    ) -> bool {
        let profile = match self.profiles.get(profile_name) {
            Some(p) => p,
            None => return !self.enforcing_mode,
        };

        if profile.mode == AppArmorMode::Disabled || !profile.enabled {
            return true;
        }

        let mut allowed = false;
        for rule in &profile.path_rules {
            if match_path_pattern(&rule.path_pattern, requested_path) {
                if rule.permissions.contains(requested_perm) {
                    allowed = true;
                    break;
                }
            }
        }

        // Log to audit if Complain, Audit, or Enforce fails
        if profile.mode == AppArmorMode::Audit || !allowed || profile.mode == AppArmorMode::Complain
        {
            let log = AppArmorAuditLog {
                profile_name: profile_name.to_string(),
                mode: profile.mode,
                action: alloc::format!("file_access:{}", requested_perm),
                target: requested_path.to_string(),
                allowed: allowed || profile.mode == AppArmorMode::Complain,
            };
            self.audit_logs.push(log);
        }

        if profile.mode == AppArmorMode::Complain {
            return true; // Complain mode always allows but logs warning
        }

        allowed
    }

    /// Check capability access (restrict process privileges)
    pub fn check_capability(&mut self, profile_name: &str, capability: &str) -> bool {
        let profile = match self.profiles.get(profile_name) {
            Some(p) => p,
            None => return !self.enforcing_mode,
        };

        if profile.mode == AppArmorMode::Disabled || !profile.enabled {
            return true;
        }

        let allowed = profile
            .capability_rules
            .iter()
            .any(|r| r.capability == capability);

        if profile.mode == AppArmorMode::Audit || !allowed || profile.mode == AppArmorMode::Complain
        {
            let log = AppArmorAuditLog {
                profile_name: profile_name.to_string(),
                mode: profile.mode,
                action: "capability".to_string(),
                target: capability.to_string(),
                allowed: allowed || profile.mode == AppArmorMode::Complain,
            };
            self.audit_logs.push(log);
        }

        if profile.mode == AppArmorMode::Complain {
            return true;
        }

        allowed
    }

    /// Check network socket creation (restrict socket domain/protocols)
    pub fn check_network(&mut self, profile_name: &str, domain: &str, protocol: &str) -> bool {
        let profile = match self.profiles.get(profile_name) {
            Some(p) => p,
            None => return !self.enforcing_mode,
        };

        if profile.mode == AppArmorMode::Disabled || !profile.enabled {
            return true;
        }

        let allowed = profile
            .network_rules
            .iter()
            .any(|r| r.domain == domain && r.protocol == protocol);

        if profile.mode == AppArmorMode::Audit || !allowed || profile.mode == AppArmorMode::Complain
        {
            let log = AppArmorAuditLog {
                profile_name: profile_name.to_string(),
                mode: profile.mode,
                action: "network".to_string(),
                target: alloc::format!("{}:{}", domain, protocol),
                allowed: allowed || profile.mode == AppArmorMode::Complain,
            };
            self.audit_logs.push(log);
        }

        if profile.mode == AppArmorMode::Complain {
            return true;
        }

        allowed
    }

    /// Enable or disable enforcing mode
    pub fn set_enforcing(&mut self, enforcing: bool) {
        self.enforcing_mode = enforcing;
    }

    /// Get enforcing mode
    pub fn is_enforcing(&self) -> bool {
        self.enforcing_mode
    }

    /// Get profile count
    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }

    /// Delete a profile
    pub fn delete_profile(&mut self, name: &str) -> Result<(), &'static str> {
        self.profiles.remove(name).ok_or("Profile not found")?;
        Ok(())
    }

    /// Get all profiles
    pub fn get_profiles(&self) -> Vec<&AppArmorProfile> {
        self.profiles.values().collect()
    }
}

impl Default for AppArmorManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_context_parsing() {
        let label = SecurityLabel::parse("system_u:system_r:httpd_t:s0-s0:c0.c3").unwrap();
        assert_eq!(label.user, "system_u");
        assert_eq!(label.role, "system_r");
        assert_eq!(label.type_, "httpd_t");
        assert_eq!(label.level, "s0-s0:c0.c3");

        let (sens, cats) = label.parse_level();
        assert_eq!(sens, 0);
        assert!(cats.contains(&0));
        assert!(cats.contains(&1));
        assert!(cats.contains(&2));
        assert!(cats.contains(&3));
        assert!(!cats.contains(&4));
    }

    #[test]
    fn test_mls_dominance() {
        let label_high = SecurityLabel::parse("system_u:system_r:admin_t:s2:c0.c10").unwrap();
        let label_low = SecurityLabel::parse("system_u:system_r:user_t:s1:c2.c5").unwrap();
        let label_unrelated = SecurityLabel::parse("system_u:system_r:user_t:s1:c11").unwrap();

        // High sensitivity and superset of categories dominates low
        assert!(label_high.dominates(&label_low));
        // Low does not dominate high
        assert!(!label_low.dominates(&label_high));
        // High does not dominate unrelated because c11 is not in high's categories (c0..=c10)
        assert!(!label_high.dominates(&label_unrelated));
    }

    #[test]
    fn test_avc_hits_misses() {
        let mut policy = SecurityPolicy::new();
        let source = SecurityLabel::parse("system_u:system_r:init_t:s0").unwrap();
        let target = SecurityLabel::parse("system_u:object_r:etc_t:s0").unwrap();

        let rule = SecurityRule {
            source: source.clone(),
            target: target.clone(),
            object_type: ObjectType::File,
            permissions: vec![SelinuxPermission::Read],
            enabled: true,
        };
        policy.add_rule(rule).unwrap();

        // First check: miss
        let allowed1 =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(allowed1);
        let (hits1, misses1) = policy.avc_stats();
        assert_eq!(hits1, 0);
        assert_eq!(misses1, 1);

        // Second check: hit
        let allowed2 =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(allowed2);
        let (hits2, misses2) = policy.avc_stats();
        assert_eq!(hits2, 1);
        assert_eq!(misses2, 1);

        // Clear cache by updating boolean/adding rule
        policy.set_boolean("any_bool", true);
        let (hits3, misses3) = policy.avc_stats();
        // Stats are retained, but cache is cleared. Next check will miss.
        let allowed3 =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(allowed3);
        let (hits4, misses4) = policy.avc_stats();
        assert_eq!(hits4, 1);
        assert_eq!(misses4, 2);
    }

    #[test]
    fn test_conditional_booleans() {
        let mut policy = SecurityPolicy::new();
        let source = SecurityLabel::parse("system_u:system_r:httpd_t:s0").unwrap();
        let target = SecurityLabel::parse("system_u:object_r:user_home_t:s0").unwrap();

        let rule = SecurityRule {
            source: source.clone(),
            target: target.clone(),
            object_type: ObjectType::File,
            permissions: vec![SelinuxPermission::Read],
            enabled: true,
        };

        policy
            .add_conditional_rule(rule, "httpd_enable_homedirs", true)
            .unwrap();

        // Boolean is not registered/false by default
        let allowed1 =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(!allowed1);

        // Toggle boolean
        policy.set_boolean("httpd_enable_homedirs", true);
        let allowed2 =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(allowed2);
    }

    #[test]
    fn test_type_transitions() {
        let mut policy = SecurityPolicy::new();
        policy
            .add_type_transition(TypeTransitionRule {
                source_type: "init_t".to_string(),
                target_type: "apache_exec_t".to_string(),
                object_type: ObjectType::Process,
                new_type: "apache_t".to_string(),
            })
            .unwrap();

        let new_domain = policy.find_transition("init_t", "apache_exec_t", ObjectType::Process);
        assert_eq!(new_domain, Some("apache_t".to_string()));

        let no_domain = policy.find_transition("init_t", "other_exec_t", ObjectType::Process);
        assert_eq!(no_domain, None);
    }

    #[test]
    fn test_textual_policy_loading() {
        let mut policy = SecurityPolicy::new();
        let policy_str = r#"
            # This is a comment
            bool httpd_enable_homedirs false;

            type_transition init_t apache_exec_t:process apache_t;

            allow init_t etc_t:file { read write };
            allowif httpd_enable_homedirs == true allow httpd_t user_home_t:file { read };
        "#;

        policy.load_policy(policy_str).unwrap();

        // Verify boolean
        assert_eq!(policy.get_boolean("httpd_enable_homedirs"), Some(false));

        // Verify type transition
        let trans = policy.find_transition("init_t", "apache_exec_t", ObjectType::Process);
        assert_eq!(trans, Some("apache_t".to_string()));

        // Verify rules loaded
        assert_eq!(policy.rule_count(), 1);

        // Check rule evaluation
        let source = SecurityLabel::parse("system_u:system_r:init_t:s0").unwrap();
        let target = SecurityLabel::parse("system_u:object_r:etc_t:s0").unwrap();
        assert!(policy.check_permission(
            &source,
            &target,
            ObjectType::File,
            SelinuxPermission::Read
        ));
    }

    #[test]
    fn test_permissive_mode_and_audit() {
        let mut policy = SecurityPolicy::new();
        let source = SecurityLabel::parse("system_u:system_r:httpd_t:s0").unwrap();
        let target = SecurityLabel::parse("system_u:object_r:shadow_t:s0").unwrap();

        // Enforcing: denied
        let allowed1 =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(!allowed1);
        assert_eq!(policy.get_audit_logs().len(), 1);
        assert!(policy.get_audit_logs()[0].contains("denied"));
        assert!(policy.get_audit_logs()[0].contains("permissive=0"));

        // Permissive: allowed but logged
        policy.set_enforcing(false);
        policy.clear_audit_logs();
        let allowed2 =
            policy.check_permission(&source, &target, ObjectType::File, SelinuxPermission::Read);
        assert!(allowed2);
        assert_eq!(policy.get_audit_logs().len(), 1);
        assert!(policy.get_audit_logs()[0].contains("denied"));
        assert!(policy.get_audit_logs()[0].contains("permissive=1"));
    }

    #[test]
    fn test_apparmor_manager() {
        let mut manager = AppArmorManager::new();

        let profile = AppArmorProfile {
            name: "test_profile".to_string(),
            path: "/etc/".to_string(),
            permissions: vec![SelinuxPermission::Read, SelinuxPermission::Write],
            enabled: true,
            mode: AppArmorMode::Enforce,
            path_rules: Vec::new(),
            capability_rules: Vec::new(),
            network_rules: Vec::new(),
        };

        manager.add_profile(profile).unwrap();
        assert_eq!(manager.profile_count(), 1);

        let allowed = manager.check_path("/etc/passwd", SelinuxPermission::Read);
        assert!(allowed);
    }

    #[test]
    fn test_apparmor_enforcing() {
        let mut manager = AppArmorManager::new();

        manager.set_enforcing(false);
        assert!(!manager.is_enforcing());

        manager.set_enforcing(true);
        assert!(manager.is_enforcing());
    }

    #[test]
    fn test_delete_rule() {
        let mut policy = SecurityPolicy::new();

        let source = SecurityLabel {
            user: "system_u".to_string(),
            role: "system_r".to_string(),
            type_: "system_t".to_string(),
            level: "s0".to_string(),
        };

        let target = SecurityLabel {
            user: "system_u".to_string(),
            role: "object_r".to_string(),
            type_: "etc_t".to_string(),
            level: "s0".to_string(),
        };

        let rule = SecurityRule {
            source,
            target,
            object_type: ObjectType::File,
            permissions: vec![SelinuxPermission::Read],
            enabled: true,
        };

        policy.add_rule(rule).unwrap();
        policy.delete_rule(0).unwrap();

        assert_eq!(policy.rule_count(), 0);
    }

    #[test]
    fn test_apparmor_glob_matching() {
        // Test single asterisk '*' wildcard (matches within directories)
        assert!(match_path_pattern("/var/log/*", "/var/log/syslog"));
        assert!(match_path_pattern("/var/log/*", "/var/log/auth.log"));
        assert!(!match_path_pattern(
            "/var/log/*",
            "/var/log/nginx/access.log"
        )); // should fail (recursive path)

        // Test double asterisk '**' wildcard (matches recursively)
        assert!(match_path_pattern(
            "/home/**/*.txt",
            "/home/ubuntu/notes.txt"
        ));
        assert!(match_path_pattern(
            "/home/**/*.txt",
            "/home/guest/documents/secret.txt"
        ));
        assert!(!match_path_pattern(
            "/home/**/*.txt",
            "/home/ubuntu/notes.pdf"
        )); // wrong extension
    }

    #[test]
    fn test_apparmor_modes_and_auditing() {
        let mut manager = AppArmorManager::new();

        // 1. Enforce mode - strictly denies access if no rule matches
        let profile_enforce =
            AppArmorProfile::new("restricted_bin".to_string(), AppArmorMode::Enforce)
                .with_path_rule("/var/log/*.log".to_string(), "r".to_string());
        manager.add_profile(profile_enforce).unwrap();

        assert!(manager.check_file_access("restricted_bin", "/var/log/syslog.log", 'r'));
        assert!(!manager.check_file_access("restricted_bin", "/etc/shadow", 'r')); // denied

        // 2. Complain mode - allows access but logs a warning
        let profile_complain =
            AppArmorProfile::new("complain_bin".to_string(), AppArmorMode::Complain)
                .with_path_rule("/var/log/*.log".to_string(), "r".to_string());
        manager.add_profile(profile_complain).unwrap();

        // Should be allowed and audited
        assert!(manager.check_file_access("complain_bin", "/etc/shadow", 'r'));
        assert_eq!(manager.audit_logs.len(), 2); // 1 from restricted_bin deny, 1 from complain_bin bypass
        assert_eq!(manager.audit_logs[1].profile_name, "complain_bin");
        assert_eq!(manager.audit_logs[1].mode, AppArmorMode::Complain);
        assert!(manager.audit_logs[1].allowed); // Complain allows
    }

    #[test]
    fn test_apparmor_capabilities_and_networks() {
        let mut manager = AppArmorManager::new();

        let profile = AppArmorProfile::new("networking_daemon".to_string(), AppArmorMode::Enforce)
            .with_capability_rule("net_bind_service".to_string())
            .with_network_rule("inet".to_string(), "tcp".to_string());
        manager.add_profile(profile).unwrap();

        // Verify capability restriction
        assert!(manager.check_capability("networking_daemon", "net_bind_service"));
        assert!(!manager.check_capability("networking_daemon", "sys_admin")); // denied

        // Verify network restriction
        assert!(manager.check_network("networking_daemon", "inet", "tcp"));
        assert!(!manager.check_network("networking_daemon", "inet", "udp")); // denied
    }
}
