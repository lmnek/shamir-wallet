use crate::db;
use crate::shamir;
use crate::wallet::create_wallet;
use crate::wallet::recover_wallet;
use crate::wallet::recover_wallet_shamir;
use crate::wallet::xprv_from_mnemonic;
use crate::wallet::WalletInterface;
use anyhow::anyhow;
use rpassword::prompt_password;
use tauri::api::cli::Matches;
use tauri::utils::assets::EmbeddedAssets;

pub fn handle_onetime_cli(
    context: &tauri::Context<EmbeddedAssets>,
) -> Option<u8> {
    match handle_cli(context) {
        Ok(false) => None,
        Ok(true) => Some(0),
        Err(err) => {
            eprintln!("Error: {}", err);
            Some(100)
        }
    }
}

pub fn handle_cli(
    context: &tauri::Context<EmbeddedAssets>,
) -> anyhow::Result<bool> {
    let matches = tauri::api::cli::get_matches(
        &context.config().tauri.cli.clone().unwrap(),
        context.package_info(),
    );

    // CLI for one-time actions
    match matches {
        Err(err) => {
            return Err(anyhow!("CLI - {}", err));
        }
        Ok(matches) => {
            if contains(&matches, "list") {
                let wallet_names = db::get_wallet_names()?; 
                println!("Printing wallet names: ");
                for name in wallet_names.iter() {
                    println!("- {}", name);
                }
                return Ok(true)
            }

            //dbg!(&matches);

            if let Some(ref subcommand) = matches.subcommand {
                let ms = &subcommand.matches;
                match subcommand.name.as_str() {
                    "new" => {
                        let wallet_name: String = get_value(ms, "name").unwrap();
                        let password = prompt_password("enter password: ")?;

                        let (_wallet, mnemonic_words) = create_wallet(wallet_name.clone(), password.clone())?;

                        if let Some(ref subcommand) = ms.subcommand {
                            if subcommand.name == "shamir" {
                                let ms2 = &subcommand.matches;
                                let treshold = get_value(ms2, "treshold")
                                    .unwrap()
                                    .parse::<u8>()?;
                                let count = get_value(ms2, "count")
                                    .unwrap()
                                    .parse::<u8>()?;
                                let (_wallet, mnemonic) =
                                    create_wallet(wallet_name.clone(), password)?;
                                let xprv = xprv_from_mnemonic(&mnemonic)?;
                                let shamir_mnemonic_shares = shamir::split(xprv, treshold, count)?;
                                for share in shamir_mnemonic_shares {
                                    println!("SHARE: ");
                                    println!("{}", share);
                                }
                            }
                        } else {
                            println!("Created new wallet: {}", mnemonic_words);
                        }
                    }
                    "recovery" => {
                        let wallet_name: String = get_value(ms, "name").unwrap();
                        let password = prompt_password("enter password: ")?;

                        if let Some(ref subcommand) = ms.subcommand {
                            if subcommand.name == "shamir" {
                                let ms2 = &subcommand.matches;
                                if let Some(_arg) = ms2.args.get("share") {
                                    // FIX: collect shares from arguments and contruct wallet
                                    // let shares: Value::Array = arg.value.into();

                                    recover_wallet_shamir(
                                        wallet_name.clone(),
                                        password,
                                        vec![],
                                    )?;
                                }
                            }
                        } else {
                            let mnemonic: String = get_value(ms, "mnemonic").unwrap();
                            recover_wallet(wallet_name.clone(), password, mnemonic)?;
                        }
                        println!("Created new wallet");
                        println!("Warning: Clear your command line history!")
                    }
                    "wallet" => {
                        let wallet_name: String = get_value(&subcommand.matches, "name").unwrap();
                        let password = prompt_password("enter password: ")?;
                        let wallet = db::retrieve_wallet(&wallet_name, &password)?;

                        if contains(ms, "sync") {
                            println!("Synchronizing blockchain...");
                            wallet.synchronize()?; 
                        }
                        if contains(ms, "balance") {
                            let balance = wallet.get_total_balance()?;
                            println!("balance: {} sats", balance);
                        }
                        if contains(ms, "address") {
                            let address = wallet.last_used_address()?; 
                            println!("New address: {}", address.to_string());
                        }
                        if contains(ms, "transactions") {
                            let txs = wallet.all_txs()?;
                            println!("Listing transactions: ");
                            for tx in txs.iter() {
                                dbg!(tx);
                            }
                        }

                        if let Some(ref subcommand) = ms.subcommand {
                            if subcommand.name == "send" {
                                let ms2 = &subcommand.matches;
                                // Tauri.conf makes sure that args are present -> "required"
                                let send_to = get_value(ms2, "recipient").unwrap();
                                let amount = get_value(ms2, "amount").unwrap().parse::<u64>()?;
                                let fee_rate = get_value(ms2, "fees").unwrap().parse::<f32>()?;

                                wallet.send(send_to, amount, fee_rate)?;
                                println!("-> Successfuly broadcasted transaction");
                            }
                        }

                        if contains(ms, "delete") {
                            match db::delete_wallet(wallet_name)? {
                                true => println!("Wallet was deleted"),
                                false => println!("Wallet deletion failed"),
                            };
                        }
                    }
                    _ => {
                        return Err(anyhow!("Unknown subcommand"))
                    }
                }
                return Ok(true)
            }
        }
    }
    return Ok(false);
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
