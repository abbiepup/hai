#[derive(Debug)]
pub enum Address {
    Virtual(Virtual),
    Physical(Physical),
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Virtual(u64);

impl Virtual {
    #[inline]
    pub const fn new(address: u64) -> Self {
        Self(address)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Physical(u64);

impl Physical {
    #[inline]
    pub const fn new(address: u64) -> Self {
        Self(address)
    }
}
