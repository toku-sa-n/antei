#![no_std]

mod global;

use r_efi::efi;

pub fn init(h: Handle, st: SystemTable) {
    global::init(h, st);
}

// To invalidate pointers of the EFI_HANDLE and the EFI_SYSTEM_TABLE, we define these structs
// without implementing `Copy`.

#[repr(transparent)]
pub struct Handle(efi::Handle);
// SAFETY: A UEFI application has only one thread.
unsafe impl Send for Handle {}

#[repr(transparent)]
pub struct SystemTable(efi::SystemTable);
// SAFETY: A UEFI application has only one thread.
unsafe impl Send for SystemTable {}
