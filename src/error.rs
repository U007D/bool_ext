use crate::consts::msg;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}", msg::SAMPLE_ERROR)]
    SampleError,
}
