#[derive(Debug)]
pub enum CliErrors {
    SrcFolderNotFound,
    CouldntReadSrcFolder,
    UnKnownPath,
    CantFindChainRpcUrlVariable,
    RPCUrlIsEmpty,
    CantFindPrivateKeyVariable,
    PrivateKeyIsEmpty,
    UnknownVariable,
}
