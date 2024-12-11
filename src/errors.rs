use std::error;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ErrorCategory {
    InputData,
    InputDataFormat,
}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    inner_error: Box<InnerError>,
}

impl Error {
    pub fn categorize(&self) -> ErrorCategory {
        match self.inner_error.code {
            ErrorCode::InvalidArgument => ErrorCategory::InputData,
            ErrorCode::InvalidArgumentFormat => ErrorCategory::InputDataFormat,
        }
    }
}

#[allow(dead_code)]
pub(crate) enum ErrorCode {
    InvalidArgument,
    InvalidArgumentFormat,
}

impl ErrorCode {
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::InvalidArgument => "Invalid value for argument",
            ErrorCode::InvalidArgumentFormat => "Invalid argument format.",
        }
    }
}

struct InnerError {
    code: ErrorCode,
    target: String,
}

impl Error {
    pub(crate) fn new(code: ErrorCode, target: impl Into<String>) -> Self {
        Self {
            inner_error: Box::new(InnerError {
                code: code,
                target: target.into(),
            }),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  target {}",
            self.inner_error.code.description(),
            self.inner_error.target
        )
    }
}
