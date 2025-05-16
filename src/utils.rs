use std::error::Error;

pub fn error_to_string(error: impl Error) -> String {
    error.to_string()
}
