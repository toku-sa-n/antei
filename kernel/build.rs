use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        test_on_qemu: {
            feature = "test_on_qemu"
        },
    }
}
