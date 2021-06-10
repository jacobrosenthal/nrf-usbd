//! USB peripheral driver for nRF microcontrollers.

#![no_std]

mod errata;
mod pac;
mod usbd;

pub use usbd::Usbd;

/// A trait for device-specific USB peripherals. Implement this to add support for a new hardware
/// platform. Peripherals that have this trait must have the same register block as NRF52 USBD
/// peripherals.
pub unsafe trait UsbPeripheral: Send + Sync {
    /// Pointer to the register block
    const REGISTERS: *const ();
}
