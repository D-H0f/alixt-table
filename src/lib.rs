pub mod table;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum TableError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Table Input Error: {0}")]
    InputError(String),
}
