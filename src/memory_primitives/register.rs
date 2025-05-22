use rust_hdl::{prelude::*, widgets::edge_ff::EdgeDFF};


#[derive(LogicBlock)]
pub struct Register<Data: Synth>{
	pub clock	: Signal<In, Clock>,
	write_enable: Signal<InOut, Bit>,
	write_value	: Signal<InOut, Data>,
	inner		: EdgeDFF<Data>,
}

impl<Data: Synth> Register<Data> {
	pub fn new(clock: Signal<In, Clock>)  -> Self {
		Self{
			clock,
			write_enable: Default::default(),
			write_value	: Default::default(),
			inner		: Default::default(),
		}
	}
}

impl<Data: Synth> Logic for Register<Data>{
	fn update(&mut self) {
		// activates at negative clock
		self.inner.clk.next = !self.clock.val();
		if !self.clock.val().clk{
			self.write_enable.next = false;
		} else { // clock is up
			if self.write_enable.val(){
				self.inner.d.next = self.write_value.val();
			}
		}
	}
}
impl<Data: Synth> Register<Data>{
	pub fn get_reader(&mut self) -> RegisterReader<Data>{
		RegisterReader{value: self.inner.q.clone()}
	}
	pub fn get_writer(&mut self) -> RegisterWriter<Data>{
		let mut write_enable_buffer = TristateBuffer::default();
		self.write_enable.join(&mut write_enable_buffer.bus);
		let mut write_value_buffer = TristateBuffer::default();
		self.write_value.join(&mut write_value_buffer.bus);


		RegisterWriter { 
			write_enable: Default::default(), 
			write_value: Default::default(), 
			clock: self.clock.clone(),
			inner_value: write_value_buffer, 
			inner_write_enable: write_enable_buffer, 
		}
	}
}

#[derive(LogicBlock)]
pub struct RegisterReader<Data: Synth>{
	pub value: Signal<Out, Data>
}
impl<Data: Synth> Logic for RegisterReader<Data>{
	#[hdl_gen]
	fn update(&mut self) {
	}
}

#[derive(LogicBlock)]
pub struct RegisterWriter<Data: Synth>{
	pub write_enable: Signal<In, Bit>,
	pub write_value	: Signal<In, Data>,
	clock: Signal<In, Clock>,
	inner_value: TristateBuffer<Data>,
	inner_write_enable: TristateBuffer<Bit>,
}

impl<Data: Synth> Logic for RegisterWriter<Data>{
	fn update(&mut self) {
		self.inner_value.write_enable.next = false;
		self.inner_write_enable.write_enable.next = false;

		if self.clock.val().clk {  //i know i could collapse, but i don't know, for some reason it doesnt seem right. Maybe because i want to separate the concept of doing somthing in the up and then doing some other thing in the down
			if self.write_enable.val(){
				self.inner_value.write_data.next 			= self.write_value.val();
				self.inner_value.write_enable.next 			= true;
				self.inner_write_enable.write_data.next 	= true;
				self.inner_write_enable.write_enable.next 	= true;
			} 
		} 	
	}
}

#[derive(Clone, Copy,Debug, PartialEq, Eq, LogicState)]
pub enum RegisterRwCommand {
    Write, Read,
}

#[derive(LogicBlock)]
pub struct RegisterRw<Data: Synth>{
	pub cmd: Signal<In, RegisterRwCommand>,
	pub value: Signal<InOut, Data>,

	reader	: RegisterReader <Data>,
	writer	: RegisterWriter <Data>

}
impl<Data:Synth> Logic for RegisterRw<Data> {
	fn update(&mut self) {
		match self.cmd.val(){
			RegisterRwCommand::Read  =>{
				self.writer.write_enable.next = false;
				self.value.next = self.reader.value.val();
			} 
			RegisterRwCommand::Write => {
				self.writer.write_value.next = self.value.val();
				self.writer.write_enable.next = true;
			}
		}
	}
}