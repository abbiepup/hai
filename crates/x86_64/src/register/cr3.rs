use crate::register::{Read, CR4};
use bitflags::bitflags;
use core::arch::asm;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct CR3: u64 {
        const PWT = 1 << 3;
        const PCD = 1 << 4;
        const PCID = 1 << 11;
    }
}

impl CR3 {
    #[inline]
    pub fn read() -> Self {
        assert!(!CR4::read().contains(CR4::PCIDE));

        todo!()
    }

    #[inline]
    pub fn read_pci() -> Self {
        assert!(CR4::read().contains(CR4::PCIDE));

        todo!()
    }

    #[inline]
    pub fn read_pci_unchecked() -> Self {
        let _cr3 = Self::read_raw();

        todo!()
    }

    #[inline]
    pub fn read_raw() -> Self {
        let cr3: u64;
        unsafe { asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags)) };
        Self::from_bits_truncate(cr3)
    }

    /// # Safety
    /// todo!()
    #[inline]
    pub unsafe fn write_raw(cr3: Self) {
        unsafe { asm!("mov cr4, {}", in(reg) cr3.bits(), options(nomem, nostack, preserves_flags)) }
    }
}
