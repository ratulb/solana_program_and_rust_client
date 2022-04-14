use client::client::Client;
use client::errors::Result;
use std::process;

fn main() -> Result<()> {
    let client = Client::default();

    if let Err(err) = client.setup_counter_account() {
        eprintln!("Error while setting counter account {}", err);
        process::exit(1);
    }

    if let Err(err) = client.check_program() {
        eprintln!("Program check failed {}", err);
        process::exit(1);
    }

    if let Err(err) = client.increament_counter() {
        eprintln!("Error while increamenting counter {}", err);
        process::exit(1);
    }

    if let Err(err) = client.get_counter_reading() {
        eprintln!("Error while getting  counter reading {}", err);
        process::exit(1);
    }

    Ok(())
}
