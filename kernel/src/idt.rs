use conquer_once::spin::Lazy;
use x86_64::structures::idt::InterruptDescriptorTable;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(InterruptDescriptorTable::new);

pub(super) fn init() {
    IDT.load();

    #[cfg(test_on_qemu)]
    tests::main();
}

#[cfg(test_on_qemu)]
mod tests {
    use {
        super::IDT,
        x86_64::{instructions::tables, structures::idt::InterruptDescriptorTable, VirtAddr},
    };

    pub(super) fn main() {
        assert_idt_address_is_correct();
    }

    fn assert_idt_address_is_correct() {
        let expected_addr = VirtAddr::from_ptr(idt());

        let descriptor_table_ptr = tables::sidt();
        let actual_base = descriptor_table_ptr.base;

        assert_eq!(
            expected_addr, actual_base,
            "The address of the current IDT is not correct."
        );
    }

    fn idt<'a>() -> &'a InterruptDescriptorTable {
        &*IDT
    }
}
