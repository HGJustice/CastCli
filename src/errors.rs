#[derive(Debug)]
pub enum CliErrors {
    SrcFoulderNotFound,
    CouldntReadSrcFoulder,
    UnKnownPath,
    CantFindChainRpcUrlVariable,
    RPCUrlIsEmpty,
    CantFindPrivateKeyVariable,
    PrivateKeyIsEmpty,
    UnknownVariable,
}
