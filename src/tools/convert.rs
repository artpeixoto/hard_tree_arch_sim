
/// A small helper trait to make `Into<T>` a bit more ergonomic
/// 
/// Example
/// ```
/// 
/// ```
pub trait ConvertInto: Sized {
    #[inline(always)]
    fn convert<T>(self) -> T where Self: Into<T>{
        Into::into(self)
    } 
}
impl<U>  ConvertInto for U where U: Sized {}