mod cr0;
mod cr2;
mod cr3;
mod cr4;
mod cr8;
mod xcr0;

pub use cr0::CR0;
pub use cr2::CR2;
pub use cr3::CR3;
pub use cr4::CR4;
pub use cr8::CR8;
pub use xcr0::XCR0;

/// Trait for reading a register.
pub trait Read: Sized {
    type Output;

    fn read() -> Self::Output;
}

/// Trait for writing to a register.
pub trait Write: Read<Output = Self> {
    unsafe fn write(reg: Self);

    #[inline]
    unsafe fn update(f: impl FnOnce(Self) -> Self::Output) {
        unsafe { Self::write(f(Self::read().into())) };
    }
}
