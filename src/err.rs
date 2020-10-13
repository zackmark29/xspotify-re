use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BadInputError;

impl fmt::Display for BadInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bad input. only tracks are accepted right now")
    }
}

impl error::Error for BadInputError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct BadResponseError {
    pub code: u16,
    pub response: String,
}

impl fmt::Display for BadResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "server responded with error code {}.\nResponse: {}",
            self.code, self.response
        )
    }
}

impl error::Error for BadResponseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}