use clap::Parser;
use dialoguer::{Select, Input};

use std::process::Command;
use foundry_cli::functions::*;

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
        let options = &["Deploy", "Read", "Write", "Verify", "Quit"];

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

                let deploy_command = Command::new("forge").args(["create", &format!("src/{}:{}", src_folder[smart_contract], contract_name),"--rpc-url", &rpc_url,"--private-key", &private_key, "--broadcast"])
                .output().expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&deploy_command.stdout));
                println!("{}", String::from_utf8_lossy(&deploy_command.stderr));
                break;
            },
            1 => {
                println!("Read selected");
                
                let contract_address: String = Input::new()
                .with_prompt("Enter the contract address")
                .interact_text()
                .unwrap();
             
                let contract_name: String = Input::new()
                .with_prompt("Enter the contract name")
                .interact_text()
                .unwrap();
                
                let functions_list = read_abi(contract_name).unwrap();

                let read_function = Select::new()
                .with_prompt("Which read function do you want to use?")
                .items(&functions_list)
                .default(0)
                .interact()
                .unwrap();

                let rpc_url = read_env_file("CHAIN_RPC_URL").unwrap();
                
                let read_command = Command::new("cast").args(["call", &contract_address, &format!("{}()", functions_list[read_function]),"--rpc-url", &rpc_url])
                .output().expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&read_command.stdout));
                println!("{}", String::from_utf8_lossy(&read_command.stderr));
                break;  
            },
            2 => {
                println!("Write selected");
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