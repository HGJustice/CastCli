#[derive(Debug)]
pub enum CliErrors {
    SrcFolderNotFound,
    CouldntReadSrcFolder,
    UnKnownPath,
    VariableEmpty(String),
    VariableNotFound(String),
    PrivateKeyIsEmpty,
    UnknownVariable,
}
