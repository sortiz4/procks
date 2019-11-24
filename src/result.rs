use std::result::Result as StdResult;
use super::Error;

pub type Result<T> = StdResult<T, Error>;
