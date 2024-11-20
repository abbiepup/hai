#![no_std]
#![no_main]

use kernel::boot::{BootInfo, Status};

#[cfg(target_os = "uefi")]
#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(
    image_handle: uefi::Handle,
    system_table: *const core::ffi::c_void,
) -> uefi::Status {
    use arrayvec::ArrayString;
    use uefi::boot::{MemoryType, exit_boot_services};
    use uefi::mem::memory_map::MemoryMap;
    use uefi::system::{firmware_revision, firmware_vendor, uefi_revision};

    unsafe { uefi::table::set_system_table(system_table.cast()) };
    unsafe { uefi::boot::set_image_handle(image_handle) };

    let (firmware_vendor, firmware_revision, uefi_revision) = {
        let mut buf = ArrayString::<128>::new();
        firmware_vendor().as_str_in_buf(&mut buf).unwrap();
        (buf.as_ptr(), firmware_revision(), uefi_revision().0)
    };

    let mmap = unsafe { exit_boot_services(MemoryType::LOADER_DATA) };

    mmap.entries().for_each(|descriptor| {
        if descriptor.ty == MemoryType::CONVENTIONAL {
            let size = descriptor.page_count * 0x1000;
            let _end = descriptor.phys_start + size;
        }
    });

    kernel_main(BootInfo {
        firmware_vendor,
        firmware_revision,
        uefi_revision,
    })
    .into()
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_boot_info: BootInfo) -> Status {
    cpu64::interrupt::enable();
    cpu64::paging::enable();

    loop {
        core::hint::spin_loop();
    }
}
