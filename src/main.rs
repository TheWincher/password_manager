// mod key_derivation;
// mod vault;
// mod vault_entry;
// mod vault_header;

// use std::{
//     io::{Write, stdin, stdout},
//     process::exit,
// };
// use vault::Vault;

// fn main() {
//     print!("Enter password : ");
//     stdout().flush().expect("error");
//     let stdin = stdin();
//     let master_password = &mut String::new();

//     stdin.read_line(master_password).expect("error");

//     let mut vault: Vault;
//     if Vault::file_exists() {
//         vault = Vault::open_existing(master_password).expect("Error when try to open vault");
//     } else {
//         vault = Vault::new(master_password).expect("Error when try to create vault");
//     }

//     loop {
//         println!("1. Ajouter un mot de passe");
//         println!("2. Lister les services");
//         println!("3. Voir une entrée");
//         println!("4. Quitter");
//         print!("Entrez votre choix: ");
//         stdout().flush().expect("error");

//         let choice = &mut String::new();
//         stdin.read_line(choice).expect("error");

//         let choice_number: u8 = choice.trim().parse().expect("Input not an integer");
//         match choice_number {
//             1 => {
//                 let service = &mut String::new();
//                 let username = &mut String::new();
//                 let password = &mut String::new();

//                 print!("Service : ");
//                 stdout().flush().expect("error");
//                 stdin.read_line(service).expect("error");

//                 print!("Username (optional) : ");
//                 stdout().flush().expect("error");
//                 stdin.read_line(username).expect("error");

//                 print!("Password : ");
//                 stdout().flush().expect("error");
//                 stdin.read_line(password).expect("error");

//                 let entry = vault_entry::VaultEntry {
//                     service: service.trim().to_owned(),
//                     username: if username.trim().is_empty() {
//                         None
//                     } else {
//                         Some(username.trim().to_owned())
//                     },
//                     password: password.trim().as_bytes().to_vec(),
//                 };

//                 vault.add_entry(entry);

//                 println!("Entry added successfully!");
//             }
//             2 => {
//                 println!("Services :");
//                 for (i, entry) in vault.get_entries().iter().enumerate() {
//                     println!("{}- {}", i, entry.service);
//                 }
//             }
//             3 => {
//                 print!("Entrez le numéro souhaité : ");
//                 stdout().flush().expect("err");

//                 let entry_number_input = &mut String::new(); 
//                 stdin.read_line(entry_number_input).expect("err");
//                 let entry_number: usize = entry_number_input.trim().parse().expect("Input not an integer");

//                 let entry = vault.get_entry(entry_number);
//                 if entry.is_none() {
//                     println!("Entry not found");
//                     continue;
//                 }

//                 println!("Service : {}", entry.unwrap().service);
//                 println!("Username : {}", entry.unwrap().username.clone().unwrap_or("".to_string()));
//                 println!("Username : {}", String::from_utf8_lossy(&entry.unwrap().password));
                

//                 print!("Username (optional) : ");
//                 stdout().flush().expect("error");

//                 print!("Password : ");
//                 stdout().flush().expect("error");
//             }
//             4 => exit(0),
//             _ => {
//                 println!("Invalid choice, please try again.");
//             }
//         }
//     }
// }

mod app;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}