use bitflags::bitflags;
use core::arch::asm;

use super::{Read, Write};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct CR4: u64 {
        /// **V**irtual-8086 **M**ode **E**xtensions
        /// - If set, enables support for the virtual interrupt flag (VIF) in virtual-8086 mode.
        const VME = 1 << 0;

        /// **P**rotected-mode **V**irtual **I**nterrupts
        /// - If set, enables support for the virtual interrupt flag (VIF) in protected mode.
        const PVI = 1 << 1;

        /// **T**ime **S**tamp **D**isable
        /// - If set, `RDTSC` instruction can only be executed when in ring 0,
        /// otherwise `RDTSC` can be used at any privilege level.
        const TSD = 1 << 2;

        /// **D**ebugging **E**xtensions
        /// - If set, enables debug register based breaks on I/O space access.
        const DE = 1 << 3;

        /// **Page** **Size** **Extension**
        /// - If set, enables 32-bit paging mode to use 4 MiB huge pages in addition to 4 KiB pages.
        /// - If PAE is enabled or the processor is in x86-64 long mode this bit is ignored.
        const PSE = 1 << 4;

        /// **P**hysical **A**ddress **E**xtension
        /// - If set, changes page table layout to translate 32-bit
        /// virtual addresses into extended 36-bit physical addresses.
        const PAE = 1 << 5;

        /// **M**achine **C**heck **E**xception
        /// - If set, enables machine check interrupts to occur.
        const MCE = 1 << 6;

        /// **P**age **G**lobal **E**nabled
        /// - If set, address translations (PDE or PTE records) may be shared between address spaces.
        const PGE = 1 << 7;

        /// **P**erformance-Monitoring **C**ounter **e**nable
        /// - If set, `RDPMC` can be executed at any privilege level, else RDPMC can only be used in ring 0.
        const PCE = 1 << 8;

        /// **O**perating **S**ystem support for **FXS**AVE and **FXR**STOR instructions
        /// - If set, enables Streaming SIMD Extensions (SSE) instructions and fast FPU save & restore.
        const OSFXSR = 1 << 9;

        /// **O**perating **System** Support for Unmasked SIMD Floating-Point Exceptions
        /// - If set, enables unmasked SSE exceptions.
        const OSXMMEXCPT = 1 << 10;

        /// **U**ser-**M**ode **I**nstruction **P**revention
        /// - If set, the `SGDT`, `SIDT`, `SLDT`, `SMSW` and `STR` instructions cannot be executed if CPL > 0.
        const UMIP = 1 << 11;

        /// **V**irtual **M**achine **E**xtensions **E**nable
        const VMXE = 1 << 13;

        /// **S**afer **M**ode E**x**tensions **E**nable
        const SMXE = 1 << 14;

        /// FSGSBASE Enable
        /// - If set, enables the instructions `RDFSBASE`, `RDGSBASE`, `WRFSBASE`, and `WRGSBASE`.
        const FSGSBASE = 1 << 16;

        /// PCID Enable
        /// - If set, enables process-context identifiers (PCIDs).
        const PCIDE = 1 << 17;

        /// XSAVE and Processor Extended States Enable
        const OSXSAVE = 1 << 18;

        /// **S**upervisor **M**ode **E**xecution **P**rotection Enable
        /// - If set, execution of code in a higher ring generates a fault.
        const SMEP = 1 << 20;

        /// **S**upervisor **M**ode **A**ccess **P**revention Enable
        /// - If set, access of data in a higher ring generates a fault.
        const SMAP = 1 << 21;

        /// **P**rotection **K**ey **E**nable
        const PKE = 1 << 22;

        /// **C**ontrol-flow **E**nforcement **T**echnology
        /// - If set, enables control-flow enforcement technology.
        const CET = 1 << 23;

        /// Enable **P**rotection **K**eys for **S**upervisor-Mode Pages
        /// - If set, each supervisor-mode linear address is associated
        /// with a protection key when 4-level or 5-level paging is in use
        const PKS = 1 << 24;
    }
}

impl Read for CR4 {
    type Output = Self;

    #[inline]
    fn read() -> Self::Output {
        let cr4: u64;
        unsafe { asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack, preserves_flags)) };
        
        Self::from_bits_truncate(cr4)
    }
}

impl Write for CR4 {
    #[inline]
    unsafe fn write(cr4: Self) {
        unsafe { asm!("mov cr4, {}", in(reg) cr4.bits(), options(nomem, nostack, preserves_flags)) }
    }
}
