mod key_derivation;
mod vault;
mod vault_entry;
mod vault_header;

use std::{
    io::{Write, stdin, stdout},
    process::exit,
};
use vault::Vault;

fn main() {
    print!("Enter password : ");
    stdout().flush().expect("error");
    let stdin = stdin();
    let master_password = &mut String::new();

    stdin.read_line(master_password).expect("error");

    let mut vault: Vault;
    if Vault::file_exists() {
        vault = Vault::open_existing(master_password).expect("Error when try to open vault");
    } else {
        vault = Vault::new(master_password).expect("Error when try to create vault");
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
            1 => {
                let service = &mut String::new();
                let username = &mut String::new();
                let password = &mut String::new();

                print!("Service : ");
                stdout().flush().expect("error");
                stdin.read_line(service).expect("error");

                print!("Username (optional) : ");
                stdout().flush().expect("error");
                stdin.read_line(username).expect("error");

                print!("Password : ");
                stdout().flush().expect("error");
                stdin.read_line(password).expect("error");

                let entry = vault_entry::VaultEntry {
                    service: service.trim().to_owned(),
                    username: if username.trim().is_empty() {
                        None
                    } else {
                        Some(username.trim().to_owned())
                    },
                    password: password.trim().as_bytes().to_vec(),
                };

                vault.add_entry(entry);

                println!("Entry added successfully!");
            }
            2 => {
                println!("Services :");
                for entry in vault.get_entries() {
                    println!("- {}", entry.service);
                }
            }
            3 => {
                println!("TODO: View an entry");
            }
            4 => exit(0),
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
