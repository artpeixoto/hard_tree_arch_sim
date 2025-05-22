use rust_hdl::prelude::*;
use crate::{instruction::TakeBits, cpu_registers::{CpuRegistersAddress, CPU_REGISTERS_ADDR_SIZE}, memory_primitives::{register::{RegisterReader, RegisterWriter}, register_bank::RegisterBank}};

use super::{core::AluOperation};
pub const ALU_COUNT			 		: usize = 32;
pub const ALU_ADDR_SIZE		    : usize = clog2(ALU_COUNT);
pub type AluAddr				= Bits<ALU_ADDR_SIZE>;
pub type AluConfigReader 		= RegisterReader<AluConfig>;

#[derive(Clone, PartialEq, Eq, Debug, Copy, Default,LogicStruct )]
pub struct AluConfig {
    pub execution_signal_input	: CpuRegistersAddress,
    pub execution_signal_output	: CpuRegistersAddress,
    pub data_input_0			: CpuRegistersAddress,
    pub data_input_1			: CpuRegistersAddress,
    pub main_data_output		: CpuRegistersAddress,
    pub aux_data_output			: CpuRegistersAddress,
    pub aux_output_activated	: bool,
    pub operation				: AluOperation,
}
impl<'a> Into<AluConfig> for &'a [bool]{
	fn into(self) -> AluConfig {
		if self.len() != AluConfig::BITS {
			panic!()
		}
		let mut arr = &self[..];
		let execution_signal_input = arr.take_bits();
		let execution_signal_output = arr.take_bits();
		let data_input_0 = arr.take_bits();
		let data_input_1 = arr.take_bits();
		let main_data_output = arr.take_bits();
		let aux_data_output  = arr.take_bits();
		let aux_output_activated  = arr.take_bits::<1>().into();
		let operation 	= arr.take_bits::<{AluOperation::BITS}>().into();

		AluConfig{
			execution_signal_input,
			execution_signal_output,
			data_input_0,
			data_input_1,
			main_data_output,
			aux_data_output,
			aux_output_activated,
			operation,
		}
	}
}

pub const ALU_CONFIG_SIGNAL_SIZE: usize
    = (6 * CpuRegistersAddress::BITS)  	// 6 ios
    + AluOperation::BITS  			    // operation
    + 1							    	// aux_output_activated
;


#[derive(LogicBlock)]
pub struct AluConfigBank {
	inner: RegisterBank<AluConfig, ALU_ADDR_SIZE, ALU_COUNT>
}

impl Logic for AluConfigBank{
	fn update(&mut self) {} //amazin
}
impl AluConfigBank{
	pub fn get_configurator(&mut self) -> AluBankConfigurator{
		let inner_writers = Box::new(std::array::from_fn(|i|self.inner.get_specific_writer(i)));
		AluBankConfigurator { write_enable: Default::default(), target_all: Default::default(), addr: Default::default(), config_value: Default::default(),inner_writers }
	}
	pub fn get_reader(&mut self, alu_addr: usize) -> AluConfigReader {
		self.inner.get_specific_reader(alu_addr)
	}
}


#[derive(LogicBlock)]
pub struct AluBankConfigurator{
    pub write_enable: Signal<In, bool>,
	pub target_all	: Signal<In, bool>,
    pub addr		: Signal<In, AluAddr>,
	pub config_value: Signal<In, AluConfig>,
    inner_writers	: Box<[RegisterWriter<AluConfig>; ALU_COUNT]>
}


impl Logic for AluBankConfigurator{
	#[hdl_gen]
    fn update(&mut self) {
		for i in 0..ALU_COUNT{
			self.inner_writers[i].write_enable.next = false;
		}
		if self.write_enable.val() {
			if self.target_all.val() {
				for i in 0..ALU_COUNT{
					self.inner_writers[i].write_value.next = self.config_value.val();
					self.inner_writers[i].write_enable.next = true;
				}	
			} else {
				self.inner_writers[self.addr.val().index()].write_value.next = self.config_value.val();
				self.inner_writers[self.addr.val().index()].write_enable.next = true;
			}
		}
    }
}
