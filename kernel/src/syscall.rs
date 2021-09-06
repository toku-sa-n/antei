use {
    crate::process::{
        ipc::{self, ReceiveFrom},
        Pid,
    },
    aligned_ptr::ptr,
    core::convert::TryInto,
    num_derive::FromPrimitive,
    num_traits::FromPrimitive,
    x86_64::{
        registers::{
            control::{Efer, EferFlags},
            model_specific::LStar,
        },
        VirtAddr,
    },
};

pub(super) fn init() {
    enable_syscall_and_sysret();
    register_syscall_handler();
}

fn enable_syscall_and_sysret() {
    // SAFETY: Enabling `syscall` and `sysret` does not violate memory safety.
    unsafe {
        Efer::update(|efer| efer.insert(EferFlags::SYSTEM_CALL_EXTENSIONS));
    }
}

fn register_syscall_handler() {
    extern "sysv64" {
        fn asm_handle_syscall();
    }

    LStar::write(VirtAddr::new(
        (asm_handle_syscall as usize).try_into().unwrap(),
    ));
}

#[no_mangle]
extern "sysv64" fn handle_syscall(index: u64, a1: u64, a2: u64) {
    match FromPrimitive::from_u64(index) {
        Some(Ty::Send) => {
            let to = Pid::new(a1.try_into().unwrap());

            let message = unsafe { ptr::get(a2 as *const _) };

            ipc::send(to, message);
        }
        Some(Ty::Receive) => {
            let from = if a1 == u64::MAX {
                ReceiveFrom::Any
            } else {
                ReceiveFrom::Pid(Pid::new(a1.try_into().unwrap()))
            };

            ipc::receive(from, a2 as *mut _);
        }
        None => panic!("Unknown index."),
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Ty {
    Send,
    Receive,
}
