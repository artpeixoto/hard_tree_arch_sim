pub struct ComponentBank<InnerComp: Sized, const COMPONENT_COUNT: usize>{
    pub components: Box<[InnerComp; COMPONENT_COUNT]>
}
