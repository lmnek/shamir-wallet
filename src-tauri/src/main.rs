// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod wallet;

use bdk::SyncOptions;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use db::retrieve_wallet;
use std::env::set_var;
use std::process::ExitCode;
use tauri;
use crate::db::save_wallet;
use crate::wallet::create_wallet;
use rpassword::prompt_password;

fn main() -> ExitCode {
    set_var("RUST_BACKTRACE", "1");

    let context = tauri::generate_context!();
    if let Some(exit_code) = handle_onetime_cli(&context) {
        return ExitCode::from(exit_code);
    };

    tauri::Builder::default()
        .setup(|app| {
            handle_cli(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cw])
        .run(context)
        .expect("error while running tauri application");
    ExitCode::from(0)
}

fn handle_onetime_cli(
    context: &tauri::Context<tauri::utils::assets::EmbeddedAssets>,
) -> Option<u8> {
    let matches = tauri::api::cli::get_matches(
        &context.config().tauri.cli.clone().unwrap(),
        context.package_info(),
    );

    // CLI for one-time actions
    match matches {
        Err(err) => {
            println!("CLI error: {}", err);
            return Some(101);
        }
        Ok(matches) => {
            if let Some(arg) = matches.args.get("create_wallet") {
                if arg.occurrences >= 1 {
                    let name: &String = &arg
                        .value
                        .to_string()
                        .chars()
                        .filter(|c| *c != '"')
                        .collect();

                    let  password = prompt_password("Set password: ").unwrap();

                    //TODO: password load
                    let (wallet, mnemonic_words) = create_wallet(name, &password);

                    save_wallet(name, &password, &wallet).unwrap();

                    println!("Created new wallet: {}", mnemonic_words);

                    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
                    let blockchain = ElectrumBlockchain::from(client);
                    wallet.sync(&blockchain, SyncOptions::default()).unwrap();

                    println!("Balance: {}", wallet.get_balance().unwrap().get_total());

                    return Some(0);
                }
            }

            // NOTE: only for testing so far
            if let Some(arg) = matches.args.get("verbose") {
                if arg.occurrences >= 1 {
                    let _ = retrieve_wallet(&"wallet1".to_owned(), &"123".to_owned()).unwrap();
                    return Some(0);
                }
            }

            if let Some(ref subcommand) = matches.subcommand {
                if subcommand.name == "wallet" {
                    let wallet_name: String = subcommand.matches.args
                        .get("name").unwrap().value
                        .to_string()
                        .chars()
                        .filter(|c| *c != '"')
                        .collect();
                    let password = prompt_password("Enter password: ").unwrap();
                    let wallet = retrieve_wallet(&wallet_name, &password).unwrap();

                    if let Some(arg) = subcommand.matches.args.get("create_address") {
                        if arg.occurrences >= 1 {
                            let address = wallet.get_address(bdk::wallet::AddressIndex::New);
                            dbg!(address);
                            return Some(0);
                        }
                    }
                    if let Some(arg) = subcommand.matches.args.get("list_addresses") {
                        if arg.occurrences >= 1 {
                            //wallet.database().get_raw_tx
                        }
                    }
                }
            }
        }
    }
    return None;
}

fn handle_cli(app: &mut tauri::App) {
    if let Ok(_matches) = app.get_cli_matches() {
        println!("Arguments for GUI app on how to run");
    }
}

#[tauri::command]
fn cw(name: String, password: String) -> String {
    let password_hash = "todo";
    //let (_wallet, mnemonic_words) = create_wallet(&name, &password_hash);
    return "dfa".to_string(); //mnemonic_words
}
