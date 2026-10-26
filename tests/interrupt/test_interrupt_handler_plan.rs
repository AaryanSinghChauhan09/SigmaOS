// Comprehensive Plan Verification & Architecture Tests for SigmaOS Interrupt Handling Subsystem

use std::fs;

#[test]
fn test_interrupt_handler_plan_documentation_exists_and_valid() {
    let docs_plan = fs::read_to_string("docs/INTERRUPT_HANDLER_PLAN.md")
        .expect("docs/INTERRUPT_HANDLER_PLAN.md must exist");
    let wiki_plan = fs::read_to_string("wiki/INTERRUPT_HANDLER_PLAN.md")
        .expect("wiki/INTERRUPT_HANDLER_PLAN.md must exist");

    assert_eq!(docs_plan, wiki_plan, "docs/ and wiki/ plans must be identical");

    // Verify key architectural sections and design rules
    assert!(docs_plan.contains("SigmaOS Interrupt Handling Master Architecture & Support Matrix Plan"));
    assert!(docs_plan.contains("Core Decoupled Pipeline (5 Stages)"));
    assert!(docs_plan.contains("Stage 1: Vector Entry"));
    assert!(docs_plan.contains("Stage 2: Generic Dispatch"));
    assert!(docs_plan.contains("Stage 3: Controller Flow"));
    assert!(docs_plan.contains("Stage 4: Handler Invocation"));
    assert!(docs_plan.contains("Stage 5: Deferred Work & Return"));

    assert!(docs_plan.contains("Architecture Audit & Support Matrix"));
    assert!(docs_plan.contains("x86_64"));
    assert!(docs_plan.contains("AArch64"));
    assert!(docs_plan.contains("RISC-V 64"));

    assert!(docs_plan.contains("Context Contracts & Execution Restrictions"));
    assert!(docs_plan.contains("Hard-IRQ Context"));
    assert!(docs_plan.contains("Threaded / Deferred Context"));
    assert!(docs_plan.contains("Exception Context"));

    assert!(docs_plan.contains("17-PR Sequence Roadmap"));
    assert!(docs_plan.contains("Periodic Maintenance PR Cadence"));
    assert!(docs_plan.contains("Component-Specific Rules"));
    assert!(docs_plan.contains("PR Template and Merge Gates"));
}

#[test]
fn test_index_files_reference_interrupt_handler_plan() {
    let docs_index = fs::read_to_string("docs/INDEX.md").expect("docs/INDEX.md must exist");
    let wiki_index = fs::read_to_string("wiki/INDEX.md").expect("wiki/INDEX.md must exist");
    let wiki_home = fs::read_to_string("wiki/Home.md").expect("wiki/Home.md must exist");

    assert!(docs_index.contains("INTERRUPT_HANDLER_PLAN.md"));
    assert!(wiki_index.contains("INTERRUPT_HANDLER_PLAN.md"));
    assert!(wiki_home.contains("INTERRUPT_HANDLER_PLAN.md"));
}

#[test]
fn test_interrupt_context_contracts_and_flow() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum IrqContext {
        HardIrq,
        Threaded,
        Exception,
        Nmi,
        Process,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum FlowType {
        Edge,
        Level,
        FastEoi,
        PerCpu,
    }

    struct IrqFlowSimulator {
        flow: FlowType,
        masked: bool,
        acked: bool,
        eoi_sent: bool,
        handled: bool,
    }

    impl IrqFlowSimulator {
        fn new(flow: FlowType) -> Self {
            Self {
                flow,
                masked: false,
                acked: false,
                eoi_sent: false,
                handled: false,
            }
        }

        fn process_irq(&mut self) -> Result<(), &'static str> {
            match self.flow {
                FlowType::Edge => {
                    self.acked = true;
                    self.handled = true;
                    self.eoi_sent = true;
                }
                FlowType::Level => {
                    self.masked = true;
                    self.acked = true;
                    self.handled = true;
                    self.eoi_sent = true;
                    self.masked = false;
                }
                FlowType::FastEoi => {
                    self.handled = true;
                    self.eoi_sent = true;
                }
                FlowType::PerCpu => {
                    self.handled = true;
                    self.eoi_sent = true;
                }
            }
            Ok(())
        }
    }

    let mut edge_sim = IrqFlowSimulator::new(FlowType::Edge);
    assert!(edge_sim.process_irq().is_ok());
    assert!(edge_sim.acked && edge_sim.eoi_sent && edge_sim.handled);

    let mut level_sim = IrqFlowSimulator::new(FlowType::Level);
    assert!(level_sim.process_irq().is_ok());
    assert!(!level_sim.masked && level_sim.acked && level_sim.eoi_sent);

    let mut fasteoi_sim = IrqFlowSimulator::new(FlowType::FastEoi);
    assert!(fasteoi_sim.process_irq().is_ok());
    assert!(fasteoi_sim.handled && fasteoi_sim.eoi_sent);

    let mut percpu_sim = IrqFlowSimulator::new(FlowType::PerCpu);
    assert!(percpu_sim.process_irq().is_ok());
    assert!(percpu_sim.handled && percpu_sim.eoi_sent);
}

#[test]
fn test_irq_domain_hardware_to_logical_mapping() {
    use std::collections::HashMap;

    struct IrqDomain {
        #[allow(dead_code)]
        domain_name: String,
        mappings: HashMap<u32, u32>, // hwirq -> logical irq
        next_logical: u32,
    }

    impl IrqDomain {
        fn new(name: &str) -> Self {
            Self {
                domain_name: name.to_string(),
                mappings: HashMap::new(),
                next_logical: 32,
            }
        }

        fn map_hwirq(&mut self, hwirq: u32) -> Result<u32, &'static str> {
            if self.mappings.contains_key(&hwirq) {
                return Err("Hardware IRQ already mapped");
            }
            let virq = self.next_logical;
            self.next_logical += 1;
            self.mappings.insert(hwirq, virq);
            Ok(virq)
        }

        fn find_virq(&self, hwirq: u32) -> Option<u32> {
            self.mappings.get(&hwirq).copied()
        }
    }

    let mut ioapic_domain = IrqDomain::new("IOAPIC-0");
    let virq_1 = ioapic_domain.map_hwirq(1).expect("Mapping hwirq 1 succeeds");
    let virq_2 = ioapic_domain.map_hwirq(2).expect("Mapping hwirq 2 succeeds");

    assert_eq!(ioapic_domain.find_virq(1), Some(virq_1));
    assert_eq!(ioapic_domain.find_virq(2), Some(virq_2));
    assert!(ioapic_domain.map_hwirq(1).is_err());
}
