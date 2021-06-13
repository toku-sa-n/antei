use crate::allocator::Allocator;
use crate::paging;
use crate::Mapper;
use core::convert::TryInto;
use core::ptr;
use elfloader::ElfBinary;
use elfloader::ElfLoader;
use elfloader::ElfLoaderErr;
use elfloader::ProgramHeader;
use elfloader::VAddr;
use os_units::Bytes;
use uefi_wrapper::service::boot::MemoryDescriptor;
use x86_64::structures::paging::PageTableFlags;
use x86_64::VirtAddr;

pub fn load(binary: &[u8], mmap: &mut [MemoryDescriptor]) {
    paging::disable_write_protect();
    unsafe { paging::enable_recursive_paging() };

    let mut allocator = Allocator::new(mmap);
    let mut mapper = unsafe { Mapper::new(&mut allocator) };
    let mut loader = Loader::new(&mut mapper);
    let elf = ElfBinary::new(binary);
    let elf = elf.expect("Not a ELF file.");

    let r = elf.load(&mut loader);
    r.expect("Failed to load a ELF file.");

    paging::enable_write_protect();
}

struct Loader<'a> {
    mapper: &'a mut Mapper<'a>,
}
impl<'a> Loader<'a> {
    fn new(mapper: &'a mut Mapper<'a>) -> Self {
        Self { mapper }
    }

    fn allocate_for_header(&mut self, h: ProgramHeader<'_>) {
        let v = VirtAddr::new(h.virtual_addr());

        let bytes = Bytes::new(h.mem_size().try_into().unwrap());

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        // SAFETY: The page will be used to load the ELF file. The memory does not have to be
        // initialized.
        unsafe {
            self.mapper
                .map_range_to_unused(v, bytes.as_num_of_pages(), flags)
        };
    }
}
impl ElfLoader for Loader<'_> {
    fn allocate(
        &mut self,
        load_headers: elfloader::LoadableHeaders<'_, '_>,
    ) -> Result<(), ElfLoaderErr> {
        for h in load_headers {
            self.allocate_for_header(h);
        }

        Ok(())
    }

    fn load(
        &mut self,
        flags: elfloader::Flags,
        base: elfloader::VAddr,
        region: &[u8],
    ) -> Result<(), ElfLoaderErr> {
        let base = VirtAddr::new(base);

        // SAFETY: Memory is allocated by the previous `allocate` call.
        unsafe { ptr::copy(region.as_ptr(), base.as_mut_ptr(), region.len()) };

        if !flags.is_write() {
            let r = self.make_readonly(base.as_u64(), region.len());
            r.expect("Failed to make a region readonly.");
        }

        Ok(())
    }

    fn relocate(&mut self, _: &elfloader::Rela<elfloader::P64>) -> Result<(), ElfLoaderErr> {
        unimplemented!("The kernel must not have a relocation section.")
    }

    fn make_readonly(&mut self, base: VAddr, size: usize) -> Result<(), ElfLoaderErr> {
        let base = VirtAddr::new(base);

        let bytes = Bytes::new(size);

        let n = bytes.as_num_of_pages();

        unsafe {
            self.mapper
                .update_flags_for_range(base, n, PageTableFlags::PRESENT)
        }
        Ok(())
    }
}
