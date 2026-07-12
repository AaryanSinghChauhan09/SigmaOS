// Hardware Abstraction Layer module - exports Phase G HAL components
#[path = "hal/interrupt_controller.rs"]
pub mod interrupt_controller;

pub use interrupt_controller::*;
