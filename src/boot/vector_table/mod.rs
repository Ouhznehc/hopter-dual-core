//! The vector table code is derived from the `cortex-m-rt` crate.

pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[cfg(feature = "stm32h747")]
mod vector_dual_core;

#[cfg(not(feature = "stm32h747"))]
mod vector;

mod stm32f401;
mod stm32f405;
mod stm32f407;
mod stm32f410;
mod stm32f411;
mod stm32f412;
mod stm32f413;
mod stm32f427;
mod stm32f429;
mod stm32f446;
mod stm32f469;
mod stm32h747cm4;
mod stm32h747cm7;
