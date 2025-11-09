use crate::arena::Arena;

trait Allocatable<'a> {
    fn write_into(&self, arena: &'a mut Arena) -> Self;
}

impl<'a> Allocatable<'a> for u8 {
    fn write_into(&self, arena: &'a mut Arena) -> Self {
        *self
    }
}
