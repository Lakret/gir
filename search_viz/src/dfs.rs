use bitvec::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
// `BitArr!(...)` expands to `BitArray<[u16; ::bitvec::mem::elts::<u16>(9)], Msb0>`
pub struct Game(BitArr!(for 9, in u16, Msb0));
