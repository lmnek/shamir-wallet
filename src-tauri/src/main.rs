// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::exit;
use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::keys::{
    bip39::{Language, Mnemonic, WordCount},
    DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
};
use bdk::template::Bip84;
use bdk::{miniscript, KeychainKind, Wallet};
use tauri;

fn main() {
    let context = tauri::generate_context!();
    handle_onetime_cli(&context);

//    let (_wallet, _mnemonic_words) = create_wallet();

    tauri::Builder::default()
        .setup(|app| { handle_cli(app); Ok(()) })
        .invoke_handler(tauri::generate_handler![cw])
        .run(context)
        .expect("error while running tauri application");
}

fn handle_onetime_cli(context: &tauri::Context<tauri::utils::assets::EmbeddedAssets>) {
    let matches = tauri::api::cli::get_matches( 
        &context.config().tauri.cli.clone().unwrap(), 
        context.package_info());

    // CLI for one-time actions
    match matches {
        Err(err) => {
            println!("CLI error: {}", err);
            exit(-1)
        }
        Ok(matches) => {
            if matches.args.iter()
                .find(|(name, _)| { name.to_string() == "create_wallet" })
                .map_or(false, |(_, arg)| arg.occurrences >= 1) {

                println!("Created new wallet: kfjdlsakjflkdsajlkfj");
                exit(0)
            }
            if let Some(_subcommand) = matches.subcommand { 
                println!("TODO: handle onetime subcommands")
            }
        }
    }
}

fn handle_cli(app: &mut tauri::App) {
    if let Ok(_matches) = app.get_cli_matches() {
        println!("Arguments for GUI app on how to run");
    }
}

#[tauri::command]
fn cw() -> String {
    let (_wallet, mnemonic_words) = create_wallet();
    return mnemonic_words
}

// can be offline
fn create_wallet() -> (Wallet<MemoryDatabase>, String) {
    let network = Network::Testnet;

    // Generate fresh mnemonic
    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    // Convert mnemonic to string
    let mnemonic_words = mnemonic.to_string();
    // Parse a mnemonic
    let mnemonic = Mnemonic::parse(&mnemonic_words).unwrap();
    // Generate the extended key
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    // Get xprv from the extended key
    let xprv = xkey.into_xprv(network).unwrap();

    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let wallet = Wallet::new(
        Bip84(xprv, KeychainKind::External),
        Some(Bip84(xprv, KeychainKind::Internal)),
        network,
        MemoryDatabase::default(),
    )
    .unwrap();

    println!(
        "mnemonic: {}\n\nrecv desc (pub key): {:#?}\n\nchng desc (pub key): {:#?}",
        mnemonic_words,
        wallet
            .get_descriptor_for_keychain(KeychainKind::External)
            .to_string(),
        wallet
            .get_descriptor_for_keychain(KeychainKind::Internal)
            .to_string()
    );

    (wallet, mnemonic_words)
}

fn _shamir_into_shares(_mnemonic: String) {}
