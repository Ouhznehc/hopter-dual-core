use crate::boot::reset;
use crate::interrupt::hardfault;
use super::Vector;

extern "C" {
    fn NonMaskableInt();

    #[cfg(not(armv6m))]
    fn MemoryManagement();

    #[cfg(not(armv6m))]
    fn BusFault();

    #[cfg(not(armv6m))]
    fn UsageFault();

    #[cfg(armv8m)]
    fn SecureFault();

    fn SVCall();

    #[cfg(not(armv6m))]
    fn DebugMonitor();

    fn PendSV();

    fn SysTick();
}


#[link_section = ".hopter_vector_table.reset_vector"]
#[no_mangle]
static __HOPTER_RESET_VECTOR: unsafe extern "C" fn() -> ! = reset::entry;

#[link_section = ".hopter_vector_table.exceptions"]
#[no_mangle]
pub static __HOPTER_EXCEPTIONS: [Vector; 14] = [
    // Exception 2: Non Maskable Interrupt.
    Vector {
        handler: NonMaskableInt,
    },
    // Exception 3: Hard Fault Interrupt.
    Vector {
        handler: hardfault::hardfault_trampoline,
    },
    // Exception 4: Memory Management Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: MemoryManagement,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 5: Bus Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector { handler: BusFault },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 6: Usage Fault Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: UsageFault,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // Exception 7: Secure Fault Interrupt [only on Armv8-M].
    #[cfg(armv8m)]
    Vector {
        handler: SecureFault,
    },
    #[cfg(not(armv8m))]
    Vector { reserved: 0 },
    // 8-10: Reserved
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    // Exception 11: SV Call Interrupt.
    Vector { handler: SVCall },
    // Exception 12: Debug Monitor Interrupt [not on Cortex-M0 variants].
    #[cfg(not(armv6m))]
    Vector {
        handler: DebugMonitor,
    },
    #[cfg(armv6m)]
    Vector { reserved: 0 },
    // 13: Reserved
    Vector { reserved: 0 },
    // Exception 14: Pend SV Interrupt [not on Cortex-M0 variants].
    Vector { handler: PendSV },
    // Exception 15: System Tick Interrupt.
    Vector { handler: SysTick },
];