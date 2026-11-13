use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct SusfsError {
    pub code: &'static str,
    pub message: String,
}

pub fn fail(code: &'static str, message: impl Into<String>) -> anyhow::Error {
    SusfsError {
        code,
        message: message.into(),
    }
    .into()
}

pub fn code_of(error: &anyhow::Error) -> &'static str {
    error
        .downcast_ref::<SusfsError>()
        .map_or("susfs_error", |e| e.code)
}
