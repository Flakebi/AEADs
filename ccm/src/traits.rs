use aead::consts::{U4, U6, U7, U8, U9, U10, U11, U12, U13, U14, U16};
use aead::generic_array::typenum::Unsigned;

mod private {
    // Sealed traits stop other crates from implementing any traits that use it.
    pub trait SealedTag {}
    pub trait SealedNonce {}

    impl SealedTag for super::U4 {}
    impl SealedTag for super::U6 {}
    impl SealedTag for super::U8 {}
    impl SealedTag for super::U10 {}
    impl SealedTag for super::U12 {}
    impl SealedTag for super::U14 {}
    impl SealedTag for super::U16 {}

    impl SealedNonce for super::U7 {}
    impl SealedNonce for super::U8 {}
    impl SealedNonce for super::U9 {}
    impl SealedNonce for super::U10 {}
    impl SealedNonce for super::U11 {}
    impl SealedNonce for super::U12 {}
    impl SealedNonce for super::U13 {}
}

// TODO: make sealed
pub trait TagSize: Unsigned + private::SealedTag {
    fn get_m_tick() -> u8 {
        (Self::to_u8() - 2)/2
    }
}
pub trait NonceSize: Unsigned + private::SealedNonce {
    fn get_l() -> u8 {
        15 - Self::to_u8()
    }
}

impl TagSize for U4 {}
impl TagSize for U6 {}
impl TagSize for U8 {}
impl TagSize for U10 {}
impl TagSize for U12 {}
impl TagSize for U14 {}
impl TagSize for U16 {}

impl NonceSize for U7 {}
impl NonceSize for U8 {}
impl NonceSize for U9 {}
impl NonceSize for U10 {}
impl NonceSize for U11 {}
impl NonceSize for U12 {}
impl NonceSize for U13 {}
