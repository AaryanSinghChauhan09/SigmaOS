// SigmaOS User-Defined Kernel Functions (UDKF) Scripting Engine
// Allows safe, in-kernel customization of allocators, scheduling algorithms, and filesystems without recompilation

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdkfHook {
    AllocatorScale,
    SchedulerWeight,
    FsCachePreload,
}

pub struct UserDefinedKernelFunctions {
    pub registered_scripts: Vec<(UdkfHook, String)>,
}

impl UserDefinedKernelFunctions {
    pub fn new() -> Self {
        UserDefinedKernelFunctions {
            registered_scripts: Vec::new(),
        }
    }

    pub fn register_function(&mut self, hook: UdkfHook, script_bytecode: &str) {
        self.registered_scripts
            .push((hook, script_bytecode.to_string()));
    }

    pub fn execute_hook(&self, hook: UdkfHook, input_val: u32) -> u32 {
        if let Some((_, script)) = self.registered_scripts.iter().find(|(h, _)| *h == hook) {
            // Simulated safe bytecode sandbox parser (e.g. evaluating basic mathematical scale actions)
            if script.contains("scale_by_2") {
                input_val * 2
            } else if script.contains("add_10") {
                input_val + 10
            } else {
                input_val
            }
        } else {
            input_val // Fallback to raw inputs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udkf_script_execution() {
        let mut engine = UserDefinedKernelFunctions::new();
        assert_eq!(engine.execute_hook(UdkfHook::AllocatorScale, 50), 50); // no script registered, return input (should be 50!)

        engine.register_function(UdkfHook::AllocatorScale, "scale_by_2".to_string().as_str());
        assert_eq!(engine.execute_hook(UdkfHook::AllocatorScale, 50), 100);

        engine.register_function(UdkfHook::SchedulerWeight, "add_10".to_string().as_str());
        assert_eq!(engine.execute_hook(UdkfHook::SchedulerWeight, 50), 60);
    }
}
