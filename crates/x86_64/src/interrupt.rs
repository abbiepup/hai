use crate::address::Virtual;
use core::arch::asm;
use core::marker::PhantomData;

/// An interrupt handler.
pub type Interrupt = extern "x86-interrupt" fn(_: u8);

/// The interrupt descriptor table.
#[repr(C)]
#[derive(Debug)]
pub struct Table([Descriptor<Interrupt>; 255]);

impl Table {
    #[inline]
    fn ptr(&self) -> Pointer {
        Pointer {
            limit: (size_of::<Table>() - 1) as u16,
            base: Virtual::new(self as *const _ as u64),
        }
    }
}

/// Pointer to the interrupt descriptor table.
#[repr(C, packed(2))]
#[derive(Debug)]
pub struct Pointer {
    limit: u16,
    base: Virtual,
}

/// An interrupt descriptor.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Descriptor<F> {
    lower: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    middle: u16,
    high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

impl<F> Descriptor<F> {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            lower: 0,
            selector: 0,
            ist: 0,
            flags: 0,
            middle: 0,
            high: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }
}

impl Descriptor<Interrupt> {}

/// Enable interrupts.
#[inline]
pub fn enable() {
    unsafe { asm!("sti", options(nomem, nostack, preserves_flags)) };
}

/// Disable interrupts.
#[inline]
pub fn disable() {
    unsafe { asm!("cli", options(nomem, nostack, preserves_flags)) };
}

/// Load an interrupt descriptor table into the cpu.
#[inline]
pub fn load(table: Table) {
    unsafe { asm!("lidt [{}]", in(reg) &table.ptr(), options(readonly, nostack, preserves_flags)) };
}
