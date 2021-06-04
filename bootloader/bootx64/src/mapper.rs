use x86_64::registers::control::Cr0;
use x86_64::registers::control::Cr0Flags;

pub fn enable_write_protect() {
    unsafe { Cr0::update(|flags| flags.insert(Cr0Flags::WRITE_PROTECT)) }
}

pub fn disable_write_protect() {
    unsafe { Cr0::update(|flags| flags.remove(Cr0Flags::WRITE_PROTECT)) }
}
