use super::Read;
use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct XCR0: u64 {
        const X87 = 1 << 0;
        const SSE = 1 << 1;
        const AVX = 1 << 2;
        const BNDREG = 1 << 3;
        const BNDCSR = 1 << 4;
        const OPMASK = 1 << 5;
        const ZMM_HI256 = 1 << 6;
        const HI16_ZMM = 1 << 7;
        const PT = 1 << 8;
        const PKRU = 1 << 9;
        const PASID = 1 << 10;
        const CET_U = 1 << 11;
        const CET_S = 1 << 12;
        const HDC = 1 << 13;
        const UINTR = 1 << 14;
        const LBR = 1 << 15;
        const HWP = 1 << 16;
        const XTILECFG = 1 << 17;
        const XTILEDATA = 1 << 18;
        const APX = 1 << 19;
    }
}

impl Read for XCR0 {
    type Output = Self;

    #[inline]
    fn read() -> Self::Output {
        let xcr0: u64;
        unsafe { asm!("mov {}, xcr0", out(reg) xcr0, options(nomem, nostack, preserves_flags)) };
        Self::from_bits_truncate(xcr0)
    }
}
