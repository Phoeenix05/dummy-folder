use std::mem::size_of;

use bitformation::prelude::*;

fn main() {
    let slice = [true, false, true, false, true, false, true, false];
    let bitf: Bitf<i8> = slice.iter().into();
    println!("{:?}, {:08b}", bitf.0, bitf.0);

    println!("bool: {}", size_of::<Bitf<i8>>())
}
