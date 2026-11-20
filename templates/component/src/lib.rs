//! __COMPONENT_NAME__ — __COMPONENT_DESC__
//!
//! Part of the SigmaOS multi-repository component ecosystem. This crate
//! declares its interface in `COMPONENT.toml` (`provides` / `depends_on`).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Errors raised by this component.
#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    /// The requested operation is not supported by this component yet.
    #[error("unsupported operation: {0}")]
    Unsupported(&'static str),
    /// A component with this name is already registered.
    #[error("duplicate component: {0}")]
    Duplicate(String),
    /// A declared dependency is not present in the registry.
    #[error("missing dependency '{dep}' required by '{component}'")]
    MissingDependency {
        /// The component that declared the dependency.
        component: String,
        /// The dependency name that could not be resolved.
        dep: String,
    },
    /// The declared dependencies form a cycle and cannot be ordered.
    #[error("dependency cycle detected involving '{0}'")]
    DependencyCycle(String),
    /// The named component is not registered.
    #[error("unknown component: {0}")]
    UnknownComponent(String),
}

/// Result alias for component operations.
pub type ComponentResult<T> = Result<T, ComponentError>;

/// The lifecycle contract every SigmaOS component fulfils.
///
/// Depending on this trait (rather than a concrete type) is what lets the hub
/// and other components treat any component polymorphically — interface
/// segregation + dependency inversion in one place.
pub trait ComponentLifecycle {
    /// Initialise the component. Returns `Ok(())` once ready.
    fn init(&mut self) -> ComponentResult<()>;
    /// Run the component's primary operation.
    fn run(&self) -> ComponentResult<()>;
    /// Release resources. Must be idempotent and safe to call when not ready.
    fn shutdown(&mut self) -> ComponentResult<()>;
    /// Whether the component has completed [`ComponentLifecycle::init`].
    fn is_ready(&self) -> bool;
}

/// A handle to an initialised component instance.
#[derive(Debug, Default, Clone)]
pub struct Component {
    initialised: bool,
}

impl Component {
    /// Create a new, uninitialised component handle.
    pub fn new() -> Self {
        Self::default()
    }
}

impl ComponentLifecycle for Component {
    fn init(&mut self) -> ComponentResult<()> {
        self.initialised = true;
        Ok(())
    }

    fn run(&self) -> ComponentResult<()> {
        if !self.initialised {
            return Err(ComponentError::Unsupported("not initialised"));
        }
        Ok(())
    }

    fn shutdown(&mut self) -> ComponentResult<()> {
        self.initialised = false;
        Ok(())
    }

    fn is_ready(&self) -> bool {
        self.initialised
    }
}

/// Owns a set of [`ComponentLifecycle`] implementations and drives them in
/// dependency order.
///
/// The registry only ever sees trait objects (`Box<dyn ComponentLifecycle>`),
/// so any component — including components from other repositories — can be
/// composed without the registry knowing their concrete types (dependency
/// inversion). `init_all` topologically sorts by declared dependencies and
/// `shutdown_all` tears down in reverse order; both fail fast on missing
/// dependencies or cycles rather than booting a half-wired system.
#[derive(Default)]
pub struct ComponentRegistry {
    /// name -> component instance
    components: std::collections::BTreeMap<String, Box<dyn ComponentLifecycle>>,
    /// name -> names this component declared it depends on
    dependencies: std::collections::BTreeMap<String, Vec<String>>,
}

