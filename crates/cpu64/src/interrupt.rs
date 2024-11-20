/// Enable interrupts.
#[inline]
pub fn enable() {
    #[cfg(target_arch = "aarch64")]
    aarch64::interrupt::enable();

    #[cfg(target_arch = "riscv64")]
    riscv64::interrupt::enable();

    #[cfg(target_arch = "x86_64")]
    x86_64::interrupt::enable();
}

/// Disable interrupts.
#[inline]
pub fn disable() {
    #[cfg(target_arch = "aarch64")]
    aarch64::interrupt::disable();

    #[cfg(target_arch = "riscv64")]
    riscv64::interrupt::disable();

    #[cfg(target_arch = "x86_64")]
    x86_64::interrupt::disable();
}
