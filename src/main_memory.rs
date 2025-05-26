use std::ops::Deref;
use std::sync::{Arc, RwLock};
use bevy::prelude::Component;
use crate::memory_primitives::register::Register;
use crate::{ClockTransition, Step};
use crate::word::{Word};

pub const MAIN_MEMORY_LEN: usize = 1024;
type MainMemoryInner = Arc<[Register<Word>; MAIN_MEMORY_LEN]>;
pub enum ConnDirection{
    Input, Output
}
#[derive(Component)]
pub struct MainMemory(MainMemoryInner);

impl MainMemory{
    pub fn new() -> Self{
        MainMemory(Arc::new(
            std::array::from_fn(|i|
                Register::new(0)
            )
        ))
    }
}
impl ClockTransition for MainMemory{
    fn step(&mut self, step: &Step) {
        for r in self.0.iter_mut() {
            r.step(step);
        }
    }
}

pub struct MainMemoryIo{
    inner: MainMemoryInner,
}
impl MainMemory{
    pub fn get_io(&self) -> MainMemoryIo{
        MainMemoryIo {
            inner: self.0.clone()
        }
    }
}

impl MainMemoryIo {
    pub fn read<'a>(&'a self, addr: usize) -> impl Deref<Target=Word>+ 'a{
        self.inner[addr].read()
    }
    pub fn write(&self, addr: usize, value: Word) {
        self.inner[addr].write(value)
    }
}