impl ComponentRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register `component` under `name`.
    ///
    /// `depends_on` lists the *names* of other components that must be
    /// initialised first. Dependencies may be registered after this call —
    /// ordering is resolved lazily by [`ComponentRegistry::init_all`].
    pub fn register(
        &mut self,
        name: impl Into<String>,
        depends_on: &[&str],
        component: impl ComponentLifecycle + 'static,
    ) -> ComponentResult<()> {
        let name = name.into();
        if self.components.contains_key(&name) {
            return Err(ComponentError::Duplicate(name));
        }
        self.dependencies.insert(
            name.clone(),
            depends_on.iter().map(|d| (*d).to_string()).collect(),
        );
        self.components.insert(name, Box::new(component));
        Ok(())
    }

    /// Borrow a registered component by name.
    pub fn get(&self, name: &str) -> Option<&dyn ComponentLifecycle> {
        self.components.get(name).map(|c| c.as_ref())
    }

    /// Number of registered components.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Compute dependency-safe init order via DFS with cycle detection.
    fn init_order(&self) -> ComponentResult<Vec<String>> {
        let mut order = Vec::with_capacity(self.components.len());
        let mut state: std::collections::BTreeMap<&str, u8> =
            std::collections::BTreeMap::new(); // 0=unvisited 1=in-stack 2=done

        fn visit<'a>(
            name: &'a str,
            deps: &'a std::collections::BTreeMap<String, Vec<String>>,
            state: &mut std::collections::BTreeMap<&'a str, u8>,
            order: &mut Vec<String>,
        ) -> ComponentResult<()> {
            match state.get(name).copied().unwrap_or(0) {
                2 => return Ok(()),
                1 => return Err(ComponentError::DependencyCycle(name.to_string())),
                _ => {}
            }
            state.insert(name, 1);
            if let Some(list) = deps.get(name) {
                for dep in list {
                    if !deps.contains_key(dep) {
                        return Err(ComponentError::MissingDependency {
                            component: name.to_string(),
                            dep: dep.clone(),
                        });
                    }
                    visit(dep, deps, state, order)?;
                }
            }
            state.insert(name, 2);
            order.push(name.to_string());
            Ok(())
        }

        for name in self.components.keys() {
            visit(name, &self.dependencies, &mut state, &mut order)?;
        }
        Ok(order)
    }

    /// Initialise every component in dependency order.
    ///
    /// Fails fast: if any component fails to init, already-initialised
    /// components are shut down (reverse order) before the error returns,
    /// so the registry never reports a half-ready fleet.
    pub fn init_all(&mut self) -> ComponentResult<()> {
        let order = self.init_order()?;
        let mut done = 0usize;
        for name in &order {
            if let Some(c) = self.components.get_mut(name) {
                if let Err(e) = c.init() {
                    // Roll back what we started, newest first.
                    for prev in order[..done].iter().rev() {
                        if let Some(p) = self.components.get_mut(prev) {
                            let _ = p.shutdown();
                        }
                    }
                    return Err(e);
                }
                done += 1;
            }
        }
        Ok(())
    }

    /// Shut down every component in reverse dependency order.
    ///
    /// Individual shutdown failures are collected and reported as
    /// [`ComponentError::Unsupported`]; shutdown is best-effort because a
    /// component may already be partially torn down.
    pub fn shutdown_all(&mut self) -> ComponentResult<()> {
        let order = self.init_order()?;
        let mut first_err = None;
        for name in order.iter().rev() {
            if let Some(c) = self.components.get_mut(name) {
                if let Err(e) = c.shutdown() {
                    if first_err.is_none() {
                        first_err = Some(e);
                    }
                }
            }
        }
        match first_err {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Arrange-Act-Assert
    #[test]
    fn init_marks_component_ready() {
        // Arrange
        let mut c = Component::new();
        assert!(!c.is_ready());

        // Act
        c.init().unwrap();

        // Assert
        assert!(c.is_ready());
    }

    #[test]
    fn run_requires_init() {
        // Arrange
        let c = Component::new();

        // Act
        let res = c.run();

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn shutdown_resets_readiness() {
        // Arrange
        let mut c = Component::new();
        c.init().unwrap();
        assert!(c.is_ready());

        // Act
        c.shutdown().unwrap();

        // Assert
        assert!(!c.is_ready());
        assert!(c.run().is_err());
    }

    // Polymorphism: any ComponentLifecycle can be driven through the trait.
    #[test]
    fn lifecycle_works_through_trait_object() {
        // Arrange
        let mut c: Box<dyn ComponentLifecycle> = Box::new(Component::new());

        // Act + Assert
        assert!(!c.is_ready());
        c.init().unwrap();
        assert!(c.is_ready());
        c.run().unwrap();
        c.shutdown().unwrap();
        assert!(!c.is_ready());
    }

    // ── ComponentRegistry ────────────────────────────────────────────────────

    use std::cell::RefCell;
    use std::rc::Rc;

    /// Test double that records lifecycle calls into a shared log.
    struct RecordingComponent {
        log: Rc<RefCell<Vec<String>>>,
        tag: &'static str,
        fail_init: bool,
    }

    impl RecordingComponent {
        fn new(tag: &'static str, log: &Rc<RefCell<Vec<String>>>) -> Self {
            Self { log: log.clone(), tag, fail_init: false }
        }
        fn failing(tag: &'static str, log: &Rc<RefCell<Vec<String>>>) -> Self {
            Self { log: log.clone(), tag, fail_init: true }
        }
    }

    impl ComponentLifecycle for RecordingComponent {
        fn init(&mut self) -> ComponentResult<()> {
            self.log.borrow_mut().push(format!("init:{}", self.tag));
            if self.fail_init {
                return Err(ComponentError::Unsupported("forced init failure"));
            }
            Ok(())
        }
        fn run(&self) -> ComponentResult<()> {
            Ok(())
        }
        fn shutdown(&mut self) -> ComponentResult<()> {
            self.log.borrow_mut().push(format!("shutdown:{}", self.tag));
            Ok(())
        }
        fn is_ready(&self) -> bool {
            true
        }
    }

    #[test]
    fn registry_inits_in_dependency_order() {
        // Arrange — consumer registered BEFORE its dependency to prove
        // ordering comes from declarations, not registration sequence.
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut reg = ComponentRegistry::new();
        reg.register("consumer", &["provider"], RecordingComponent::new("consumer", &log)).unwrap();
        reg.register("provider", &[], RecordingComponent::new("provider", &log)).unwrap();

        // Act
        reg.init_all().unwrap();

        // Assert
        assert_eq!(
            *log.borrow(),
            vec!["init:provider".to_string(), "init:consumer".to_string()]
        );
    }

    #[test]
    fn registry_shutdown_runs_in_reverse_order() {
        // Arrange
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut reg = ComponentRegistry::new();
        reg.register("a", &[], RecordingComponent::new("a", &log)).unwrap();
        reg.register("b", &["a"], RecordingComponent::new("b", &log)).unwrap();
        reg.init_all().unwrap();
        log.borrow_mut().clear();

        // Act
        reg.shutdown_all().unwrap();

        // Assert — b depends on a, so b shuts down first
        assert_eq!(
            *log.borrow(),
            vec!["shutdown:b".to_string(), "shutdown:a".to_string()]
        );
    }

    #[test]
    fn registry_rejects_missing_dependency() {
        // Arrange
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut reg = ComponentRegistry::new();
        reg.register("app", &["ghost"], RecordingComponent::new("app", &log)).unwrap();

        // Act
        let res = reg.init_all();

        // Assert
        assert!(matches!(res, Err(ComponentError::MissingDependency { .. })));
    }

    #[test]
    fn registry_rejects_dependency_cycle() {
        // Arrange
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut reg = ComponentRegistry::new();
        reg.register("x", &["y"], RecordingComponent::new("x", &log)).unwrap();
        reg.register("y", &["x"], RecordingComponent::new("y", &log)).unwrap();

        // Act
        let res = reg.init_all();

        // Assert
        assert!(matches!(res, Err(ComponentError::DependencyCycle(_))));
    }

    #[test]
    fn registry_rejects_duplicate_names() {
        // Arrange
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut reg = ComponentRegistry::new();
        reg.register("dup", &[], RecordingComponent::new("dup", &log)).unwrap();

        // Act
        let res = reg.register("dup", &[], RecordingComponent::new("dup2", &log));

        // Assert
        assert!(matches!(res, Err(ComponentError::Duplicate(_))));
    }

    #[test]
    fn registry_rolls_back_partial_init_on_failure() {
        // Arrange — provider inits fine, consumer fails: provider must be
        // shut down so the fleet is not left half-ready.
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut reg = ComponentRegistry::new();
        reg.register("provider", &[], RecordingComponent::new("provider", &log)).unwrap();
        reg.register("consumer", &["provider"], RecordingComponent::failing("consumer", &log)).unwrap();

        // Act
        let res = reg.init_all();

        // Assert
        assert!(res.is_err());
        assert_eq!(
            *log.borrow(),
            vec![
                "init:provider".to_string(),
                "init:consumer".to_string(),
                "shutdown:provider".to_string(),
            ]
        );
    }
}
