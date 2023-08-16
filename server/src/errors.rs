use actix_web::{error, Result};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CLiError {
    #[display(fmt = "Generic Error")]
    GenericError = 1,
    #[display(fmt = "Error: not implemented")]
    NotImplementedError = 2,
    #[display(fmt = "binary not found")]
    BinaryNotFound = 3,
}

impl actix_web::error::ResponseError for CLiError {}


// #[derive(Debug, Error)]
// pub struct ErrorResponse  {
//     code: CLiError,
//     desc: String,
// }