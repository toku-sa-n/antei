use super::Process;

struct Manager<const N: usize> {
    processes: [Option<Process>; N],
}
