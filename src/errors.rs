
#[derive(Debug)]

pub enum CliErrors {
    SrcFoulderNotFound,
    CouldntReadSrcFoulder,
    CantFindChainRpcUrlVariable,
    RPCUrlIsEmpty,
    CantFindPrivateKeyVariable,
    PrivateKeyIsEmpty,
    UnknownVariable,
}