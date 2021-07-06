use conquer_once::spin::OnceCell;
use x86_64::instructions::segmentation;
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
        segmentation::set_cs(code);

        segmentation::load_ds(data);
        segmentation::load_es(data);
        segmentation::load_fs(data);
        segmentation::load_gs(data);
        segmentation::load_ss(data);
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
