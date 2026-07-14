use sigma_security::capability::Capability;
use sigma_security::enforcement::{EnforcementResult, Operation};
use sigma_security::{Policy, SigmaSecurity};

#[test]
fn test_security_capability_enforcement() {
    let mut system = SigmaSecurity::new();
    
    // Define a declarative policy (e.g., standard web server)
    let mut web_policy = Policy::new("nginx-compat");
    web_policy.allow(Capability::NetworkBind(80));
    web_policy.allow(Capability::NetworkBind(443));
    web_policy.allow(Capability::FileRead("/var/www/html".to_string()));

    // Create profile
    let profile = system.create_profile(web_policy);
    
    // Evaluate operations against the profile
    let engine = &system.enforcement_engine;
    
    // Allowed operation
    let op1 = Operation::NetworkBind(80);
    assert_eq!(engine.evaluate_operation(&profile, &op1), EnforcementResult::Allowed);
    
    // Denied operation (wrong port)
    let op2 = Operation::NetworkBind(8080);
    assert!(matches!(engine.evaluate_operation(&profile, &op2), EnforcementResult::Denied(_)));
    
    // Denied operation (hardware access)
    let op3 = Operation::HardwareAccess("gpu0".to_string());
    assert!(matches!(engine.evaluate_operation(&profile, &op3), EnforcementResult::Denied(_)));
}

#[test]
fn test_policy_validation() {
    let policy = Policy::new("test");
    let validated = policy.validate();
    assert_eq!(validated.name, "test");
}
