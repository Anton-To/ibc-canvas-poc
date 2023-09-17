use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("cannot upgrade from a newer contract version")]
    BadVersion {},

    #[error("can only upgrade from same contract type")]
    TypeMismatch {},

    #[error("square root of canvas size must be a whole number")]
    BadCanvas {},

    #[error("unknown color")]
    UnknownColor {},

    #[error("pixel out of bounds")]
    PixelOutOfBounds {},

    #[error("only admin can execute this function")]
    Unauthorized {},

    #[error("only unordered channels are supported")]
    OrderedChannel {},

    #[error("invalid IBC channel version. Got ({actual}), expected ({expected})")]
    InvalidVersion { actual: String, expected: String },
}
