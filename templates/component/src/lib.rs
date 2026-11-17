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

    /// Initialise the component. Returns `Ok(())` once ready.
    pub fn init(&mut self) -> ComponentResult<()> {
        self.initialised = true;
        Ok(())
    }

    /// Returns `true` once [`Component::init`] has succeeded.
    pub fn is_ready(&self) -> bool {
        self.initialised
    }

    /// Placeholder for the component's primary operation.
    ///
    /// Replace with the real implementation for this component; keep the
    /// `ComponentResult` contract and update `COMPONENT.toml` accordingly.
    pub fn run(&self) -> ComponentResult<()> {
        if !self.initialised {
            return Err(ComponentError::Unsupported("not initialised"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
}
