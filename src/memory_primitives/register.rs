use std::ops::Deref;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use bevy::asset::uuid::Uuid;
use bevy::prelude::*;
use crate::{Step, ClockTransition};

#[derive(Component)]

pub struct Register<Data>{
	next_value	: Arc<RwLock<Option<Data>>>,
	value		: Arc<RwLock<Data>>,
}


impl<Data> ClockTransition for Register<Data>{
	fn step(&mut self, _step: &Step) {
		if let Some(value) = self.next_value.write().unwrap().take() {
			*self.value.write().unwrap() = value;
		}
	}
}

impl<Data> Register<Data>{
	pub fn read<'a>(&'a self) -> impl Deref<Target=Data> + 'a{
		self.value.read().unwrap()
	}

	pub fn write<'a>(&'a self, new_value:  Data) {
		self.next_value.write().unwrap().replace(new_value);
	}

	pub fn new(initial_value: Data) -> Register<Data>{
		Register{
			next_value: Arc::new(RwLock::new(None)),
			value: Arc::new(RwLock::new(initial_value)),	
		}
	}

	pub fn get_reader(& self) -> RegisterReader<Data>{
		RegisterReader{
			value: self.value.clone(), 
		}
	}
	pub fn get_writer(& self) -> RegisterWriter<Data>{
		RegisterWriter{
			write: self.next_value.clone(),
		}	
	} 
}

#[derive(Component)]
pub struct RegisterReader<Data>{
	value: Arc<RwLock<Data>>
}

impl<Data> RegisterReader<Data>{
	pub fn read<'a>(&'a self) -> impl Deref<Target=Data> + 'a {
		self.value.read().unwrap()
	}
	pub fn is_same(&self, reg: &Register<Data>) -> bool{
		self.value.deref() as *const _ == reg.value.deref() as *const _
	}
}

#[derive(Component)]
pub struct RegisterWriter<Data>{
	write	: Arc<RwLock<Option<Data>>>,
}

impl<Data> RegisterWriter<Data>{
	pub fn write(&mut self, value: Data) {
		*self.write.write().unwrap() = Some(value);
	}

	pub fn is_same(&self, reg: &Register<Data>) -> bool{
		self.write.deref() as *const _ == reg.next_value.deref() as *const _
	}
}