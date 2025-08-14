use axplat::{
    init::InitIf,
    mem::{pa, phys_to_virt},
};

use crate::config::devices::{UART_INTERRUPT, UART_PADDR};

struct InitIfImpl;

#[impl_plat_interface]
impl InitIf for InitIfImpl {
    /// This function should be called immediately after the kernel has booted,
    /// and performed earliest platform configuration and initialization (e.g.,
    /// early console, clocking).
    fn init_early(_cpu_id: usize, _mbi: usize) {
        axcpu::init::init_trap();
        crate::console::init_early(phys_to_virt(pa!(UART_PADDR)));
        axplat::console::init_console_irq(UART_INTERRUPT);
        crate::time::init_early();
    }

    /// Initializes the platform at the early stage for secondary cores.
    #[cfg(feature = "smp")]
    fn init_early_secondary(_cpu_id: usize) {
        axcpu::init::init_trap();
    }

    /// Initializes the platform at the later stage for the primary core.
    ///
    /// This function should be called after the kernel has done part of its
    /// initialization (e.g, logging, memory management), and finalized the rest of
    /// platform configuration and initialization.
    fn init_later(_cpu_id: usize, _arg: usize) {
        crate::time::init_percpu();
    }

    /// Initializes the platform at the later stage for secondary cores.
    #[cfg(feature = "smp")]
    fn init_later_secondary(_cpu_id: usize) {
        crate::time::init_percpu();
    }
}
