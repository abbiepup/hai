use core::arch::asm;

/// Enable interrupts.
#[inline]
pub fn enable() {
    unsafe {
        asm!(
            "csrsi sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        )
    };
}

/// Disable interrupts.
#[inline]
pub fn disable() {
    unsafe {
        asm!(
            "csrci sstatus, 0x2",
            options(nomem, nostack, preserves_flags)
        )
    };
}
