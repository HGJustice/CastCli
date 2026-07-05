use clap::Parser;
use dialoguer::{Input, Select};

use foundry_cli::functions::*;
use std::process::Command;

#[derive(Parser)]
#[command(name = "Wizz", version = "0.0.1", about = "Guided Foundry CLI wrapper")]
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
                let command_variables = write_or_read(false).unwrap();
                let (param_types, user_values) = get_func_name_param(&command_variables).unwrap();

                let signature = format!("{}({})({})", command_variables.function.function_name, param_types.join(","), command_variables.function.return_types.join(","));

                let mut command_args = vec![String::from("call"), command_variables.contract_address, signature];
                command_args.extend(user_values);
                command_args.extend(["--rpc-url".to_string(), command_variables.rpc_url,]);

                let read_command = Command::new("cast")
                .args(&command_args)
                .output()
                .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&read_command.stdout));
                println!("{}", String::from_utf8_lossy(&read_command.stderr));
            }
            2 => {
                let command_variables = write_or_read(true).unwrap();
                let private_key = read_env_file("PRIVATE_KEY").unwrap();

                let (param_types, user_values) = get_func_name_param(&command_variables).unwrap();

                let signature = format!("{}({})", command_variables.function.function_name, param_types.join(","));

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
