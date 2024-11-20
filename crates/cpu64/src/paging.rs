#[repr(C, align(4096))]
#[derive(Debug)]
pub struct PageTable {
    entries: [u64; 512],
}

/// Enable paging.
#[inline]
pub fn enable() {
    #[cfg(target_arch = "x86_64")]
    x86_64::paging::enable();
}

/// Disable paging.
#[inline]
pub fn disable() {
    #[cfg(target_arch = "x86_64")]
    x86_64::paging::disable();
}
