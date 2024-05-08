use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("Cannot write to register")]
    RegisterWriteProhibited,
    #[error("Cannot access register wich is out of bounds")]
    OutOfBoundRegister,
    #[error("Cannot access specified address")]
    AddressNotFound,
    #[error("Cannot memmory thats out of bounds")]
    OutOfBoundsPointer,
    #[error("Instruction is not supported yet")]
    InstructionNotImplemented,
    #[error("Instruction function is not supported yet")]
    FuctionNotImplemented,
    #[error("unknown error ocurred")]
    Unknown,
}

pub type AppResult<T, E = AppErrors> = std::result::Result<T, E>;