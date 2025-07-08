use std::collections::HashMap;
use super::AluOperation;
use crate::application::draw::port::SignalType::Activation;
use crate::application::draw::port::{PortData, PortSignalDirection, SignalType};
use crate::application::grid::component::{PortDataContainer, PortName};
use crate::application::simulation::alu::AluPortName::{
    ActivationIn, ActivationOut, DataIn0, DataIn1, DataOut0, DataOut1,
};
use crate::application::simulation::main_memory::{MainMemory, MainMemoryIo};
use crate::memory_primitives::register::Register;
use crate::word::{ToBool, ToWord, Word};
use std::mem::transmute;
use std::ops::Index;
use PortSignalDirection::{Input, Output};
use SignalType::Data;
use crate::application::simulation::cpu_registers::{CpuRegisterActReader, CpuRegisterActWriter, CpuRegisterDataReader, CpuRegisterDataWriter, CpuRegisterReadRequest, CpuRegisterWriteRequest};

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AluCoreState {
    Normal,
    Waiting,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AluPortName {
    DataIn0,
    DataIn1,
    ActivationIn,

    DataOut0,
    DataOut1,
    ActivationOut,
    // SetupIn,
}

impl PortName for AluPortName {
    fn all_port_names() -> Vec<Self> {
        vec![
            DataIn0,
            DataIn1,
            ActivationIn,
            DataOut0,
            DataOut1,
            ActivationOut,
        ]
    }

    fn small_name(&self) -> &str {
        match self {
            DataIn0 => "di0",
            DataIn1 => "di1",
            ActivationIn => "act_i",
            DataOut0 => "do0",
            DataOut1 => "do1",
            ActivationOut => "act_o",
        }
    }
}

pub struct AluPortsData {
    // pub state_in            : PortInfo,
    pub data_input_0: PortData,
    pub data_input_1: PortData,
    pub activation_input: PortData,

    pub data_output_0: PortData,
    pub data_output_1: PortData,
    pub activation_output: PortData,
}
impl PortDataContainer<AluPortName, PortData> for AluPortsData {
    fn get_for_port(&self, port_name: &AluPortName) -> &PortData {
        match port_name {
            ActivationIn => &self.activation_input,
            DataIn0 => &self.data_input_0,
            DataIn1 => &self.data_input_1,
            DataOut0 => &self.data_output_0,
            DataOut1 => &self.data_output_1,
            ActivationOut => &self.activation_output,
        }
    }
}

pub struct AluCore {
    pub addr: usize,
    pub operation       : AluOperation,
    pub old_operation   : AluOperation,
    pub main_memory     : MainMemoryIo,

    pub inner_memory_0  : Word,
    pub inner_memory_1  : Word,

    pub data_input_0    : CpuRegisterDataReader,
    pub data_input_1    : CpuRegisterDataReader,
    pub activation_input: CpuRegisterActReader,

    pub data_output_0   : CpuRegisterDataWriter,
    pub data_output_1   : CpuRegisterDataWriter,
    pub activation_output: CpuRegisterActWriter,
}

impl AluCore {
    pub fn collect_read_requests<'a>(&'a mut self) -> HashMap<AluPortName, CpuRegisterReadRequest<'a>> {
        [
            (DataIn0, self.data_input_0.get_read_request()),
            (DataIn1, self.data_input_1.get_read_request()),
            (ActivationIn, self.activation_input.get_read_request()),
        ]
        .into_iter()
        .filter_map(|(name, opt_req)|
            opt_req.map(|req| (name, req))
        )
        .collect()
    }
    pub fn collect_write_requests(&mut self) -> HashMap<AluPortName, CpuRegisterWriteRequest> {
        [
            (DataOut0, self.data_output_0.get_write_request()),
            (DataOut1, self.data_output_1.get_write_request()),
            (ActivationOut, self.activation_output.get_write_request()),
        ]
        .into_iter()
        .filter_map(|(name, opt_req)|
            opt_req.map(|req| (name, req))
        )
        .collect()
    }
    pub fn new(alu_addr: usize, main_memory: &MainMemory) -> Self {
        AluCore {
            addr                : alu_addr,
            main_memory         : main_memory.get_io(),
            operation           : AluOperation::NoOp,
            old_operation       : AluOperation::NoOp,

            inner_memory_0      : Default::default(),
            inner_memory_1      : Default::default(),

            data_input_0        : CpuRegisterDataReader::new(),
            data_input_1        : CpuRegisterDataReader::new(),
            activation_input    : CpuRegisterActReader::new(),

            data_output_0       : CpuRegisterDataWriter::new(),
            data_output_1       : CpuRegisterDataWriter::new(),
            activation_output   : CpuRegisterActWriter::new(),
        }
    }
    pub fn get_ports_info(&self) -> AluPortsData {
        let ports_data = self.operation.get_ports_config();
        AluPortsData {
            data_input_0: PortData {
                active: ports_data.data_input_0.is_some(),
                signal_dir: Input,
                signal_type: Data,
            },
            data_input_1: PortData {
                active: ports_data.data_input_1.is_some(),
                signal_dir: Input,
                signal_type: Data,
            },
            activation_input: PortData {
                active: ports_data.activation_input.is_some(),
                signal_dir: Input,
                signal_type: Activation,
            },
            data_output_0: PortData {
                active: ports_data.data_output_0.is_some(),
                signal_dir: Output,
                signal_type: Data,
            },
            data_output_1: PortData {
                active: ports_data.data_output_1.is_some(),
                signal_dir: Output,
                signal_type: Data,
            },
            activation_output: PortData {
                active: ports_data.activation_output.is_some(),
                signal_dir: Output,
                signal_type: Activation,
            },
        }
    }


    pub fn set_new_operation(&mut self, new_operation: AluOperation){
        self.old_operation = self.operation.clone();
        self.operation = new_operation.clone();

        let ports_config = new_operation.get_ports_config();
        self.data_input_0.set_connection(ports_config.data_input_0);
        self.data_input_1.set_connection(ports_config.data_input_1);
        self.activation_input
            .set_connection(ports_config.activation_input);
        self.data_output_0
            .set_connection(ports_config.data_output_0);
        self.data_output_1
            .set_connection(ports_config.data_output_1);
        self.activation_output
            .set_connection(ports_config.activation_output);
        self.inner_memory_0 = 0;
        self.inner_memory_1 = 0;
    }

    pub fn execute(&mut self) {
        let op = self.operation;
        match &op {
            AluOperation::NoOp => {}
            AluOperation::Eq { .. } => {
                if self.activation_input.read().unwrap() {
                    let in_0 = self.data_input_0.read().unwrap();
                    let in_1 = self.data_input_1.read().unwrap();
                    let res = in_0 == in_1;
                    self.data_output_0.write(res.to_word());
                    self.activation_output.write(true)
                } else {
                    self.activation_output.write(false)
                }
            }
            // AluOperation::Mov {
            //     activation_input,
            //     ..
            //     // address_input,
            //     // data_output,
            //     // activation_output
            // } => {
            //     if self.activation_input.read(){
            //         let in_addr = match address_input{
            //             MovInput::Source(source) => *source,
            //             // MovInput::SourceAddr(src_addr) => {
            //             //     *self.data_input_1.read() as usize
            //             // }
            //         };
            //         // let res = self.data_input_0.read(in_addr);
            //
            //         self.data_output_0.write(*res);
            //
            //         activation_output.map(
            //             |addr| self.activation_output.write(true, addr)
            //         );
            //     } else {
            //         activation_output.map(
            //             |addr| self.activation_output.write(false, addr)
            //         );
            //     }
            // }
            AluOperation::Not {
                activation_input,
                data_input,
                data_output,
                activation_output,
            } => {
                if self.activation_input.read().unwrap() {
                    let inp = self.data_input_0.read().unwrap();
                    self.data_output_0.write(!inp);
                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(true);
                }
            }
            AluOperation::And {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 & inp_1;
                    self.data_output_0.write(res);
                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(true);
                }
            }
            AluOperation::Or {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 | inp_1;
                    self.data_output_0.write(res);
                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::Xor {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 ^ inp_1;
                    self.data_output_0.write(res);
                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::ShiftLeft {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 << inp_1;
                    self.data_output_0.write(res);

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::ShiftRight {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let res = inp_0 >> inp_1;
                    self.data_output_0.write(res);

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::SelectPart { .. } => {
                todo!()
            }
            AluOperation::Add { ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let (first_word, overflow) = inp_0.overflowing_add(inp_1);
                    self.data_output_0.write(first_word);
                    self.data_output_1.write(overflow as i32 );
                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::Sub {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    let (first_word, overflow) = inp_0.overflowing_sub(inp_1);
                    self.data_output_0.write(first_word,);
                    self.data_output_1.write(overflow as i32);

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::Mul {
                second_word_output,
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let inp_0 = self.data_input_0.read().unwrap();
                    let inp_1 = self.data_input_1.read().unwrap();

                    if let Some(_second_word_output) = second_word_output {
                        let (first_word_res, second_word_res) =
                            (inp_0 as i32).widening_mul(inp_1 as i32);
                        self.data_output_0
                            .write(unsafe { transmute(first_word_res) },);
                        self.data_output_1
                            .write(second_word_res);
                    } else {
                        self.data_output_0
                            .write((inp_0) * (inp_1), );
                    }

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::Div {
                div_by_zero_flag_output,
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let dividend = self.data_input_0.read().unwrap();
                    let divisor = self.data_input_1.read().unwrap();

                    if divisor == 0 {
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(1);
                        }
                        self.data_output_0.write(0);
                    } else {
                        let res = dividend / divisor;
                        self.data_output_1.write(res);
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(0);
                        }
                    }

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::Rem {
                div_by_zero_flag_output,
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let dividend = self.data_input_0.read().unwrap();
                    let divisor = self.data_input_1.read().unwrap();

                    if divisor == 0 {
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(1, );
                        }
                        self.data_output_0.write(0);
                    } else {
                        let res = dividend % divisor;
                        self.data_output_1.write(res);
                        if let Some(_div_by_zero_flag_output) = div_by_zero_flag_output {
                            self.data_output_1.write(0);
                        }
                    }

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::Neg {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let res = -self.data_input_0.read().unwrap();
                    self.data_output_0.write(res);

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::ReadFromMem {
                ..
            } => {
                if self.activation_input.read() .unwrap(){
                    let addr = self.data_input_0.read().unwrap();
                    let res = self.main_memory.read(addr as usize);
                    self.data_output_0.write(res);

                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::WriteToMem {
                ..
            } => {
                if self.activation_input.read().unwrap() {
                    let addr = self.data_input_0.read().unwrap();
                    let data = self.data_input_1.read().unwrap();
                    self.main_memory.write(addr as usize, data);
                    self.activation_output.write(true);
                } else {
                    self.activation_output.write(false);
                }
            }
            AluOperation::Latch {
                ..
            } => {
                let hold_input = self.data_input_1.read().unwrap().to_bool();
                let previous_hold = self.inner_memory_1.to_bool();
                if hold_input {
                    if self.activation_input.read().unwrap() {
                        if !previous_hold {
                            let current_data = self.data_input_0.read().unwrap();
                            self.inner_memory_0 = current_data;
                            self.data_output_0.write(current_data);
                            todo!()
                        } else {
                        }
                    }
                }
                todo!()
            }
        }
    }
}

