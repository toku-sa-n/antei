use conquer_once::spin::OnceCell;
use x86_64::instructions::segmentation::Segment;
use x86_64::instructions::segmentation::CS;
use x86_64::instructions::segmentation::DS;
use x86_64::instructions::segmentation::ES;
use x86_64::instructions::segmentation::FS;
use x86_64::instructions::segmentation::GS;
use x86_64::instructions::segmentation::SS;
use x86_64::structures::gdt::Descriptor;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::SegmentSelector;

static GDT: OnceCell<GlobalDescriptorTable> = OnceCell::uninit();

static SELECTORS: OnceCell<Selectors> = OnceCell::uninit();

struct Selectors {
    kernel_code: SegmentSelector,
    kernel_data: SegmentSelector,
}
impl Selectors {
    fn new(kernel_code: SegmentSelector, kernel_data: SegmentSelector) -> Self {
        Self {
            kernel_code,
            kernel_data,
        }
    }
}

pub fn init() {
    init_gdt();
    lgdt();
    load_segments();

    #[cfg(feature = "test_on_qemu")]
    tests::main();
}

fn init_gdt() {
    let r = GDT.try_init_once(|| {
        let mut gdt = GlobalDescriptorTable::new();

        let kernel_code = gdt.add_entry(Descriptor::kernel_code_segment());
        let kernel_data = gdt.add_entry(Descriptor::kernel_data_segment());
        gdt.add_entry(Descriptor::user_data_segment());
        gdt.add_entry(Descriptor::user_code_segment());

        init_selectors(Selectors::new(kernel_code, kernel_data));

        gdt
    });
    r.expect("Failed to initialize GDT.");
}

fn init_selectors(selectors: Selectors) {
    let r = SELECTORS.try_init_once(|| selectors);
    r.expect("Failed to initialize `SELECTORS`.")
}

fn lgdt() {
    gdt().load();
}

fn load_segments() {
    let selectors = selectors();

    let code = selectors.kernel_code;
    let data = selectors.kernel_data;

    unsafe {
        CS::set_reg(code);

        DS::set_reg(data);
        ES::set_reg(data);
        FS::set_reg(data);
        GS::set_reg(data);
        SS::set_reg(data);
    }
}

fn gdt<'a>() -> &'a GlobalDescriptorTable {
    let gdt = GDT.try_get();
    gdt.expect("GDT is not initialized.")
}

fn selectors<'a>() -> &'a Selectors {
    let selectors = SELECTORS.try_get();
    selectors.expect("`SELECTORS` is not initialized.")
}

#[cfg(feature = "test_on_qemu")]
mod tests {
    use {
        super::selectors,
        x86_64::instructions::segmentation::{Segment, CS, DS, ES, FS, GS, SS},
    };

    pub(super) fn main() {
        assert();
    }

    fn assert() {
        let selectors = selectors();

        let code = selectors.kernel_code;
        let data = selectors.kernel_data;

        macro_rules! assert_segment {
            ($seg:ident,$correct:expr) => {
                assert_eq!(
                    $seg::get_reg(),
                    $correct,
                    "Incorrect {}",
                    core::stringify!($seg)
                );
            };
        }

        assert_segment!(CS, code);

        assert_segment!(DS, data);
        assert_segment!(ES, data);
        assert_segment!(FS, data);
        assert_segment!(GS, data);
        assert_segment!(SS, data);
    }
}
