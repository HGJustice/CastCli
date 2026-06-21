use clap::Parser;
use dialoguer::{Input, Select};

use foundry_cli::functions::*;
use std::process::Command;

#[derive(Parser)]
#[command(name = "Wizz", version, about = "Guided Foundry CLI wrapper")]
struct Cli;

fn main() {
    dotenv::dotenv().ok();
    let _cli = Cli::parse();

    loop {
        let options = &["Deploy", "Read", "Write", "Quit"];

        let main_menu = Select::new()
            .with_prompt("Welcome to Foundry Wizard, Pick one of these options")
            .items(options)
            .default(0)
            .interact()
            .unwrap();

        match main_menu {
            0 => {
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

                let deploy_command = Command::new("forge")
                    .args([
                        "create",
                        &format!("src/{}:{}", src_folder[smart_contract], contract_name),
                        "--rpc-url",
                        &rpc_url,
                        "--private-key",
                        &private_key,
                        "--broadcast",
                    ])
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&deploy_command.stdout));
                println!("{}", String::from_utf8_lossy(&deploy_command.stderr));
            }
            1 => {
                let command_varibles = write_or_read(false).unwrap();

                let read_command = Command::new("cast")
                    .args([
                        "call",
                        &command_varibles.contract_address,
                        &format!("{}()", command_varibles.function_name),
                        "--rpc-url",
                        &command_varibles.rpc_url,
                    ])
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&read_command.stdout));
                println!("{}", String::from_utf8_lossy(&read_command.stderr));
            }
            2 => {
                let command_variables = write_or_read(true).unwrap();
                let private_key = read_env_file("PRIVATE_KEY").unwrap();

                let mut param_types: Vec<String> = Vec::new();
                let mut user_values: Vec<String> = Vec::new();

                for (param_name, param_type) in &command_variables.function_params {
                    param_types.push(param_type.clone());
                    let val = Input::new()
                    .with_prompt(format!("Enter value for {} ({})", param_name, param_type))
                    .interact_text()
                    .unwrap();
                    user_values.push(val);
                }

                let signature = format!("{}({})", command_variables.function_name, param_types.join(","));

                let mut command_args = vec![String::from("send"), command_variables.contract_address, signature];
                command_args.extend(user_values);
                command_args.extend(["--rpc-url".to_string(), command_variables.rpc_url, "--private-key".to_string(), private_key,]);

                let write_command = Command::new("cast")
                .args(&command_args)
                .output()
                .expect("Failed to execute command");
                println!("{}", String::from_utf8_lossy(&write_command.stdout));
                println!("{}", String::from_utf8_lossy(&write_command.stderr));
            }
            3 => break,
            _ => unreachable!(),
        }
    }
}
