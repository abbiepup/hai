use core::arch::asm;

/// Enable interrupts.
#[inline]
pub fn enable() {
    unsafe { asm!("msr daifclr, #2", options(nomem, nostack, preserves_flags)) };
}

/// Disable interrupts.
#[inline]
pub fn disable() {
    unsafe { asm!("msr daifset, #2", options(nomem, nostack, preserves_flags)) };
}
