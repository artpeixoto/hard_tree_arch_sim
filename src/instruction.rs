use std::{ iter::repeat_n};
use rust_hdl::{prelude::*};
use crate::{alu::{AluAddr, AluOperation, ALU_ADDR_SIZE, ALU_CONFIG_SIGNAL_SIZE}, cpu_registers::CpuRegisterAddress, word::{ToWord, Word, WORD_SIZE}};

pub const CONTROLLER_INSTRUCTION_SIZE   		: usize = 64;

#[derive( PartialEq, Copy, Clone, Debug,Eq, Default)]
pub enum Instruction {
    SetAluConfig{
        alu_addr	: AluAddr,
        alu_config	: AluOperation
    },

    SetLiteral{
        register_index  : CpuRegisterAddress,
        literal			: Word,
    },

    // PopStack{
    //     register_index	: CpuRegistersAddress,
    // },
    //
    // PushToStack{
    //     register_index	: CpuRegistersAddress,
    // },

    WaitForActivationSignal{
        register_index  : CpuRegisterAddress
    },

    Jump{
        relative        : bool,
        addr            : Word
    },
    
    ResetAll,

    #[default]
    NoOp,
}


impl From<Bits<CONTROLLER_INSTRUCTION_SIZE>> for Instruction {
	fn from(val: Bits<CONTROLLER_INSTRUCTION_SIZE>) -> Self {
        let bit_vec = val.to_bit_vec().0;
        if bit_vec[0]{
            let mut index = 1; let mut next = index + ALU_ADDR_SIZE;
            let alu_addr = (&bit_vec[index..next]).to_bits(); 
            index = next; next = next + ALU_CONFIG_SIGNAL_SIZE;
            let alu_config = (&bit_vec[index..next]).into();
            return Instruction::SetAluConfig { 
                alu_addr, 
                alu_config  
            };
        } else {
            let mut bit_vec = &bit_vec[1..]; 
            match bit_vec.take_bits::<3>().to_u8(){
                0b000 => {
                    Instruction::SetLiteral { 
                        register_index  : bit_vec.take_bits(), 
                        literal         : bit_vec.take_bits::<WORD_SIZE>().to_word()
                    }
                }
                // 0b001 => ControllerInstruction::PopStack {
                //     register_index: bit_vec.take_bits()
                // },
                // 0b010 => ControllerInstruction::PushToStack {
                //     register_index: bit_vec.take_bits()
                // },
                0b011 => Instruction::WaitForActivationSignal { 
                    register_index: bit_vec.take_bits()
                },
                0b100 => Instruction::Jump { 
                    relative: bit_vec.take_bits::<1>().into(),
                    addr: bit_vec.take_bits::<WORD_SIZE>().into()
                },
                0b101 => Instruction::ResetAll,
                0b111 => Instruction::NoOp,
                _     => panic!()
            }
        }
	}
}

impl Synth for Instruction {
    const BITS: usize = CONTROLLER_INSTRUCTION_SIZE;

    fn descriptor() -> TypeDescriptor {
        TypeDescriptor{
            name: "ControllerInstruction".to_string(),
            kind: TypeKind::Bits(CONTROLLER_INSTRUCTION_SIZE)
        }
    }

    fn vcd(self) -> VCDValue {
        self.to_bits::<64>().vcd()
        // use Either::{Left, Right};
        // enum StringOrBinary<'a>{
        //     String(Cow<'a, str>),
        //     Binary(Vec<vcd::Value>)
        // }
        // impl<'a> Into<StringOrBinary<'a>> for &'a str{
        //     fn into(self) -> StringOrBinary<'a> {
        //         StringOrBinary::String(Cow::Borrowed(self))
        //     }
        // }
        
        // impl<'a> Into<StringOrBinary<'a>> for &'a str{
        //     fn into(self) -> StringOrBinary<'a> {
        //         StringOrBinary::String(Cow::Borrowed(self))
        //     }
        // }
        // struct ToVcdValueMotherfucker{
        //     name: Cow<'static, str>,
        //     values: Option<Vec<(&'static str, StringOrBinary<'static>)>>
        // }
        // impl Into<VCDValue> for ToVcdValueMotherfucker{
        //     fn into(self) -> VCDValue {
        //         if self.values.is_none(){
        //             VCDValue::String(self.name.into_owned())
        //         } else {
        //             VCDValue::Composite([
        //                 VCDValue::String(self.name.into_owned()),
        //                 VCDValue::Composite(
        //                     self.values
        //                     .as_ref()
        //                     .unwrap()
        //                     .iter()
        //                     .map(|(name, value)|
        //                         VCDValue::Composite(vec![ 
        //                             Box::new(VCDValue::String(name.to_string())),
        //                             Box::new({ match value{
        //                                 StringOrBinary::String(cow) => VCDValue::String(cow.clone().into_owned()),
        //                                 StringOrBinary::Binary(values) => VCDValue::Vector(values.clone()),
        //                             }})
        //                         ])
        //                     )
        //                     .map(|x| Box::new(x))
        //                     .collect::<Vec<_>>()
        //                 ),

        //             ].into_iter().map(|x| Box::new(x)).collect::<Vec<_>>())
        //         }
        //     }
        // }
        
        // macro_rules! vcd {
        //     (
        //         $name:ty {
        //             ${}
        //         }

        //     ) => {
                
        //     };
        // } 
        // match self 
        // {
        //     ControllerInstruction::SetAluConfig { alu_addr, alu_config } => 

        //     ,
        //     ControllerInstruction::SetLiteral { register_index, literal } => todo!(),
        //     ControllerInstruction::PopStack { register_index } => todo!(),
        //     ControllerInstruction::PushToStack { register_index } => todo!(),
        //     ControllerInstruction::WaitForActivationSignal { register_index } => todo!(),
        //     ControllerInstruction::Jump { relative, addr } => todo!(),
        //     ControllerInstruction::ResetAll => todo!(),
        //     ControllerInstruction::NoOp => todo!(),
        // }
    }

