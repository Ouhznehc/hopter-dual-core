#[cfg(feature = "stm32h747")]
pub(crate) mod reset_dual_core;

#[cfg(not(feature = "stm32h747"))]
pub(crate) mod reset;

mod system_init;
mod vector_table;
