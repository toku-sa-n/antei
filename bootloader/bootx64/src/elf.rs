use crate::Mapper;
use elfloader::ElfLoader;

struct Loader<'a> {
    binary: &'a [u8],
    mapper: &'a mut Mapper<'a>,
}
impl<'a> Loader<'a> {
    fn new(binary: &'a [u8], mapper: &'a mut Mapper<'a>) -> Self {
        Self { binary, mapper }
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
