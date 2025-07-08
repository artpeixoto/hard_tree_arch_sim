pub trait UsedIn: Sized{
    
    #[inline(always)]
    fn used_in<F: FnOnce(Self) -> O , O>(self, fun: F) -> O{
        fun(self) 
    }
}
impl<T> UsedIn for T where T: Sized{}