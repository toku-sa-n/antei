use conquer_once::spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(InterruptDescriptorTable::new);

pub fn init() {
    IDT.load();
}
