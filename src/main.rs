
mod vault;
mod key_derivation;
mod vault_header;

use vault::Vault;
use std::{io::{Write, stdin, stdout}, process::exit};

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
        vault = Vault::new(master_password).expect("Error when try to create vault");
        println!("Vault file does not exist");
    }

    loop {
        println!("1. Ajouter un mot de passe");
        println!("2. Lister les services");
        println!("3. Voir une entrée");
        println!("4. Quitter");

        let choice = &mut String::new();
        stdin.read_line(choice).expect("error");

        let choice_number: u8 = choice.trim().parse().expect("Input not an integer");
        match choice_number {
            1 => {},
            2 => {},
            3 => {},
            4 => exit(0),
            _ => {},
        }
    }
}
