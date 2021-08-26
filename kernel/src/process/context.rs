#[repr(C)]
#[derive(Clone, Debug, Default)]
pub(super) struct Context {
    pub(super) rax: u64,
    pub(super) rbx: u64,
    pub(super) rcx: u64,
    pub(super) rdx: u64,
    pub(super) rsp: u64,
    pub(super) rbp: u64,
    pub(super) rsi: u64,
    pub(super) rdi: u64,

    pub(super) r8: u64,
    pub(super) r9: u64,
    pub(super) r10: u64,
    pub(super) r11: u64,
    pub(super) r12: u64,
    pub(super) r13: u64,
    pub(super) r14: u64,
    pub(super) r15: u64,

    pub(super) cr3: u64,
    pub(super) cs: u64,
    pub(super) ss: u64,
    pub(super) fs: u64,
    pub(super) gs: u64,
    pub(super) rip: u64,
    pub(super) rflags: u64,
    _fxsave_must_be_16_bytes_aligned: u64,

    pub(super) fxsave_area: [u128; 4],
}
