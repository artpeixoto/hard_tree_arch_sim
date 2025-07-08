pub type Word = i32;

pub trait ToWord {
    fn to_word(&self) -> i32;
}
impl ToWord for bool{
    fn to_word(&self) -> i32{
        match self{
            &true  => {!0}
            &false => {0}
        }
    }
}
pub trait ToBool{
    fn to_bool(&self) -> bool;
}
impl ToBool for Word{
    fn to_bool(&self) -> bool {
        self != &0
    }
}