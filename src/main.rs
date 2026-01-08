
mod vault;
mod key_derivation;
mod vault_header;

use vault::Vault;
use std::io::{Write, stdin, stdout};

fn main() {
    print!("Enter password : ");
    stdout().flush().expect("error");
    let stdin = stdin();
    let master_password = &mut String::new();
    
    stdin.read_line(master_password).expect("error");


    let mut vault: Vault;
    if Vault::file_exists() {
        println!("Vault file exists");
        vault = Vault::open_existing(master_password).expect("Error when try to open vault");
    } else {
        println!("Vault file does not exist");
    }
}
