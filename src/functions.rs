use serde_json::Value;
use dialoguer::{Input, Select};

use std::fs;
use crate::types::*;
use crate::errors::CliErrors;

pub fn search_dir() -> Result<Vec<String>, CliErrors> {

    let mut files:Vec<String> = Vec::new();

    if !fs::exists("src/").unwrap() { return Err(CliErrors::SrcFoulderNotFound) }

    for entries in fs::read_dir("src/").unwrap() {
        let entry = entries.unwrap();
        files.push(entry.file_name().to_string_lossy().to_string());
        
    }
    return Ok(files);
}

pub fn read_env_file(varible_name: &str) -> Result<String, CliErrors> {
    match varible_name {
        "CHAIN_RPC_URL" => {
            let var =  std::env::var(varible_name).map_err(|_| CliErrors::CantFindChainRpcUrlVariable)?;
            if var.is_empty() {
                Err(CliErrors::RPCUrlIsEmpty)
            } else {
                Ok(var)
            }
        },
        "PRIVATE_KEY" => {
            let var =  std::env::var(varible_name).map_err(|_| CliErrors::CantFindPrivateKeyVariable)?;
            if var.is_empty() {
                Err(CliErrors::PrivateKeyIsEmpty)
            } else {
                Ok(var)
            }
        }
        _ => Err(CliErrors::UnknownVariable)
    }
}

pub fn read_abi(contract_name: String, write: bool) -> Result<Vec<AbiFunction>, CliErrors> {
    let mut functions:Vec<AbiFunction> = Vec::new();

    let file_path = format!("out/{}.sol/{}.json", contract_name, contract_name);
    let file_content = fs::read_to_string(file_path).unwrap();
    let json: Value = serde_json::from_str(&file_content).unwrap();

    let abi = json["abi"].as_array().unwrap();

    for item in abi {
        
        if item["type"] == "function" {
            let mut params: Vec<(String, String)> = Vec::new();

            let name = item["name"].as_str().unwrap();
            let mutability = item["stateMutability"].as_str().unwrap();
            let inputs = item["inputs"].as_array().unwrap();

            for input in inputs {
                let param_name = input["name"].as_str().unwrap().to_string();
                let param_type = input["type"].as_str().unwrap().to_string();
                params.push((param_name, param_type));
            }
       
            let filter_1 = if write { "nonpayable" } else { "view" };
            let filter_2 = if write { "payable" } else { "pure" };

            if mutability == filter_1 || mutability == filter_2 {
                let function = AbiFunction { function_name: name.to_string(), function_params: params};
                functions.push(function);
            }
        }
    }
    return Ok(functions)
}

pub fn write_or_read(write: bool) -> Result<CommandVariables, CliErrors> {
        let contract_address: String = Input::new()
        .with_prompt("Enter the contract address")
        .interact_text()
        .unwrap();
        
        let contract_name: String = Input::new()
        .with_prompt("Enter the contract name")
        .interact_text()
        .unwrap();
        
        let functions_list = read_abi(contract_name, write).unwrap();

        let write_function = Select::new()
        .with_prompt("Which write function do you want to use?")
        .items(&functions_list)
        .default(0)
        .interact()
        .unwrap();

        let rpc_url = read_env_file("CHAIN_RPC_URL").unwrap();

        Ok(CommandVariables {
            contract_address: contract_address, 
            function_name: functions_list[write_function].function_name.clone(),
            function_params: functions_list.
            rpc_url: rpc_url
        })
}