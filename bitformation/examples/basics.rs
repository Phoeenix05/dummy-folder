use bitformation::prelude::*;

fn main() {
    let slice: [i16; 4] = [125, 2, 3, 4];
    let bitf: Bitf = slice.iter().into();
    println!("{:?}", bitf);
}
