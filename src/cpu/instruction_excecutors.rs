use crate::{
    error::{AppErrors, AppResult},
    memory::MemoryOpSize,
};

use super::{
    instructions::{
        decoder::{
            self, BTypeDecoder, Funct3Decoder, Funct7Decoder, ITypeDecoder, JTypeDecoder,
            RTypeDecoder, Rs1Decoder, Rs2Decoder, STypeDecoder, UTypeDecoder,
        },
        implementations::{CpuInstructionsOpCodes, SubFunctions},
    },
    Cpu,
};
impl Cpu {
    pub fn execute(&mut self, instruction: u32) -> AppResult<()> {
        match decoder::get_op_code(instruction) {
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE => {
                let decoder = ITypeDecoder::new(instruction);
                match decoder.get_funct3() {
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
                                decoder.get_funct3(),
                                Some(variant),
                            )),
                        }
                    }
                    _ => Err(AppErrors::FuctionNotImplemented(decoder.get_funct3(), None)),
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
                match (decoder.get_funct3(), decoder.get_funct7()) {
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
                        decoder.get_funct3(),
                        Some(decoder.get_funct7()),
                    )),
                }
            }
            CpuInstructionsOpCodes::LOAD => InstructionsExecutor::load(self, instruction),
            CpuInstructionsOpCodes::STORE => {
                let decoder = STypeDecoder::new(instruction);
                let addr: u64 =
                    self.registers[decoder.get_rs1() as usize].wrapping_add(decoder.get_imm());
                match decoder.get_funct3() {
                    SubFunctions::SB => self.system_bus.store(
                        addr,
                        MemoryOpSize::B8,
                        self.registers[decoder.get_rs2() as usize],
                    ),
                    SubFunctions::SH => self.system_bus.store(
                        addr,
                        MemoryOpSize::B16,
                        self.registers[decoder.get_rs2() as usize],
                    ),
                    SubFunctions::SW => self.system_bus.store(
                        addr,
                        MemoryOpSize::B32,
                        self.registers[decoder.get_rs2() as usize],
                    ),
                    SubFunctions::SD => self.system_bus.store(
                        addr,
                        MemoryOpSize::B64,
                        self.registers[decoder.get_rs2() as usize],
                    ),
                    _ => Err(AppErrors::FuctionNotImplemented(decoder.get_funct3(), None)),
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
                match decoder.get_funct3() {
                    SubFunctions::BEQ => InstructionsExecutor::beq(self, decoder),
                    SubFunctions::BNE => InstructionsExecutor::bne(self, decoder),
                    SubFunctions::BLT => InstructionsExecutor::blt(self, decoder),
                    SubFunctions::BLTU => InstructionsExecutor::bltu(self, decoder),
                    SubFunctions::BGE => InstructionsExecutor::bge(self, decoder),
                    SubFunctions::BGEU => InstructionsExecutor::bgeu(self, decoder),
                    _ => Err(AppErrors::FuctionNotImplemented(decoder.get_funct3(), None)),
                }
            }
            CpuInstructionsOpCodes::INT_REG_IMMEDIATE_RV64I => {
                let decoder = RTypeDecoder::new(instruction);
                let funct7 = if decoder.get_funct3() == SubFunctions::ADDIW.0 {
                    0x00
                } else {
                    decoder.get_funct7()
                };
                match (decoder.get_funct3(), funct7) {
                    SubFunctions::ADDIW => {
                        InstructionsExecutor::addiw(self, ITypeDecoder::new(instruction))
                    }
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3(),
                        Some(decoder.get_funct7()),
                    )),
                }
            }
            CpuInstructionsOpCodes::INT_REG_REG_RV64I => {
                let decoder = RTypeDecoder::new(instruction);
                match (decoder.get_funct3(), decoder.get_funct7()) {
                    SubFunctions::ADDW => InstructionsExecutor::addw(self, decoder),
                    SubFunctions::SUBW => InstructionsExecutor::subw(self, decoder),
                    _ => Err(AppErrors::FuctionNotImplemented(
                        decoder.get_funct3(),
                        Some(decoder.get_funct7()),
                    )),
                }
            }
            _ => Err(AppErrors::InstructionNotImplemented { instruction }),
        }
    }
}

pub struct InstructionsExecutor;
