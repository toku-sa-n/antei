use crate::Allocator;
use elfloader::ElfLoader;

struct Loader<'a> {
    allocator: &'a mut Allocator<'a>,
    binary: &'a [u8],
}
impl<'a> Loader<'a> {
    fn new(binary: &'a [u8], allocator: &'a mut Allocator<'a>) -> Self {
        Self { allocator, binary }
    }
}
impl ElfLoader for Loader<'_> {
    fn allocate(&mut self, load_headers: elfloader::LoadableHeaders) -> Result<(), &'static str> {
        todo!()
    }

    fn load(
        &mut self,
        flags: elfloader::Flags,
        base: elfloader::VAddr,
        region: &[u8],
    ) -> Result<(), &'static str> {
        todo!()
    }

    fn relocate(&mut self, entry: &elfloader::Rela<elfloader::P64>) -> Result<(), &'static str> {
        todo!()
    }
}
