use crate::register::{CR0, Write};

/// Enables paging.
#[inline]
pub fn enable() {
    unsafe { CR0::update(|cr0| cr0 | CR0::PG) };
}

/// Disables paging.
#[inline]
pub fn disable() {
    unsafe { CR0::update(|cr0| cr0 & !CR0::PG) };
}
