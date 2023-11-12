pub mod prelude;

use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub struct Bitf(i64);

impl Into<Bitf> for Iter<'_, i8> {
    fn into(self) -> Bitf {
        assert_eq!(self.len(), 8);

        let mut bitf = 0i64;
        self.for_each(|n| {
            bitf <<= 8;
            bitf |= *n as i64;
        });

        Bitf(bitf)
    }
}

impl Into<Bitf> for Iter<'_, i16> {
    fn into(self) -> Bitf {
        assert_eq!(self.len(), 4);

        let mut bitf = 0i64;
        self.for_each(|n| {
            bitf <<= 16;
            bitf |= *n as i64;
        });

        Bitf(bitf)
    }
}

impl Into<Bitf> for Iter<'_, i32> {
    fn into(self) -> Bitf {
        assert_eq!(self.len(), 2);

        let mut bitf = 0i64;
        self.for_each(|n| {
            bitf <<= 32;
            bitf |= *n as i64;
        });

        Bitf(bitf)
    }
}
