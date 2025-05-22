use rust_hdl::prelude::{signed_cast, unsigned_cast, Bits, Signed};

pub trait SignedCast<const N: usize>: Sized {
    fn signed(self) -> Signed<N>;
}

impl<const N: usize> SignedCast<N> for Bits<N> {
    #[inline(always)]
    fn signed(self) -> Signed<N> {
        signed_cast(self)
    }
}
impl<const N: usize> UnsignedCast<N> for Signed<N> {
    #[inline(always)]
    fn unsigned(self) -> Bits<N> {
        unsigned_cast(self) 
    }
}

pub trait UnsignedCast<const N: usize>: Sized {
    fn unsigned(self) -> Bits<N>;
}
