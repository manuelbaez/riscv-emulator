use std::result;

use crate::{
    error::{AppErrors, AppResult},
    memory::MemoryOpSize,
};

use super::{
    instructions::{
        decoder::{
            self,
            b32::{Funct3Decoder, Funct7Decoder, Rs1Decoder, Rs2Decoder},
            BTypeDecoder, ITypeDecoder, InstructionSize, JTypeDecoder, RTypeDecoder, STypeDecoder,
            UTypeDecoder,
        },
        implementations::{CpuInstructionsOpCodes, SubFunctions},
    },
    side_effects::OperationSideEffect,
    Cpu,
};
impl Cpu {
    fn exec_32bit_instruction(&mut self, instruction: u32) -> AppResult<OperationSideEffect> {
        match decoder::get_op_code(instruction) {
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE => {
                let decoder = ITypeDecoder::new(instruction);
                match decoder.get_funct3_field() {
                    SubFunctions::ADDI => InstructionsExecutor::addi(self, decoder),
                    SubFunctions::SLTI => InstructionsExecutor::slti(self, decoder),
                    SubFunctions::SLTIU => InstructionsExecutor::sltiu(self, decoder),
                    SubFunctions::ORI => InstructionsExecutor::ori(self, decoder),
                    SubFunctions::XORI => InstructionsExecutor::xori(self, decoder),
                    SubFunctions::ANDI => InstructionsExecutor::andi(self, decoder),
                    SubFunctions::SLLI => InstructionsExecutor::slli(self, decoder),
                    SubFunctions::SRLI_SRAI_F3 => {
                        let variant = (decoder.get_imm() & 0x0fc0_u64) as u8; //Filter top 6 bits to match RV64I variants
                        match (SubFunctions::SRLI_SRAI_F3, variant) {
                            SubFunctions::SRLI => InstructionsExecutor::srli(self, decoder),
                            SubFunctions::SRAI => InstructionsExecutor::srai(self, decoder),
                            _ => Err(AppErrors::FuctionNotImplemented(
                                decoder.get_funct3_field(),
                                Some(variant),
                            )),
                        }
                    }
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3_field(),
                        None,
                    )),
                }
            }
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE_LUI => {
                InstructionsExecutor::lui(self, UTypeDecoder::new(instruction))
            }
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE_AUIPC => {
                InstructionsExecutor::auipc(self, UTypeDecoder::new(instruction))
            }
            CpuInstructionsOpCodes::INT_REG_REG_RV32I => {
                let decoder = RTypeDecoder::new(instruction);
                match (decoder.get_funct3_field(), decoder.get_funct7_field()) {
                    SubFunctions::ADD => InstructionsExecutor::add(self, decoder),
                    SubFunctions::SUB => InstructionsExecutor::sub(self, decoder),
                    SubFunctions::SLT => InstructionsExecutor::slt(self, decoder),
                    SubFunctions::SLTU => InstructionsExecutor::sltu(self, decoder),
                    SubFunctions::AND => InstructionsExecutor::and(self, decoder),
                    SubFunctions::OR => InstructionsExecutor::or(self, decoder),
                    SubFunctions::XOR => InstructionsExecutor::xor(self, decoder),
                    SubFunctions::SLL => InstructionsExecutor::sll(self, decoder),
                    SubFunctions::SRL => InstructionsExecutor::srl(self, decoder),
                    SubFunctions::SRA => InstructionsExecutor::sra(self, decoder),
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3_field(),
                        Some(decoder.get_funct7_field()),
                    )),
                }
            }
            CpuInstructionsOpCodes::LOAD => InstructionsExecutor::load(self, instruction),
            CpuInstructionsOpCodes::STORE => {
                let decoder = STypeDecoder::new(instruction);
                let addr: u64 = self.registers[decoder.get_rs1_field() as usize]
                    .wrapping_add(decoder.get_imm());
                match decoder.get_funct3_field() {
                    SubFunctions::SB => self
                        .system_bus
                        .store(
                            addr,
                            MemoryOpSize::B8,
                            self.registers[decoder.get_rs2_field() as usize],
                        )
                        .map(|_| OperationSideEffect::None),
                    SubFunctions::SH => self
                        .system_bus
                        .store(
                            addr,
                            MemoryOpSize::B16,
                            self.registers[decoder.get_rs2_field() as usize],
                        )
                        .map(|_| OperationSideEffect::None),
                    SubFunctions::SW => self
                        .system_bus
                        .store(
                            addr,
                            MemoryOpSize::B32,
                            self.registers[decoder.get_rs2_field() as usize],
                        )
                        .map(|_| OperationSideEffect::None),
                    SubFunctions::SD => self
                        .system_bus
                        .store(
                            addr,
                            MemoryOpSize::B64,
                            self.registers[decoder.get_rs2_field() as usize],
                        )
                        .map(|_| OperationSideEffect::None),
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3_field(),
                        None,
                    )),
                }
            }
            CpuInstructionsOpCodes::CONTROL_JAL => {
                InstructionsExecutor::jal(self, JTypeDecoder::new(instruction))
            }
            CpuInstructionsOpCodes::CONTROL_JALR => {
                InstructionsExecutor::jalr(self, ITypeDecoder::new(instruction))
            }
            CpuInstructionsOpCodes::CONDITIONAL_BRANCHES => {
                let decoder = BTypeDecoder::new(instruction);
                match decoder.get_funct3_field() {
                    SubFunctions::BEQ => InstructionsExecutor::beq(self, decoder),
                    SubFunctions::BNE => InstructionsExecutor::bne(self, decoder),
                    SubFunctions::BLT => InstructionsExecutor::blt(self, decoder),
                    SubFunctions::BLTU => InstructionsExecutor::bltu(self, decoder),
                    SubFunctions::BGE => InstructionsExecutor::bge(self, decoder),
                    SubFunctions::BGEU => InstructionsExecutor::bgeu(self, decoder),
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3_field(),
                        None,
                    )),
                }
            }
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE_RV64I => {
                let decoder = RTypeDecoder::new(instruction);
                let funct7 = if decoder.get_funct3_field() == SubFunctions::ADDIW.0 {
                    0x00
                } else {
                    decoder.get_funct7_field()
                };
                match (decoder.get_funct3_field(), funct7) {
                    SubFunctions::ADDIW => {
                        InstructionsExecutor::addiw(self, ITypeDecoder::new(instruction))
                    }
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3_field(),
                        Some(decoder.get_funct7_field()),
                    )),
                }
            }
            CpuInstructionsOpCodes::INT_REG_REG_RV64I => {
                let decoder = RTypeDecoder::new(instruction);
                match (decoder.get_funct3_field(), decoder.get_funct7_field()) {
                    SubFunctions::ADDW => InstructionsExecutor::addw(self, decoder),
                    SubFunctions::SUBW => InstructionsExecutor::subw(self, decoder),
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3_field(),
                        Some(decoder.get_funct7_field()),
                    )),
                }
            }
            CpuInstructionsOpCodes::MEM_ORDERING => {
                let decoder = ITypeDecoder::new(instruction);
                match decoder.get_funct3_field() {
                    SubFunctions::FENCE => {
                        //Not necesary for the moment being,since
                        //this is an in-order execution emulator
                        Ok(OperationSideEffect::None)
                    }
                    _ => Err(AppErrors::InstructionNotImplemented { instruction }),
                }
            }
            CpuInstructionsOpCodes::SYSCALLS => {
                let decoder = ITypeDecoder::new(instruction);
                match decoder.get_imm() as u16 {
                    SubFunctions::EBREAK => Ok(OperationSideEffect::TriggerBreakpoint),
                    SubFunctions::ECALL => Ok(OperationSideEffect::TriggerSyscall),
                    _ => Err(AppErrors::InstructionNotImplemented { instruction }),
                }
            }
            _ => Err(AppErrors::InstructionNotImplemented { instruction }),
        }
    }
    pub fn execute(&mut self, instruction: u32) -> AppResult<OperationSideEffect> {
        let op_code = decoder::get_op_code(instruction);
        let instruction_size = decoder::get_instruction_size(op_code)?;
        let exec_result = match instruction_size {
            InstructionSize::B16 => {
                let instruction = instruction & 0xffff;
                Err(AppErrors::InstructionNotImplemented {
                    instruction: instruction,
                })
            }
            InstructionSize::B32 => self.exec_32bit_instruction(instruction),
        };

        exec_result.map(|result| match result {
            OperationSideEffect::SkipPCIncrease => OperationSideEffect::None,
            _ => {
                self.increase_program_counter(instruction_size);
                result
            }
        })
    }
}

pub struct InstructionsExecutor;
