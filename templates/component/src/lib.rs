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
}
