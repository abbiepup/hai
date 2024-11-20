use super::{Read, Write};
use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CR0: u64 {
        /// **P**rotected Mode **E**nable
        /// - If 1, system is in protected mode, else, system is in real mode.
        const PE = 1 << 0;

        /// **M**onitor co-**p**rocessor
        /// - Controls interaction of `WAIT`/`FWAIT` instructions with TS flag in `CR0`.
        const MP = 1 << 1;

        /// **Em**ulation
        /// If set, no x87 floating-point unit present, if clear, x87 FPU present.
        const EM = 1 << 2;

        /// **T**ask **s**witched
        /// - Allows saving x87 task context upon a task switch only after x87 instruction used.
        const TS = 1 << 3;

        /// **E**xtension **t**ype
        /// - On the 386, it allowed to specify whether the external math coprocessor was an 80287 or 80387.
        const ET = 1 << 4;

        /// **N**umeric **e**rror
        /// - Enable internal x87 floating point error reporting when set,
        /// else enables PC style x87 error detection.
        const NE = 1 << 5;

        /// **W**rite **p**rotect
        /// - When set, the CPU cannot write to read-only pages when privilege level is 0.
        const WP = 1 << 16;

        /// **A**lignment **m**ask
        /// - Alignment check enabled if AM set, AC flag (in EFLAGS register) set, and privilege level is 3.
        const AM = 1 << 18;

        /// Not-write through
        /// - Globally enables/disable write-through caching.
        const NM = 1 << 29;

        /// **C**ache **d**isable
        /// - Globally enables/disable the memory cache.
        const CD = 1 << 30;

        /// Paging
        /// - If 1, enable paging and use the § CR3 register, else disable paging.
        const PG = 1 << 31;
    }
}

impl Read for CR0 {
    type Output = Self;

    #[inline]
    fn read() -> Self::Output {
        let flags;
        unsafe { asm!("mov {}, cr0", out(reg) flags, options(nomem, nostack, preserves_flags)) };
        Self::from_bits_retain(flags)
    }
}

impl Write for CR0 {
    #[inline]
    unsafe fn write(flags: Self) {
        unsafe {
            asm!("mov cr0, {}", in(reg) flags.bits(), options(nomem, nostack, preserves_flags))
        }
    }
}
