use clap::Parser;
use dialoguer::Select;

#[derive(Parser)]
#[command(name = "foundry-wizard", version, about = "Guided Foundry CLI wrapper")]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    let _cli = Cli::parse();

    loop {
        let options = &["Deploy", "Write", "Read", "Verify", "Quit"];

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => println!("Cast selected"),
            1 => println!("Deploy selected"),
            2 => println!("Verify selected"),
            3 => break,
            _ => unreachable!(),
        }
    }
}