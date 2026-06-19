use clap::Parser;
use dialoguer::{Select, Input};
use foundry_cli::errors::CliErrors::{self, RPCUrlIsEmpty};

use std::fs;
use std::process::Command;


#[derive(Parser)]
#[command(name = "foundry-wizard", version, about = "Guided Foundry CLI wrapper")]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    dotenv::dotenv().ok();
    let _cli = Cli::parse();

    loop {
        let options = &["Deploy", "Write", "Read", "Verify", "Quit"];

        let main_menu = Select::new()
            .with_prompt("Welcome to Foundry Wizard, Pick one of these options")
            .items(options)
            .default(0)
            .interact()
            .unwrap();

        match main_menu {
            0 => {
                println!("Deploy selected");
                
                let src_folder = search_dir().unwrap();
              
                let smart_contract = Select::new()
                .with_prompt("Please choose which contract to deploy")
                .items(&src_folder)
                .default(0)
                .interact()
                .unwrap();

                let contract_name: String = Input::new()
                .with_prompt("Enter the contract name")
                .interact_text()
                .unwrap();

                let rpc_url = read_env_file("CHAIN_RPC_URL").unwrap();
                let private_key = read_env_file("PRIVATE_KEY").unwrap();

                // coonsutructor arguments for contract

                // build and run command using foundrys tools
                let deploy_command = Command::new("forge").args(["create", &format!("src/{}:{}", src_folder[smart_contract], contract_name),"--rpc-url", &rpc_url,"--private-key", &private_key, "--broadcast"])
                 .output().expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&deploy_command.stdout));
                println!("{}", String::from_utf8_lossy(&deploy_command.stderr));
                break;
            },
            1 => {
                println!("Write selected");
                break;  
            },
            2 => {
                println!("Read selected");
                break;
            }
            3 => {
                println!("Verify selected");
                break;
            },
            4 => break,
            _ => unreachable!(),
        }
    }
}


fn search_dir() -> Result<Vec<String>, CliErrors> {

    let mut contracts:Vec<String> = Vec::new();

    if !fs::exists("src/").unwrap() {
        return Err(CliErrors::SrcFoulderNotFound)
    }

    for entries in fs::read_dir("src/").unwrap() {
        let entry = entries.unwrap();
        contracts.push(entry.file_name().to_string_lossy().to_string());
        
    }
    return Ok(contracts);
}

fn read_env_file(varible_name: &str) -> Result<String, CliErrors> {
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