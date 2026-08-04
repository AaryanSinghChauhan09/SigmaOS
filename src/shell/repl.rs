// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell with full desktop GUI-parity and defensive auditing commands

use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use crate::accessibility::{
    AccessibilityCategory, AccessibilityFeature, AccessibilityFramework, AccessibilityProfile,
    AccessibilitySetting,
};
use crate::compatibility::{
    ApplicationBinary, BinaryFormat, CompatibilityManager, CompatibilityMode, TargetPlatform,
};
use crate::customization::{CustomizationEngine, Theme};
use crate::dashboard::{MetricType, SystemMonitor, UnifiedDashboard, WidgetType};
use crate::package::{PackageFormat, PackageSource, UnifiedPackage, UniversalPackageManager};
use crate::resilience::{RecoveryAction, RecoveryEventType, RecoveryRule, SelfHealingModule};
use crate::virtualization::{
    Container, ResourcePool, VirtualMachine, VirtualizationOrchestrator, VirtualizationTech,
    VmState,
};
