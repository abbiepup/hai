use super::Read;
use crate::address::Virtual;
use core::arch::asm;

#[derive(Debug)]
pub struct CR2;

impl Read for CR2 {
    type Output = Virtual;

    #[inline]
    fn read() -> Self::Output {
        let address;
        unsafe { asm!("mov {}, cr2", out(reg) address, options(nomem, nostack, preserves_flags)) };
        Virtual::new(address)
    }
}
