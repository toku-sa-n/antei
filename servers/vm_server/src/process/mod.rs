mod manager;

use {pid::Pid, x86_64::structures::paging::PhysFrame};

struct Process {
    pid: Pid,
    pml4: PhysFrame,
}
