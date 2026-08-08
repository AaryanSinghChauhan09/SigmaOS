// Hardware Abstraction Layer module - exports Phase G HAL components
pub mod interrupt_controller;
pub mod deterministic_interrupt;

pub use interrupt_controller::*;
pub use deterministic_interrupt::*;
