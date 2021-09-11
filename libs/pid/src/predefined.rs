use super::Pid;

pub const IDLE: Pid = Pid::new(0);
pub const INIT: Pid = Pid::new(1);
pub const SYSPROC: Pid = Pid::new(2);
pub const PM: Pid = Pid::new(3);
