use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("invalid wasm binary: {0}")]
    InvalidWasmError(String),

    #[error("invalid parameters: {0}")]
    InvalidParameters(String),
}
