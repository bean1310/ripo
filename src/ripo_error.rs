use thiserror::Error;

#[derive(Error, Debug)]
pub enum RipoError {
    #[error("Invalid File")] 
    InvalidFileError,
}