pub mod prelude;

use num::Num;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub struct Bitf<T: Num>(pub T);

macro_rules! bitf_impl {
    (
        iter<$iter_ty:ty> -> bitf<$bitf_ty:ty>,
        in = $iter_len:literal,
        shift = $shift:literal
    ) => {
        impl Into<Bitf<$bitf_ty>> for Iter<'_, $iter_ty> {
            #[inline(always)]
            fn into(self) -> Bitf<$bitf_ty> {
                assert_eq!(
                    self.len(),
                    $iter_len,
                    "Iterator length is not {}",
                    $iter_len
                );
                let base: $bitf_ty = 0;
                let bitf = self.fold(base, |acc, &b| (acc << $shift) | b as $bitf_ty);
                Bitf(bitf)
            }
        }
    };
}

bitf_impl!(iter<i8> -> bitf<i64>, in = 8, shift = 8);
bitf_impl!(iter<i16> -> bitf<i64>, in = 4, shift = 16);
bitf_impl!(iter<i32> -> bitf<i64>, in = 2, shift = 32);
bitf_impl!(iter<bool> -> bitf<i8>, in = 8, shift = 1);

// impl Into<Bitf<i64>> for Iter<'_, i8> {
//     fn into(self) -> Bitf<i64> {
//         assert_eq!(self.len(), 8, "Iterator length is not 8");
//         let bitf = self.fold(0i64, |acc, &b| (acc << 8) | b as i64);
//         Bitf(bitf)
//     }
// }

// impl Into<Bitf<i64>> for Iter<'_, i16> {
//     fn into(self) -> Bitf<i64> {
//         assert_eq!(self.len(), 4, "Iterator length is not 4");
//         let bitf = self.fold(0i64, |acc, &b| (acc << 16) | b as i64);
//         Bitf(bitf)
//     }
// }

// impl Into<Bitf<i64>> for Iter<'_, i32> {
//     fn into(self) -> Bitf<i64> {
//         assert_eq!(self.len(), 2, "Iterator length is not 2");
//         let bitf = self.fold(0i64, |acc, &b| (acc << 32) | b as i64);
//         Bitf(bitf)
//     }
// }

// impl Into<Bitf<i8>> for Iter<'_, bool> {
//     fn into(self) -> Bitf<i8> {
//         assert_eq!(self.len(), 8, "Iterator length is not 8");
//         let bitf = self.fold(0i8, |acc, &b| (acc << 1) | b as i8);
//         Bitf(bitf)
//     }
// }