    fn verilog(self) -> VerilogLiteral {
        let a = self.to_bits::<64>();        
        a.verilog()
    }
}
pub trait TakeBits {
    fn take_bits<const N: usize>(&mut self) -> Bits<N>;
}
impl TakeBits for &[bool]{
    fn take_bits<const N: usize>(&mut self) -> Bits<N> {
        let res_bits = &self[..N];
        *self = &self[N..];
        res_bits.to_bits()
    }
}


impl ToBits for Instruction {
    fn to_bits<const N: usize>(self) -> Bits<N> {
        let inner: Bits<CONTROLLER_INSTRUCTION_SIZE> = 
            match self {
                Instruction::SetAluConfig { alu_addr, alu_config } => {
                    bits::<1>(0b1).to_bit_vec()
                    .cat(alu_addr)
                    .cat(alu_config)
                },
                Instruction::SetLiteral { register_index, literal } => {
                    bits::<4>(0b0000).to_bit_vec()
                    .cat(register_index)
                    .cat(literal)
                },
                
                // ControllerInstruction::PopStack { register_index } => {
                //     bits::<4>(0b0001).to_bit_vec()
                //     .cat(register_index)
                // },
                // ControllerInstruction::PushToStack { register_index } =>
                //     bits::<4>(0b0010).to_bit_vec()
                //     .cat(register_index),

                Instruction::WaitForActivationSignal { register_index } =>
                    bits::<4>(0b0011).to_bit_vec()
                    .cat(register_index),

                Instruction::Jump { relative, addr } => 
                    bits::<4>(0b0100).to_bit_vec()
                    .cat(relative)
                    .cat(addr),

                Instruction::ResetAll => 
                    bits::<4>(0b0101).to_bit_vec(),

                Instruction::NoOp => 
                    bits::<4>(0b0111).to_bit_vec()
            }
            .extend(HorizontalDir::Left, false, 64)
            .to_bits()
            ;

        bit_cast(inner)
    }
}

pub trait ToBitVec: Sized{
    fn to_bit_vec(self) -> BitVec;
}

impl<S: Synth> ToBitVec for S{
    fn to_bit_vec(self) -> BitVec{
        let veri = self.verilog();
        let mut bits = Vec::new();
        for i in 0..veri.bits as u64{
            let bit = veri.val.bit(i);
            bits.push(bit);
        }

        bits.reverse();
        
        BitVec(bits)
    }
}

impl ToBitVec for BitVec{
    #[inline]
    fn to_bit_vec(self) -> BitVec {
        self
    }
}

pub struct BitVec(pub Vec<bool>);

impl BitVec{
    pub fn bits(&self) -> usize{
        self.0.len()
    }
    pub fn cat(mut self, rhs: impl ToBitVec) -> Self{
        self.0.append(&mut rhs.to_bit_vec().0);
        self
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HorizontalDir{
    Left,
    Right
}

impl BitVec{
    pub fn extend(mut self, align: HorizontalDir, fill_value: bool, size: usize) -> Self{
        let mut rest = Vec::from_iter(repeat_n(fill_value, size - self.0.len()));
        match align{
            HorizontalDir::Left =>{
                self.0.append(&mut rest);
                self
            } ,
            HorizontalDir::Right => {
                rest.append(&mut self.0);
                Self(rest)
            },
        }
    }
}

impl ToBits for BitVec{
    fn to_bits<const N: usize>(self) -> Bits<N> {
        let mut res = bits(0);
        for (index, bit) in self.0.iter().enumerate(){
            res = res.replace_bit((self.0.len() - 1) - index, *bit);
        }
        res
    }
}

