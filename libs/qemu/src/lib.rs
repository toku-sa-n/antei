#![no_std]

use {
    static_assertions::const_assert_eq,
    x86_64::instructions::{hlt, port::PortWriteOnly},
};

const IO_BASE: u16 = 0xf4;

enum Code {
    Success = 33,
    Failure = 35,
}

const_assert_eq!((Code::Success as u32) & 1, 1);
const_assert_eq!((Code::Failure as u32) & 1, 1);

pub fn exit_success() -> ! {
    exit_with_code(Code::Success);
}

pub fn exit_failure() -> ! {
    exit_with_code(Code::Failure);
}

fn exit_with_code(code: Code) -> ! {
    try_exit_with_code(code);
    halt_and_loop();
}

fn try_exit_with_code(code: Code) {
    let mut port = PortWriteOnly::new(IO_BASE);
    let code = (code as u16) >> 1;

    // SAFETY: `IO_BASE` must be the correct one.
    unsafe {
        port.write(code);
    }
}

fn halt_and_loop() -> ! {
    loop {
        hlt();
    }
}
