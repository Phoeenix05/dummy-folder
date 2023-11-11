pub mod prelude;

use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub struct Bitf(i64);

impl Into<Bitf> for Iter<'_, i8> {
    fn into(self) -> Bitf {
        todo!()
    }
}

impl Into<Bitf> for Iter<'_, i16> {
    fn into(self) -> Bitf {
        assert_eq!(self.len(), 4);

        // let mut bitf: i64 = 0;
        // bitf <<= 8;
        // bitf |= self.as_slice()[0] as i64;
        // println!("{bitf:064b}");
        let mut bitf = 0i64;
        // bitf <<= 8;
        // bitf |= self.as_slice()[0] as i64;
        self.for_each(|n| {
            bitf <<= 16;
            bitf |= *n as i64;
        });
        println!("{bitf:064b}");

        Bitf(bitf)
    }
}

impl Into<Bitf> for Iter<'_, i32> {
    fn into(self) -> Bitf {
        todo!()
    }
}

// 0000000001111101 
// 0000000000000010
// 0000000000000011
// 0000000000000100
