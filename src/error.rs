#[derive(Debug)]
pub(crate) enum ExtError {
    JsonParsingError(String),
    FileNotFound(String),
    EnvironmentVarError(String),
    DirectoryNotFound(String),
    Unknown(String),
}

pub(crate) type Result<T> = std::result::Result<T, ExtError>;

pub(crate) fn convert_result<T, E, F>(result: std::result::Result<T, E>, error: F) -> Result<T>
where
    F: Fn(String) -> ExtError,
    E: std::fmt::Debug,
{
    match result {
        Ok(t) => Ok(t),
        Err(y) => Err(error(format!("{:#?}", y))),
    }
}
