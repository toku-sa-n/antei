use {core::mem::size_of, static_assertions::const_assert_eq};

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct Context {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,

    rsp: u64,
    rbp: u64,
    rsi: u64,
    rdi: u64,

    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,

    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,

    cs: u64,
    ss: u64,
    fs: u64,
    gs: u64,

    cr3: u64,
    rip: u64,
    rflags: u64,
    _fxsave_requires_16_bytes_alinged_address: u64,

    fxsave_area: [u8; 512],
}
const_assert_eq!(size_of::<Context>(), 8 * 4 * 6 + 512);
impl Context {
    pub(super) fn switch(old: *mut Context, new: *mut Context) {
        extern "sysv64" {
            fn asm_switch_context(old: *mut Context, new: *mut Context);
        }

        unsafe {
            asm_switch_context(old, new);
        }
    }
}
impl Default for Context {
    fn default() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsp: 0,
            rbp: 0,
            rsi: 0,
            rdi: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            cs: 0,
            ss: 0,
            fs: 0,
            gs: 0,
            cr3: 0,
            rip: 0,
            rflags: 0,
            _fxsave_requires_16_bytes_alinged_address: 0,
            fxsave_area: [0; 512],
        }
    }
}
