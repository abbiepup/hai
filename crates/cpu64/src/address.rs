#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VirtualAddress(usize);

impl From<usize> for VirtualAddress {
    #[inline]
    fn from(address: usize) -> Self {
        Self(address)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PhysicalAddress(usize);

impl From<usize> for PhysicalAddress {
    #[inline]
    fn from(address: usize) -> Self {
        Self(address)
    }
}
