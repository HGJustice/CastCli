use serde_json::Value;

use std::fs;
use crate::errors::CliErrors;

pub fn search_dir() -> Result<Vec<String>, CliErrors> {

    let mut files:Vec<String> = Vec::new();

    if !fs::exists("src/").unwrap() {
        return Err(CliErrors::SrcFoulderNotFound)
    }

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

pub fn read_abi(contract_name: String, write: bool) -> Result<Vec<String>, CliErrors> {
    let mut functions:Vec<String> = Vec::new();

    let file_path = format!("out/{}.sol/{}.json", contract_name, contract_name);
    let file_content = fs::read_to_string(file_path).unwrap();
    let json: Value = serde_json::from_str(&file_content).unwrap();

    let abi = json["abi"].as_array().unwrap();

    for item in abi {
        // Only grab functions, not events or constructors
        if item["type"] == "function" {
            let name = item["name"].as_str().unwrap();
            let mutability = item["stateMutability"].as_str().unwrap();
            
            // For reads: "view" or "pure"
            // For writes: "nonpayable" or "payable"
            let filter_1 = if write { "nonpayable" } else { "view" };
            let filter_2 = if write { "payable" } else { "pure" };

            if mutability == filter_1 || mutability == filter_2 {
                functions.push(name.to_string());
            }
        }
    }

    return Ok(functions)
}