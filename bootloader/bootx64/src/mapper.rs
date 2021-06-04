use crate::Allocator;

struct Mapper<'a> {
    allocator: &'a mut Allocator<'a>,
}
impl<'a> Mapper<'a> {
    fn new(allocator: &'a mut Allocator<'a>) -> Self {
        Self { allocator }
    }
}
