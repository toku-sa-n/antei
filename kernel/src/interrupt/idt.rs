use core::convert::TryInto;

use {
    conquer_once::spin::Lazy,
    x86_64::{structures::idt::InterruptDescriptorTable, VirtAddr},
};

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    extern "sysv64" {
        fn asm_interrupt_handler_0x20();
    }

    let mut idt = InterruptDescriptorTable::new();

    // SAFETY: The address is correct.
    unsafe {
        idt[0x20].set_handler_addr(VirtAddr::new(
            (asm_interrupt_handler_0x20 as usize).try_into().unwrap(),
        ));
    }

    idt
});

pub(crate) fn init() {
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
