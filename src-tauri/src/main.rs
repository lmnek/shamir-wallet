// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod db;
mod wallet;
mod shamir;

use cli::{handle_gui_cli, handle_onetime_cli};
use serde::Serialize;
use std::env::set_var;
use std::process::ExitCode;
use std::sync::Mutex;
use tauri::{self, Manager, State};
use wallet::{create_wallet, recover_wallet, recover_wallet_shamir, xprv_from_mnemonic, MyWallet, WalletInterface};

fn main() -> ExitCode {
    set_var("RUST_BACKTRACE", "1"); //NOTE: remove for production

    let context = tauri::generate_context!();
    if let Some(exit_code) = handle_onetime_cli(&context) {
        return ExitCode::from(exit_code);
    };

    tauri::Builder::default()
        .setup(|app| {
            handle_gui_cli(app);
            // let window = app.get_window("main").unwrap();
            // window.open_devtools();
            Ok(())
        })
        .manage::<StateContent>(Mutex::new(vec![]))
        .invoke_handler(tauri::generate_handler![
            load_wallet,
            cw,
            cw_shamir,
            rw,
            rw_shamir,
            get_wallet_names,
            get_wallet_data,
            close_wallet,
            delete_wallet
        ])
        .run(context)
        .expect("error while running tauri application");
    ExitCode::from(0)
}

type StateContent = Mutex<Vec<(String, Mutex<MyWallet>)>>;

// Create wallet and return mnemonic
#[tauri::command]
fn cw(name: String, password: String, state: State<StateContent>) -> Result<String, String> {
    match create_wallet(name.clone(), password) {
        Ok((wallet, mnemonic_words)) => {
            add_wallet_state(name, wallet, state);
            Ok(mnemonic_words)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn cw_shamir(name: String, password: String, treshold: u8, count: u8, state: State<StateContent>) -> Result<Vec<String>, String> {
    match create_wallet(name.clone(), password) {
        Ok((wallet, mnemonic)) => {
            let xprv = xprv_from_mnemonic(&mnemonic);
            let shamir_mnemonic_shares = shamir::split(xprv, treshold, count).unwrap();

            add_wallet_state(name, wallet, state);
            Ok(shamir_mnemonic_shares)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn load_wallet(name: String, password: String, state: State<StateContent>) -> Result<(), String> {
    match db::retrieve_wallet(&name, &password) {
        Ok(wallet) => {
            add_wallet_state(name, wallet, state);
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

// Retrive wallet from mnemonic
#[tauri::command]
fn rw(name: String, password: String, mnemonic: String, state: State<StateContent>) -> Result<(), String> {
    match recover_wallet(name.clone(), password, mnemonic) {
        Ok(wallet) => {
            add_wallet_state(name, wallet, state);
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn rw_shamir(name: String, password: String, mnemonics: Vec<Vec<String>>, state: State<StateContent>) -> Result<(), String> {
    match recover_wallet_shamir(name.clone(), password, mnemonics) {
        Ok(wallet) => {
            add_wallet_state(name, wallet, state);
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Serialize)]
struct TransactionData {
    sender_address: String,
    sent: u64,
    received: u64,
    timestamp: u64,
}

#[derive(Serialize)]
struct WalletData {
    name: String,
    balance: u64,
    address: String,
    transactions: Vec<TransactionData>,
}

#[tauri::command]
fn get_wallet_data(name: String, state: State<StateContent>) -> Result<WalletData, String> {
    with_wallet(&name, state, |wallet| {
        wallet.synchronize();
        Ok(WalletData {
            name: name.clone(),
            balance: wallet.get_total_balance().unwrap(),
            address: wallet.last_used_address().unwrap().to_string(),
            transactions: wallet
                .all_txs()
                .unwrap()
                .iter()
                .map(|tx| TransactionData {
                    sender_address: "TODO:".to_string(),
                    sent: tx.sent,
                    received: tx.received,
                    timestamp: match &tx.confirmation_time {
                        Some(ct) => ct.timestamp,
                        None => 0,
                    }
                })
                .collect(),
        })
    })
}

#[tauri::command]
fn get_wallet_names() -> Result<Vec<String>, String> {
    db::get_wallet_names().map_err(|err| err.to_string())
}

#[tauri::command]
fn close_wallet(name: String, state: State<StateContent>) {
    let mut wallets = state.lock().unwrap();
    let index_to_remove = wallets
        .iter()
        .position(|(w_name, _)| *w_name == name)
        .unwrap();
    wallets.remove(index_to_remove);
}

#[tauri::command]
fn delete_wallet(name: String, state: State<StateContent>) {
    db::delete_wallet(name.clone()).unwrap();
    close_wallet(name, state);
}

fn add_wallet_state(name: String, wallet: MyWallet, state: State<StateContent>) {
    let mut wallets = state.lock().unwrap();
    wallets.push((name, Mutex::new(wallet)));
}

fn with_wallet<F, R>(name: &String, state: State<StateContent>, callback: F) -> R
where
    F: FnOnce(&MyWallet) -> R,
{
    let wallets = state.lock().unwrap();
    let wallet = wallets
        .iter()
        .find(|(n, _)| n.eq(name))
        .unwrap()
        .1
        .lock()
        .unwrap();
    callback(&wallet)
}
