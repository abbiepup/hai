#[repr(C)]
pub struct BootInfo {
    pub firmware_vendor: *const u8,
    pub firmware_revision: u32,
    pub uefi_revision: u32,
}

#[repr(C)]
pub enum Status {
    Success,
}

#[cfg(target_os = "uefi")]
impl From<Status> for uefi::Status {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => Self::SUCCESS,
        }
    }
}
