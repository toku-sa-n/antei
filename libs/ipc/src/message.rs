use pid::Pid;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Message {
    pub header: Header,
    pub body: Body,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Header {
    pub sender_pid: Pid,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Body(pub u64, pub u64, pub u64, pub u64, pub u64);
