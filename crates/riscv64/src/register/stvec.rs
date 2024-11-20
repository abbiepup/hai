use core::arch::asm;

#[derive(Debug)]
pub struct STVEC {}

#[derive(Debug)]
pub enum Mode {
    Direct = 0,
    Vectored = 1,
}

pub fn read() -> (usize, Mode) {
    let stvec: usize;

    unsafe { asm!("csrr {}, stvec", out(reg) stvec) };

    let base = stvec & !0b11;

    let mode = match stvec & 0b11 {
        0 => Mode::Direct,
        1 => Mode::Vectored,
        _ => unreachable!(),
    };

    (base, mode)
}

/// # Safety
pub unsafe fn write(base: usize, mode: Mode) {
    let mode = match mode {
        Mode::Direct => 0,
        Mode::Vectored => 1,
    };

    let stvec = (base & !0b11) | mode;

    unsafe { asm!("csrw stvec, {}", in(reg) stvec) };
}
