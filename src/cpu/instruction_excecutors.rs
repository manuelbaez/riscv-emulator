use crate::{
    error::{AppErrors, AppResult},
    memory::MemoryOpSize,
};

use super::{
    instructions::{
        decoder::{
            self,
            b32::{
                Funct3Decoder, Funct7Decoder, ITypeDecoder, InstructionFormat,
                Instrunction32Decoder, Rs1Decoder, Rs2Decoder, STypeDecoder,
            },
            InstructionSize,
        },
        implementations::{CpuInstructionsOpCodes, SubFunctions},
    },
    side_effects::OperationSideEffect,
    Cpu,
};
impl Cpu {
    fn exec_32bit_instruction(&mut self, instruction: u32) -> AppResult<OperationSideEffect> {
        let decoder = Instrunction32Decoder::new(instruction);
        match decoder::get_op_code(instruction) {
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE => {
                match decoder.get_funct3_field() {
                    SubFunctions::ADDI => InstructionsExecutor::addi(self, decoder),
                    SubFunctions::SLTI => InstructionsExecutor::slti(self, decoder),
                    SubFunctions::SLTIU => InstructionsExecutor::sltiu(self, decoder),
                    SubFunctions::ORI => InstructionsExecutor::ori(self, decoder),
                    SubFunctions::XORI => InstructionsExecutor::xori(self, decoder),
                    SubFunctions::ANDI => InstructionsExecutor::andi(self, decoder),
                    SubFunctions::SLLI => InstructionsExecutor::slli(self, decoder),
                    SubFunctions::SRLI_SRAI_F3 => {
                        let variant =
                            (decoder.get_imm_field(InstructionFormat::I) & 0x0fc0_u64) as u8; //Filter top 6 bits to match RV64I variants
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
                InstructionsExecutor::lui(self, decoder)
            }
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE_AUIPC => {
                InstructionsExecutor::auipc(self, decoder)
            }
            CpuInstructionsOpCodes::INT_REG_REG_RV32I => {
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
            CpuInstructionsOpCodes::LOAD => InstructionsExecutor::load(self, decoder),
            CpuInstructionsOpCodes::STORE => {
                let addr: u64 = self.registers[decoder.get_rs1_field() as usize]
                    .wrapping_add(decoder.get_imm_field(InstructionFormat::S));
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
            CpuInstructionsOpCodes::CONTROL_JAL => InstructionsExecutor::jal(self, decoder),
            CpuInstructionsOpCodes::CONTROL_JALR => InstructionsExecutor::jalr(self, decoder),
            CpuInstructionsOpCodes::CONDITIONAL_BRANCHES => match decoder.get_funct3_field() {
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
            },
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE_RV64I => {
                let funct7 = if decoder.get_funct3_field() == SubFunctions::ADDIW.0 {
                    0x00
                } else {
                    decoder.get_funct7_field()
                };
                match (decoder.get_funct3_field(), funct7) {
                    SubFunctions::ADDIW => InstructionsExecutor::addiw(self, decoder),
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3_field(),
                        Some(decoder.get_funct7_field()),
                    )),
                }
            }
            CpuInstructionsOpCodes::INT_REG_REG_RV64I => {
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
                match decoder.get_funct3_field() {
                    SubFunctions::FENCE => {
                        //Not necesary for the moment being,since
                        //this is an in-order execution emulator
                        Ok(OperationSideEffect::None)
                    }
                    _ => Err(AppErrors::InstructionNotImplemented { instruction }),
                }
            }
            CpuInstructionsOpCodes::SYSCALLS_CSR => {
                let decoder = Instrunction32Decoder::new(instruction);
                match decoder.get_funct3_field() {
                    0x00 => match decoder.get_imm_field(InstructionFormat::I) as u16 {
                        SubFunctions::EBREAK => Ok(OperationSideEffect::TriggerBreakpoint),
                        SubFunctions::ECALL => Ok(OperationSideEffect::TriggerSyscall),
                        _ => Err(AppErrors::InstructionNotImplemented { instruction }),
                    },
                    SubFunctions::CSRRW => {
                        Err(AppErrors::InstructionNotImplemented { instruction })
                    }
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
