pub struct ComponentBank<C: Sized, const COMPONENT_COUNT: usize>{
    pub components: Box<[C; COMPONENT_COUNT]> 
}
