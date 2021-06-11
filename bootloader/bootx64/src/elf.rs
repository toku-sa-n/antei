use crate::Allocator;
use elfloader::ElfLoader;

struct Mapper<'a> {
    allocator: &'a mut Allocator<'a>,
    binary: &'a [u8],
}
impl<'a> Mapper<'a> {
    fn new(binary: &'a [u8], allocator: &'a mut Allocator<'a>) -> Self {
        Self { allocator, binary }
    }
}
