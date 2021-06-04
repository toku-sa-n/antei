use crate::Allocator;
use xmas_elf::ElfFile;

struct Mapper<'a> {
    binary: &'a [u8],
    file: ElfFile<'a>,
    allocator: &'a mut Allocator<'a>,
}
impl<'a> Mapper<'a> {
    fn new(binary: &'a [u8], allocator: &'a mut Allocator<'a>) -> Self {
        let file = ElfFile::new(binary);
        let file = file.expect("Not an ELF file.");

        Self {
            binary,
            file,
            allocator,
        }
    }
}
