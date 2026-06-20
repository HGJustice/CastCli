
pub struct CommandVariables {
    pub contract_address: String, 
    pub function_name: String, 
    pub function_params: Vec<(String, String)>,
    pub rpc_url: String
}

impl std::fmt::Display for AbiFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.function_name)
    }
}

pub struct AbiFunction {
    pub function_name: String, 
    pub function_params: Vec<(String, String)>
}   