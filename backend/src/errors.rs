

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EnvVarNotFound(&'static str),
    // DatabaseError(String),
    // DatabasePoolError(String),
    // DatabasePoolTimeoutError(String),
    // DatabaseUrlError(String),
    // DatabaseConnectionError(String),
    // DatabaseQueryError(String),
    // DatabaseTransactionError(String),
    // DatabaseMigrat
}