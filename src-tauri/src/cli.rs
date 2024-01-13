use crate::db::retrieve_wallet;
use crate::wallet::create_wallet;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::SyncOptions;
use rpassword::prompt_password;

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
            if let Some(arg) = matches.args.get("create_wallet") {
                if arg.occurrences >= 1 {
                    let name: &String = &arg.value.to_string().replace("\"", "");

                    let password = prompt_password("Set password: ").unwrap();

                    //TODO: password load
                    let (wallet, mnemonic_words) = create_wallet(name, &password);

                    println!("Created new wallet: {}", mnemonic_words);

                    let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
                    let blockchain = ElectrumBlockchain::from(client);
                    wallet.sync(&blockchain, SyncOptions::default()).unwrap();

                    println!("Balance: {}", wallet.get_balance().unwrap().get_total());

                    return Some(0);
                }
            }

            if let Some(ref subcommand) = matches.subcommand {
                if subcommand.name == "wallet" {
                    let wallet_name: String = subcommand
                        .matches .args
                        .get("name")
                        .unwrap()
                        .value
                        .to_string()
                        .replace("\"", "");

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

// Entry arguments for running GUI app
pub fn handle_gui_cli(app: &mut tauri::App) {
    if let Ok(_matches) = app.get_cli_matches() {
        println!("Arguments for GUI app on how to run");
    }
}
