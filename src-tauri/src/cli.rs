use crate::db;
use crate::wallet::create_wallet;
use crate::wallet::WalletInterface;
use bdk::KeychainKind;
use rpassword::prompt_password;
use tauri::api::cli::Matches;

pub fn handle_onetime_cli(
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
            if contains(&matches, "list") {
                println!("Printing wallet names: ");
                for name in db::get_wallet_names().unwrap().iter() {
                    println!("- {}", name);
                }
                return Some(0);
            }

            if let Some(name) = get_value(&matches, "new_wallet") {
                let password = prompt_password("Set password: ").unwrap();

                 match create_wallet(name, password) {
                    Ok((_wallet, mnemonic_words)) => {
                        println!("Created new wallet: {}", mnemonic_words);
                        return Some(0);
                    } 
                    Err(err) => {
                        println!("Error: {}", err);
                        return Some(101);
                    }
                };
            }

            if let Some(_name) = get_value(&matches, "recover_wallet") {
                //TODO:
                todo!("recover");
            }   

            if let Some(ref subcommand) = matches.subcommand {
                if subcommand.name == "wallet" {
                    let wallet_name: String = get_value(&subcommand.matches, "name").unwrap();
                    let password = match prompt_password("Enter password: ") {
                        Ok(pwd) => {
                            println!("-> correct");
                            pwd
                        }
                        Err(_) => {
                            eprintln!("-> error while entering the password");
                            return Some(101);
                        }
                    };
                    let wallet = match db::retrieve_wallet(&wallet_name, &password) {
                        Ok(w) => w,
                        Err(err) => {
                            eprintln!("Error while retrieving wallet: {}", err);
                            return Some(102);
                        }
                    };

                    let ms = &subcommand.matches;

                    if contains(ms, "sync") {
                        println!("Synchronizing blockchain...");
                        wallet.synchronize();
                        println!("-> finished");
                    }
                    if contains(ms, "balance") {
                        let balance = wallet.get_total_balance().unwrap();
                        println!("Balance: {} sats", balance)
                    }
                    if contains(ms, "address") {
                        let address = wallet.last_used_address().unwrap();
                        println!("New address: {}", address.to_string());
                    }
                    if contains(ms, "transactions") {
                        let txs = wallet.all_txs().unwrap();
                        println!("Listing transactions: ");
                        for tx in txs.iter() {
                            dbg!(tx);
                        }
                    }

                    if let Some(ref subcommand) = ms.subcommand {
                        if subcommand.name == "send" {
                            let ms2 = &subcommand.matches;
                            let send_to = get_value(ms2, "recipient").unwrap();
                            let amount = get_value(ms2, "amount").unwrap().parse::<u64>().unwrap();

                            println!("{} {}", send_to, amount);
                            println!("signer count {}", wallet.get_signers(KeychainKind::External).signers().len());
                            wallet.send(send_to, amount);
                        }
                    }

                    if contains(ms, "delete") {
                        match db::delete_wallet(wallet_name) {
                            Ok(true) => println!("Wallet was deleted"),
                            Ok(false) => println!("Wallet deletion failed"),
                            Err(err) => println!("Error when deleting wallet: {}", err.to_string()),
                        };
                    }

                    return Some(0);
                }
            }
        }
    }
    return None;
}

// Entry arguments for running GUI app
pub fn handle_gui_cli(app: &mut tauri::App) {
    if let Ok(_matches) = app.get_cli_matches() {
        println!("Arguments for GUI app on how to run");
    }
}

fn contains(matches: &Matches, arg_name: &str) -> bool {
    if let Some(arg) = matches.args.get(arg_name) {
        arg.occurrences >= 1
    } else {
        false
    }
}

fn get_value(matches: &Matches, arg_name: &str) -> Option<String> {
    if let Some(arg) = matches.args.get(arg_name) {
        if arg.occurrences >= 1 {
            Some(arg.value.to_string().replace("\"", ""))
        } else {
            None
        }
    } else {
        None
    }
}
